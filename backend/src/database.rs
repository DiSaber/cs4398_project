use std::str::FromStr;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};

#[derive(Debug, Clone)]
pub struct Database {
    pub db_pool: SqlitePool,
}

impl Database {
    pub async fn connect() -> Result<Self, sqlx::Error> {
        let opts = SqliteConnectOptions::from_str("sqlite://data.db")
            .expect("Url should be valid")
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);
        let db_pool = SqlitePool::connect_with(opts).await?;

        Ok(Self { db_pool })
    }

    /// Creates the db tables when they don't exist.
    pub async fn create_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS lobbies (
                id INTEGER PRIMARY KEY,
                join_code BLOB NOT NULL UNIQUE CHECK (length(join_code) = 3),
                created_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL CHECK (expires_at >= created_at)
            ) STRICT
            "#,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
