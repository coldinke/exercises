import sys, pika

messages = ' '.join(sys.argv[1:]) or "Hello World\n"

# connect the rabbitMQ
connection = pika.BlockingConnection(
    pika.ConnectionParameters('localhost')
)
# get the channel from the previous connection
channel = connection.channel()

channel.queue_declare(queue='task_queue', durable=True)

channel.basic_publish(exchange='', 
    routing_key='task_queue', body=messages,
    properties=pika.BasicProperties(
        delivery_mode = pika.DeliveryMode.Persistent
    ))
print(f" [x] Sent {messages}")

# Close the connection
connection.close()