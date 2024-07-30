#!/usr/bin/env python
import pika
from functools import cache

# Establish a connection with RabbitMQ server
connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))

channel = connection.channel()

# Declare a queue named 'rpc_queue'
channel.queue_declare(queue='rpc_queue')

# Use caching to store results of Fibonacci calculations to improve performance
@cache
def fib(n):
    if n == 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fib(n - 1) + fib(n - 2)

# Define the callback function to process RPC requests
def on_request(ch, method, props, body):
    n = int(body)

    print(f" [.] fib({n})")
    response = fib(n)

    # Send the response back to the client using the 'reply_to' and 'correlation_id' properties
    ch.basic_publish(exchange='',
        routing_key=props.reply_to,
        properties=pika.BasicProperties(correlation_id =\
            props.correlation_id),
        body=str(response))
    
    # Send an acknowledgment to RabbitMQ that the message has been processed
    ch.basic_ack(delivery_tag=method.delivery_tag)

# Set QoS to process one message at a time
channel.basic_qos(prefetch_count=1)
# Set up the consumer to call the 'on_request' function when a message is received
channel.basic_consume(queue='rpc_queue', on_message_callback=on_request)

print(" [x] Awaiting RPC requests")
# Start consuming messages
channel.start_consuming()