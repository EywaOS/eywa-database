//! Database connection pool management.

use super::config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database as SeaDatabase, DatabaseConnection};
use tracing::info;

use crate::Result;

/// Wrapper for database connections.
///
/// Provides a simple interface to connect to a PostgreSQL database
/// with Sea-ORM using smart defaults.
pub struct Database;

impl Database {
    /// Connect to the database using the provided URL.
    ///
    /// Uses smart defaults for all pool settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use eywa_database::Database;
    ///
    /// # async fn example() -> eywa_database::Result<()> {
    /// let db = Database::connect("postgres://user:pass@localhost:5432/dbname").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(db_url: &str) -> Result<DatabaseConnection> {
        Self::connect_with_config(&DatabaseConfig::new(db_url)).await
    }

    /// Connect to the database using the provided configuration.
    ///
    /// This method allows full control over connection pool settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use eywa_database::{Database, DatabaseConfig};
    ///
    /// # async fn example() -> eywa_database::Result<()> {
    /// let config = DatabaseConfig::new("postgres://user:pass@localhost:5432/dbname");
    /// let db = Database::connect_with_config(&config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_with_config(config: &DatabaseConfig) -> Result<DatabaseConnection> {
        info!("Connecting to database...");

        let mut opt = ConnectOptions::new(config.url.clone());
        opt.max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect_timeout(config.connect_timeout())
            .acquire_timeout(config.acquire_timeout())
            .idle_timeout(config.idle_timeout())
            .max_lifetime(config.max_lifetime())
            .sqlx_logging(config.sql_logging);

        SeaDatabase::connect(opt)
            .await
            .map_err(|e| eywa_errors::AppError::DatabaseError(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_connect_requires_url() {
        // This test verifies that the API exists
        // Actual connection tests would require a running database
        let config = DatabaseConfig::new("postgres://localhost:5432/test");
        assert_eq!(config.url, "postgres://localhost:5432/test");
    }
}
