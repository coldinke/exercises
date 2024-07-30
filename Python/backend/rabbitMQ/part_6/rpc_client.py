#!/usr/bin/env python
import pika
import uuid
import sys

class FibonacciRpcClient(object):

    def __init__(self):
        # Establish a connection with RabbitMQ server
        self.connection = pika.BlockingConnection(
            pika.ConnectionParameters(host='localhost'))

        self.channel = self.connection.channel()

        # Declare a nameless callback queue
        # 'exclusive=True' ensures the queue is deleted when the connection is closed
        result = self.channel.queue_declare(queue='', exclusive=True)
        self.callback_queue = result.method.queue

        # Set up the consumer to call the 'on_response' function when a message is received
        self.channel.basic_consume(
            queue=self.callback_queue,
            on_message_callback=self.on_response,
            auto_ack=True
        )

        self.response = None
        self.corr_id = None

    def on_response(self, ch, method, props, body):
        # Check if the correlation ID matches and store response
        if self.corr_id == props.correlation_id:
            self.response = body

    def call(self, n):
        self.response = None
        self.corr_id = str(uuid.uuid4())
        # Publish the request message to the 'rpc_queue' with the correlation ID and reply-to properties
        self.channel.basic_publish(
            exchange='',
            routing_key='rpc_queue',
            properties=pika.BasicProperties(
                reply_to=self.callback_queue,
                correlation_id=self.corr_id,
            ),
            body=str(n)
        )
        # Wait for the response
        while self.response is None:
            self.connection.process_data_events(time_limit=None)
        return int(self.response)

# Create an instance of the FibonacciRpcClient
fibonacci_rpc = FibonacciRpcClient()

# Get the input number from command line arguments or use 30 ad the default
n = sys.argv[1] if len(sys.argv) > 1 else 30

print(f" [x] Requesting fib({n})")
# Call the Fibonacci function via RPC and print the response
response = fibonacci_rpc.call(n)
print(f" [.] Got {response}")