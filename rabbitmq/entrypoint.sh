#!/bin/sh

# starting server and sending it to background
rabbitmq-server &

until rabbitmqctl await_startup; do
  sleep 2
done

rabbitmqctl import_definitions /etc/rabbitmq/definitions.json

# needed for keeping service running
wait
