#!/bin/sh

# As the routing to the api-service is handled via ingress / httpRoutes in
# k8s, the frontend container does not need to route to the api-service.
# When nginx starts, it tries to automatically resolve upstreams that are
# set in the `proxy_pass` directive. Resolving `api-service` in the k8s
# deployment isn't always possible, so the upstream is moved to a
# variable.
# To avoid that, the upstream is assigned to a variable first. In that
# form, nginx resolves the hostname only when a request actually uses that
# upstream.

# Once `proxy_pass` uses a variable, nginx requires an explicit resolver
# directive. This cannot point to 127.0.0.1 unless a DNS server is
# actually running on localhost inside the container. Hardcoding Docker’s
# usual resolver address 127.0.0.11 is also not ideal, because container
# DNS addresses differ between Docker and Podman.
set -eu

NGINX_RESOLVER="$(awk '/^nameserver / {print $2; exit}' /etc/resolv.conf)"

if [ -z "${NGINX_RESOLVER:-}" ]; then
  echo "No nameserver found in /etc/resolv.conf" >&2
  exit 1
fi

export NGINX_RESOLVER
envsubst '${NGINX_RESOLVER}' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf

exec nginx -g 'daemon off;'
