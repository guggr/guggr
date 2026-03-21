#!/bin/sh
set -eu

NGINX_RESOLVER="$(awk '/^nameserver / {print $2; exit}' /etc/resolv.conf)"

if [ -z "${NGINX_RESOLVER:-}" ]; then
  echo "No nameserver found in /etc/resolv.conf" >&2
  exit 1
fi

export NGINX_RESOLVER
envsubst '${NGINX_RESOLVER}' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf

exec nginx -g 'daemon off;'
