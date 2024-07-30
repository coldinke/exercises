#!/usr/bin/env python
import pika
import sys

# Establish a connection with RabbitMQ server
connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

# Declare an exchange named 'topic_logs'
# 'exchange_type='topic'' means the exchange will route messages to queues based on matching routing key patterns
channel.exchange_declare(exchange='topic_logs', exchange_type='topic')

# Declare a nameless queue
# 'exclusive=True' ensures the queue is deleted when the connection is closed
result = channel.queue_declare('', exclusive=True)
queue_name = result.method.queue

# Get the binding keys from command line arguments
binding_keys = sys.argv[1:]
if not binding_keys:
    sys.stderr.write("Usage: %s [binding_key]...\n" % sys.argv[0])
    sys.exit(1)

# Bind the queue to the 'topic_logs' exchange for each binding key
for binding_key in binding_keys:
    channel.queue_bind(
        exchange='topic_logs', queue=queue_name, routing_key=binding_key
    )

print(' [*] Waiting for logs, To exit press CTRL+C')

# Define the callback function to process messages from the queue
def callback(ch, method, properties, body):
    print(f" [x] {method.routing_key}:{body}")

# Set up the consumer to call the 'callback' function when a message is received
# 'auto_ack=True' means messages are automatically acknowledged
channel.basic_consume(
    queue=queue_name, on_message_callback=callback, auto_ack=True
)

# Start consuming messages
channel.start_consuming()