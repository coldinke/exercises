#!/usr/bin/env python
import pika

# Establish a connection with Rabbit server
connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

# Declare a exchange named 'logs'
# see the emit_log.py's commits
channel.exchange_declare(exchange='logs', exchange_type='fanout')

# Declare a nameless queue
# 'exclusive=True' ensures the queue is deleted when the connection is closed
result = channel.queue_declare(queue='', exclusive=True)
queue_name = result.method.queue

# Bind the nameless queue to the 'logs' exchange
channel.queue_bind(exchange='logs', queue=queue_name)

print(' [*] Waiting for logs. To exit press CTRL+C')

# Define the callback function to prcess messages from the queue
def callback(ch, method, propertites, body):
    print(f" [x] {body}")

# Set up the consumer to call the 'callback' function when a message is received
# 'auto_ack=True' means messages are automatically acknowledged
channel.basic_consume(
    queue=queue_name, on_message_callback=callback, auto_ack=True)

# Start consuming messages
channel.start_consuming()