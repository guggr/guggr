@help:
    just --list api-service

run:
    env $(cat .env | xargs) cargo run --bin api-service
