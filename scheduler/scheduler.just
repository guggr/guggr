@help:
    just --list scheduler

# Loads variables from scheduler/.env and runs the scheduler
run:
    env $(cat .env | xargs) cargo run --bin scheduler
