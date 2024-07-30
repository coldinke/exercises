#!/usr/bin/ent python
import pika
import sys

# Establish a connection with RabbitMQ server
connection = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost'))
channel = connection.channel()

# Declare an exchange named 'topic_logs'
# 'exchange_type='topic'' means the exchange will route messages to queues based on matching routing key patterns
channel.exchange_declare(exchange='topic_logs', exchange_type='topic')

# Get the routing key from command line arguments or use 'anonymous.info' as the default routing key
routing_key = sys.argv[1] if len(sys.argv) > 2 else 'anonymous.info'
# Get the messages form command line arguments or use 'Hello World!' as the default message
message = ' '.join(sys.argv[2:]) or 'Hello World!'

# Publish the message to the 'topic_logs' exchange with the specified routing key
channel.basic_publish(
    exchange='topic_logs', routing_key=routing_key, body=message)

# Print a confirmation message to the console 
print(f' [x] Sent {routing_key}:{message}')

# Close the connection to the RabbitMQ server
connection.close()