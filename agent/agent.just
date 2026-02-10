@help:
    just --list agent

run:
    env $(cat .env | xargs) cargo run --bin agent
