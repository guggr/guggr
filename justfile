@help:
	just --list

# Contains database-specific recipes
mod db 'database/db.just'
# Contains evaluator-specific recipes
mod evaluator 'evaluator/evaluator.just'
# Contains scheduler-specific recipes
mod scheduler 'scheduler/scheduler.just'
# Contains api-service-specific recipes
mod api-service 'api-service/api-service.just'

mod agent 'agent/agent.just'

alias c := check
alias f := fmt
alias l := lint
alias t := test
alias r := rust
alias clippy := lint-rust
alias nextest := test-rust

# Format the source code
[group('bundle')]
[parallel]
check: fmt lint test

# Format the source code
[group('bundle')]
[parallel]
fmt: fmt-rust fmt-pnpm

# Lint the source code
[group('bundle')]
[parallel]
lint: lint-rust

# Test the source code
[group('bundle')]
[parallel]
test: test-rust

# Run all rust recipes
[group('bundle')]
rust: fmt-rust lint-rust test-rust autoinherit machete
	@just api-service gen-spec
	pnpm prettier -w api-service/openapi.json

# Run prettier
[group('pnpm')]
[group('format')]
fmt-pnpm:
	pnpm prettier . -w --log-level warn

# Run rustfmt
[group('rust')]
[group('format')]
fmt-rust:
	rustfmt-nightly

# Run clippy
[group('rust')]
[group('lint')]
lint-rust:
	cargo clippy --all-features --all-targets -- -D warnings

# Run nextest
[group('rust')]
[group('test')]
test-rust profile="default":
	cargo nextest run --no-tests warn {{ if profile != "default" { "-P " + profile } else { "" } }}

# Run machete
[group('rust')]
machete:
	cargo machete

# Run autoinherit
[group('rust')]
autoinherit:
	cargo autoinherit

# Run clippy with pedantic settings
[group('rust')]
bashme:
	cargo clippy --all-features --all-targets -- -W clippy::nursery -W clippy::pedantic

# Compile the docs and open them
[group('rust')]
docs:
	cargo doc --document-private-items --open

# Asks for confirmation
_confirmation:
	#!/usr/bin/env bash
	read -p "Are you sure? [y/N] " -n 1 -r
	echo ""
	if [[ ! $REPLY =~ ^[Yy]$ ]]; then
		echo "Aborting."
		exit 1
	fi
