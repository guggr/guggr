@help:
    just --list evaluator

run:
    env $(cat .env.example | xargs) cargo run
