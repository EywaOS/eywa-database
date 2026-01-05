//! # EYWA Database
//!
//! Shared database connection and utilities for EYWA services.
//!
//! ## Features
//!
//! - Connection pool management with smart defaults
//! - Configurable pool settings via `DatabaseConfig`
//! - Transaction helpers for safe database operations
//! - Seamless integration with Sea-ORM
//!
//! ## Quick Start
//!
//! ```no_run
//! use eywa_database::{Database, DatabaseConfig};
//!
//! # async fn example() -> eywa_database::Result<()> {
//! // Simple connection with defaults
//! let db = Database::connect("postgres://user:pass@localhost:5432/dbname").await?;
//!
//! // Or with custom configuration
//! let config = DatabaseConfig::new("postgres://user:pass@localhost:5432/dbname");
//! let db = Database::connect_with_config(&config).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Using Transactions
//!
//! ```no_run
//! use eywa_database::transaction;
//! use sea_orm::DatabaseConnection;
//!
//! # async fn example(db: &DatabaseConnection) -> eywa_database::Result<()> {
//! transaction::with_transaction(db, |txn| async move {
//!     // Your transactional logic here
//!     Ok(())
//! }).await?;
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod pool;
pub mod transaction;

// Re-export commonly used types
pub use config::DatabaseConfig;
pub use eywa_errors::{AppError, Result};
pub use pool::Database;
pub use sea_orm;

// Re-export Sea-ORM types for convenience
pub use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    ColumnTrait, QueryFilter, QuerySelect, IntoActiveModel, ActiveModelTrait, TransactionTrait,
};

/// Prelude module with common imports
pub mod prelude {
    pub use crate::{Database, DatabaseConfig};
    pub use eywa_errors::{AppError, Result};
    pub use sea_orm::*;
}
