# Database

## Folder Structure

- `database-client/`: Contains code for Rust database client using [Diesel](https://diesel.rs) -> Import this to use the database in your Rust code
- `design/`: Contains database schema in DBML (for database editors) and a picture of the current schema
- `migrations/`: Contains database migrations, managed via diesel CLI
- `scripts/`: Contains setup scripts for PostgreSQL and test data for initial population of the database
- `.env.example`: Contains database URL, required for local development with diesel
- `db.just`: Contains useful scripts to manage the database and models as a just submodule, invoke using `just db $RECIPE`

## Migrations

1. Generate a new migration by running

   ```sh
   just db gen-migration $MIGRATION_NAME
   ```

2. Edit the generated files `up.sql` and `down.sql`
3. Check your new migration with

   ```sh
   just db list-migrations
   ```

4. Apply your migration

   ```sh
   just db migrate
   ```

   > [!IMPORTANT]
   > If this command fails with the following error:
   >
   > ```log
   > thread 'main' (122899) panicked at src/main.rs:143:19:
   > Unrecognized option: 'insertable'
   > ```
   >
   > You are using an outdated `diesel_ext` version. For `nixpkgs`, a PR exists to update `diesel_ext` to the latest version ([nixpkgs#487982](https://github.com/NixOS/nixpkgs/pull/487982)). Until it is merged, an overlay has been added to the Nix flake to use the updated version from the PR.
