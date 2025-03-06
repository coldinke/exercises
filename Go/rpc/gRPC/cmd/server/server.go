package main

import (
	pb "chat/proto"
	"fmt"
	"log"
	"net"
	"sync"
	"time"

	"google.golang.org/grpc"
)

type chatServer struct {
	pb.UnimplementedChatServiceServer
	mu      sync.Mutex
	clients map[string][]pb.ChatService_ChatStreamServer
}

func newServer() *chatServer {
	return &chatServer{
		clients: make(map[string][]pb.ChatService_ChatStreamServer),
	}
}

func (s *chatServer) broadcastMessage(msg *pb.Message) {
	log.Printf("Broadcasting message from %s: %s", msg.UserId, msg.Content)

	s.mu.Lock()
	defer s.mu.Unlock()

	for _, clients := range s.clients {
		for _, client := range clients {
			if err := client.Send(msg); err != nil {
				log.Printf("Failed to send message to client: %v", err)
			}
		}
	}
}

func (s *chatServer) ChatStream(stream pb.ChatService_ChatStreamServer) error {
	// 接收第一条消息来获取用户ID
	msg, err := stream.Recv()
	if err != nil {
		return err
	}
	userID := msg.UserId
	log.Printf("New user connected: %s", userID)

	// 注册客户端流
	s.mu.Lock()
	s.clients[userID] = append(s.clients[userID], stream)
	clientCount := len(s.clients)
	s.mu.Unlock()
	log.Printf("Total connected clients: %d", clientCount)

	msg.Timestamp = time.Now().Unix()
	log.Printf("Broadcasting join message for user %s", userID)
	s.broadcastMessage(msg)

	// 广播接收到的消息
	for {
		msg, err := stream.Recv()
		if err != nil {
			log.Printf("Client %s disconnected: %v", userID, err)

			s.mu.Lock()
			// 移除断开连接的客户端
			clients := s.clients[userID]
			for i, client := range clients {
				if client == stream {
					s.clients[userID] = append(clients[:i], clients[i+1:]...)
					break
				}
			}
			s.mu.Unlock()
			leaveMsg := &pb.Message{
				UserId:    userID,
				Content:   fmt.Sprintf("%v leave the chat room!", userID),
				Timestamp: time.Now().Unix(),
			}
			s.broadcastMessage(leaveMsg)
			return err
		}
		log.Printf("Received message from %s: %s", userID, msg.Content)

		msg.Timestamp = time.Now().Unix()
		s.broadcastMessage(msg)
	}
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterChatServiceServer(grpcServer, newServer())

	log.Println("Starting gRPC server on port 50051...")
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
