#!/usr/bin/env python
import pika
import sys

# Establish a connection with RabbitMQ server
connection = pika.BlockingConnection(
    pika.ConnectionParameters('localhost')
)
channel = connection.channel()

# Declare a queue named 'task_queue'
# 'durable=True' ensures that the queue will survive a RabbitMQ server restart
channel.queue_declare(queue='task_queue', durable=True)

# Get the message form command line arguments or use "Hello World\n" as the default message
messages = ' '.join(sys.argv[1:]) or "Hello World\n"

# Publish the message to teh 'task_queue
# 'exchange' is set to any empty string, which means the default exchange
# 'routing_key' is the name of the queue where the message will be delivered
# 'body' is the content of the message being sent
# 'properties' with 'deliver_mode=2 makes the mssage persistent
channel.basic_publish(exchange='', 
    routing_key='task_queue', body=messages,
    properties=pika.BasicProperties(
        delivery_mode = pika.DeliveryMode.Persistent
    ))

# Print a confirmation message to the console
print(f" [x] Sent {messages}")

# Close the connection to the Rabbit server
connection.close()