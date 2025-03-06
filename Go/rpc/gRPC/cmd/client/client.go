package main

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"os"

	pb "chat/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewChatServiceClient(conn)
	stream, err := client.ChatStream(context.Background())
	if err != nil {
		log.Fatalf("Failed to create stream: %v", err)
	}

	fmt.Print("Enter your user ID: ")
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	userID := scanner.Text()

	log.Printf("Connected to chat server as %s", userID)

	joinMsg := &pb.Message{
		UserId:  userID,
		Content: fmt.Sprintf("%v joined the chat", userID),
	}

	if err := stream.Send(joinMsg); err != nil {
		log.Fatalf("Failed to send initial message: %v", err)
	}
	log.Printf("Sent join message: %s", joinMsg.Content)

	go func() {
		for {
			msg, err := stream.Recv()
			if err != nil {
				log.Printf("Error receiving message: %v", err)
				return
			}
			if msg.UserId == userID {
				continue
			} else {
				fmt.Printf("%s: %s\n", msg.UserId, msg.Content)
			}
		}
	}()

	for scanner.Scan() {
		msg := &pb.Message{
			UserId:  userID,
			Content: scanner.Text(),
		}
		if err := stream.Send(msg); err != nil {
			log.Printf("Failed to send message: %v", err)
			break
		}
	}
}
