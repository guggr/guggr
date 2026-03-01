#! /bin/sh

# Fail on errors
set -e

# Generate the reference file
cargo run --bin gen-openapi "api-service/openapi-check.json"
# Format the reference file (for comparison)
pnpm prettier -w api-service/openapi-check.json
# Diff files, exits with 1 on difference between files
diff api-service/openapi.json api-service/openapi-check.json
# Delete reference (otherwise pre-commit hook fails due to modified files)
rm api-service/openapi-check.json
