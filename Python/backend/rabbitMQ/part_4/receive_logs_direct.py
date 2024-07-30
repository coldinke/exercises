#!/usr/bin/env python
import pika
import sys

# Establish a connection with RabbitMQ server
connction = pika.BlockingConnection(
    pika.ConnectionParameters('localhost'))
channel = connction.channel()

# Declare an exchange named 'direct_logs'
# 'exchange_type='direct'' means the exchange will route message to the queues based on the routing key
channel.exchange_declare(exchange='direct_logs', exchange_type='direct')

# Declare a nameless queue
# 'exclusive=True' ensures the queue is deleted with the connection is closed 
result = channel.queue_declare(queue='', exclusive=True)
queue_name = result.method.queue

# Get the severities from command line arguments
severities = sys.argv[1:]
if not severities:
    sys.stderr.write("Usage: %s [info] [warning] [error]\n" % sys.argv[0])
    sys.exit(1)

# Bind the queue to the 'direct_logs' exchange for each severity
for severity in severities:
    channel.queue_bind(
        exchange='direct_logs', queue=queue_name, routing_key=severity)

print(' [*] Waiting for logs. To exit press CTRL+C')

# Define the callback function to process messages from the queue
def callback(ch, method, properties, body):
    print(f" [x] {method.routing_key}:{body}")

# Set up the consumer to call the 'callback' function when a message is received
# 'auto_ack=True' means messages are automatically acknowledged
channel.basic_consume(
    queue=queue_name, on_message_callback=callback, auto_ack=True)

# Start consuming messages
channel.start_consuming()