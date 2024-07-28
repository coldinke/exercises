#!/usr/bin/env python
import pika
import sys

# Establish a connection with RabbitMQ server
connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

# Declare a exchange named 'logs'
# 'exchange_type='fanout'' means the exchange will broadcast all the messages it receives to all the queues it knows
channel.exchange_declare(exchange='logs', exchange_type='fanout')

# Get the message from command line arguments or use "info: Hello World!" as the default message
message = ' '.join(sys.argv[1:]) or "info: Hello World!"

# Publish the message to the 'logs' exchange
# 'routing_key' is empty because 'fanout' exchanges do not use routing keys
channel.basic_publish(exchange='logs', routing_key='', body=message)

# Print a confirmation message to the console
print(f" [x] Sent {message}")

# Close the connection to the RabbitMQ server
connection.close()