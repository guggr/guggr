# Deployment Guide

## Quick Start

Get a few configuration files and the compose file and then start the stack:

```sh
wget -O compose.yaml https://raw.githubusercontent.com/guggr/guggr/refs/heads/main/compose.prebuilt.yaml
# Nginx config that proxies between backend and frontend
wget -O nginx.conf https://raw.githubusercontent.com/guggr/guggr/refs/heads/main/nginx.conf
# RabbitMQ entrypoint that loads the definitions
wget -O rabbitmq_entrypoint.sh https://raw.githubusercontent.com/guggr/guggr/refs/heads/main/rabbitmq/entrypoint.sh
# RabbitMQ definitions
wget -O rabbitmq_definitions.json  https://raw.githubusercontent.com/guggr/guggr/refs/heads/main/charts/guggr/files/rabbitmq_definitions.json
# Example environment file. Replace the secrets before starting!
wget -O .env https://raw.githubusercontent.com/guggr/guggr/refs/heads/main/.env.example
chmod +x rabbitmq_entrypoint.sh

docker compose up -d
```

guggr will then be available under http://localhost:8080

> [!WARNING]
> This setup is intended for evaluation and local testing only. For production use, a Kubernetes deployment is recommended.

## Production Deployment

Deploying guggr for production use can be achieved by first adding our helm repository and then installing the helm chart.

```sh
helm repo add guggr https://guggr.github.io/guggr
helm install guggr guggr/guggr-chart
```

You can customize the deployment by overwriting helm values with

```sh
helm install guggr guggr/guggr-chart -f yourvalues.yaml
```

Example configurations can be seen [here](../charts/guggr/README.md#example-configurations)
