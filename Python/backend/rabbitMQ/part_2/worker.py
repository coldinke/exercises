#!/usr/bin/env python
import pika, sys, os, time


def main():
    # Establish a connection with RabbitMQ server
    connection = pika.BlockingConnection(
        pika.ConnectionParameters('localhost'))
    channel = connection.channel()

    # Declare a queue named 'task_queue'
    # 'durable=True' ensures that the queue will survivie a RabbitMQ server restart
    channel.queue_declare(queue='task_queue', durable=True)
    
    # Define the callback function to process message form the queue
    def callback(ch, method, properties, body):
        print(f" [x] Received {body.decode()}")
        # Simulate work
        time.sleep(body.count(b'.'))
        print(f" [x] Done")
        # Send an acknowledgment to RabbitMQ that the message has been processed
        ch.basic_ack(delivery_tag = method.delivery_tag)

    # Option to enable automatic message acknowledgment
    # channel.basic_consume(queue='hello', on_message_callback=callback, auto_ack=True)

    # Ensure that RabbitMQ only delivers one message to the worker at a time 
    channel.basic_qos(prefetch_count=1)
    # Set up the consumer to call the 'callback' function when a message is received 
    channel.basic_consume(queue='task_queue', on_message_callback=callback)

    # Print a message indicating the consumer is waiting for messages
    print(" [*] Waiting for message. to exit press CTRL+C")
    # Start consuming message.
    channel.start_consuming()

if __name__ == '__main__':
    try:
        # Run the main function
        main()
    except KeyboardInterrupt:
        # Handle the interrupt signal
        print("Interrupted")
        try:
            sys.exit(0)
        except SystemExit:
            os._exit(0)