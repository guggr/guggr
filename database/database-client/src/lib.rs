pub mod insertable;
pub mod models;
pub mod schema;

use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use thiserror::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Could not connect to the database: {0}")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("Failed to run migrations: {0}")]
    MigrationError(String),
}

pub fn establish_connection(database_url: &str) -> Result<PgConnection, DbError> {
    let mut conn = PgConnection::establish(database_url)?;

    // Automatically run migrations on every connection attempt
    run_migrations(&mut conn)?;

    Ok(conn)
}

fn run_migrations(connection: &mut impl MigrationHarness<diesel::pg::Pg>) -> Result<(), DbError> {
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| DbError::MigrationError(e.to_string()))?;

    Ok(())
}
