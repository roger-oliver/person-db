#!/bin/bash
# Wait for RabbitMQ to be ready
until rabbitmqctl await_startup; do
  sleep 1
done

# Create the queue
rabbitmqadmin declare queue name=people-queue durable=true