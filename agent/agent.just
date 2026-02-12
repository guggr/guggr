@help:
    just --list agent

run:
    env $(cat .env.example | xargs) cargo run --bin agent
