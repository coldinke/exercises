#!/usr/bin/env python
import pika

# Part I: send.py

# Establish a connection with RabbitMQ server
# 'localhost' indicates the RabbitMQ server is running on the local machine 
connection = pika.BlockingConnection(
    pika.ConnectionParameters('localhost'))

# Create a new channel with the connection
channel = connection.channel()

# Declare a queue named 'hello'
# If the queue doesn't exist, it will be created
channel.queue_declare(queue='hello')

# Publish a message to the queue 'hello'
# 'exchange' is set to an empty string, which means the default exchange
# 'routing_key' is the name of the queue where the message will be delivered
# 'body' is the content of the message being sent 
channel.basic_publish(exchange='',
                      routing_key='hello',
                      body='Hello World!')

# Print a confiramtion message to the console
print(" [x] Sent 'Hello World'")

# Close the connection to the RabbitMQ server
connection.close()

