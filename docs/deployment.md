# Deployment Guide

## Quick Start

Clone this repository and run `docker compose up -d`. This will build all containers and start them:

```sh
git clone git@github.com:guggr/guggr.git && cd guggr
docker compose up -d
```

Guggr will then be available under http://localhost:8080

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
