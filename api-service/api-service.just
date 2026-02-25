@help:
    just --list api-service

run:
    env $(cat .env | xargs) cargo run --bin api-service

# Generate OpenApi specification file
gen-spec path="openapi.json":
    cargo run --bin gen-openapi {{path}}
