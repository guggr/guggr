@help:
    just --list evaluator

run:
    env $(cat .env | xargs) cargo run --bin evaluator
