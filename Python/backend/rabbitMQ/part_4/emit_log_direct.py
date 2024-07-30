#!/usr/bin/env python
import pika
import sys

# Establish a connection with RabbitMQ server
connction = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connction.channel()

# Declare an exchange named 'direct_logs'
# 'exchange_type='direct'' means the exchange will route messages to the queue based on the routing key
channel.exchange_declare(exchange='direct_logs', exchange_type='direct')

# Get the severity from command line arguments or use 'info' as the default severtiy
severity = sys.argv[1] if len(sys.argv) > 1 else 'info'
# Get the message from command line arguments or use 'Hello World!' as the default message
message = ' '.join(sys.argv[2:]) or 'Hello World!'

# Publish the message to the 'direct_logs' exchange with the specified severity as the routing key
channel.basic_publish(
    exchange='direct_logs', routing_key=severity, body=message)

# Print a confirmation message to hte console
print(f" [x] Sent {severity}:{message}")

# Close the connection to the RabbitMQ server
connction.close()