@help:
    just --list db

models_file := "database-client/src/models.rs"
insertable_file := "database-client/src/insertable.rs"
schema_file := "database-client/src/schema.rs"

# Generates new migration
[group('migration')]
gen-migration MIGRATION_NAME:
    diesel migration generate {{ MIGRATION_NAME }}

# List all migrations, also shows pending
[group('migration')]
list-migrations:
    diesel migration list

# Runs migration
[group('migration')]
_run-migrations:
    diesel migration run
    diesel print-schema > {{ schema_file }}

# Perform database migrations and regenerate models
[group('migration')]
migrate: && _run-migrations models
    @just _confirmation

# Revert migration
[group('migration')]
revert-migration:
    @just _confirmation
    diesel migration revert

# Generate models and adapt them for usage
models:
    diesel_ext --model \
        --import-types "diesel::prelude::*" \
        --import-types "crate::schema::*" \
        --import-types "diesel::data_types::PgInterval" \
        --add-table-name \
        --schema-file {{ schema_file }} \
        --map "Interval PgInterval" \
        > {{ models_file }}

    sed -i 's/Identifiable/Identifiable, Insertable/' {{ models_file }}
    @just fmt-rust
