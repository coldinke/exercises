#!/usr/bin/env python
import pika, sys, os

def main():
    # Establish a connection with RabbitMQ server
    connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
    # Create a new channel with connection
    channel = connection.channel()

    # Declare a queue named 'hello'
    # If the queue doesn't exist, it will be created
    channel.queue_declare(queue='hello')

    # Define the callback function to process messages from the queue
    def callback(ch, method, properties, body):
        print(f" [x] Received {body}")

    # Set up the consumer to call the 'callback' function when a message is received
    # 'auto_ack=True' means messages are automatically acknowledged
    channel.basic_consume(queue='hello', on_message_callback=callback, auto_ack=True)

    # Print a message indicating the consumer is waiting for message
    print(" [*] Waiting for messages. To exit press CTRL+C")

    # Start consuming messages
    channel.start_consuming()


if __name__ == '__main__':
    try:
        # Run the main function
        main()
    except KeyboardInterrupt:
        # Handle the interrupt signal (e.g., CTRL+C)
        print("Interrupted")
        try:
            sys.exit(0)
        except SystemExit:
            os._exit(0)