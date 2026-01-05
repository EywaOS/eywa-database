# eywa-database

Shared database connection and utilities for EYWA services.

## Overview

`eywa-database` provides a unified interface for connecting to PostgreSQL with Sea-ORM across all EYWA services. It handles connection pooling with smart defaults and offers transaction helpers for safe database operations.

## Features

- **Connection Pool Management** - Automatic connection pooling with sensible defaults
- **Configurable Settings** - Fine-tune pool behavior via `DatabaseConfig`
- **Transaction Helpers** - Safe transaction handling with automatic commit/rollback
- **Sea-ORM Integration** - Seamless integration with Sea-ORM entities and queries
- **Structured Configuration** - Deserialize from TOML/JSON or construct programmatically

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
eywa-database = { path = "../eywa-database" }
```

Or use via `eywa-axum` (recommended):

```toml
[dependencies]
eywa-axum = { path = "../eywa-axum" }
```

## Quick Start

### Basic Connection

```rust
use eywa_axum::Database;

async fn main() -> eywa_axum::Result<()> {
    // Simple connection with smart defaults
    let db = Database::connect("postgres://user:pass@localhost:5432/eywa").await?;

    // Use the connection
    // ...
}
```

### With Custom Configuration

```rust
use eywa_axum::{Database, DatabaseConfig};

async fn main() -> eywa_axum::Result<()> {
    let config = DatabaseConfig {
        url: "postgres://user:pass@localhost:5432/eywa".to_string(),
        max_connections: 50,
        min_connections: 2,
        connect_timeout_secs: 10,
        sql_logging: true,
        ..Default::default()
    };

    let db = Database::connect_with_config(&config).await?;
    Ok(())
}
```

### Loading from Config File

```rust
use eywa_axum::{Database, DatabaseConfig, EywaConfig};

#[derive(Debug, serde::Deserialize)]
struct MyServiceConfig {
    database: DatabaseConfig,
}

async fn main() -> eywa_axum::Result<()> {
    // Load from config/default.toml
    let config: MyServiceConfig = EywaConfig::load()?;
    let db = Database::connect_with_config(&config.database).await?;

    Ok(())
}
```

Example `config/default.toml`:

```toml
[database]
url = "postgres://eywa:secret@localhost:5432/eywa"
max_connections = 100
min_connections = 5
connect_timeout_secs = 8
acquire_timeout_secs = 8
idle_timeout_secs = 8
max_lifetime_secs = 8
sql_logging = true
```

## Configuration Options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | *required* | PostgreSQL connection URL |
| `max_connections` | `u32` | `100` | Maximum connections in the pool |
| `min_connections` | `u32` | `5` | Minimum idle connections |
| `connect_timeout_secs` | `u64` | `8` | Connection timeout in seconds |
| `acquire_timeout_secs` | `u64` | `8` | Timeout to acquire a connection |
| `idle_timeout_secs` | `u64` | `8` | Idle timeout before connection is closed |
| `max_lifetime_secs` | `u64` | `8` | Maximum lifetime of a connection |
| `sql_logging` | `bool` | `true` | Enable SQL query logging |

## Using Transactions

### Basic Transaction

```rust
use eywa_axum::transaction;

async fn transfer_funds(db: &DatabaseConnection, from: Uuid, to: Uuid, amount: i64) -> eywa_axum::Result<()> {
    transaction::with_transaction(db, |txn| async move {
        // Debit from account
        // Credit to account
        // Both operations use `txn` instead of `db`

        Ok(())
    }).await?;

    Ok(())
}
```

### Transaction with Custom Error Type

```rust
use eywa_axum::transaction;

#[derive(Debug)]
enum MyError {
    InsufficientFunds,
    DatabaseError,
}

impl From<eywa_axum::AppError> for MyError {
    fn from(e: eywa_axum::AppError) -> Self {
        MyError::DatabaseError
    }
}

async fn transfer_funds(db: &DatabaseConnection, from: Uuid, to: Uuid, amount: i64) -> Result<(), MyError> {
    transaction::with_transaction_custom_err(db, |txn| async move {
        // Your transactional logic here
        Ok(())
    }).await?;

    Ok(())
}
```

## Usage in Services

### Complete Example with eywa-axum

```rust
use eywa_axum::prelude::*;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
struct AppState {
    db: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config: MyConfig = EywaConfig::load()?;

    // Connect to database
    let db = Database::connect_with_config(&config.database).await?;

    // Run migrations
    migration::Migrator::up(&db, None).await?;

    // Create application state
    let state = AppState { db: Arc::new(db) };

    // Start server
    EywaApp::new(state)
        .info("My Service", "1.0.0", "My EYWA service")
        .mount::<MyController>()
        .serve("0.0.0.0:3000")
        .await
}

#[controller]
impl MyController {
    #[route(get, "/")]
    async fn index(state: State<AppState>) -> Result<Json<Vec<Item>>> {
        let items = Items::find()
            .all(state.db.as_ref())
            .await?;

        Ok(Json(items))
    }
}
```

## Smart Defaults

The `Database::connect()` method uses these defaults automatically:

- **Max connections**: 100
- **Min connections**: 5
- **Timeouts**: 8 seconds (connect, acquire, idle, lifetime)
- **SQL logging**: Enabled

These work well for most services, but can be overridden via `DatabaseConfig` when needed.

## License

MIT
