//! Database configuration with smart defaults.

use serde::Deserialize;
use std::time::Duration;

/// Database configuration with sensible defaults.
///
/// Can be deserialized from TOML/JSON or constructed directly.
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL (e.g., "postgres://user:pass@localhost:5432/dbname")
    pub url: String,

    /// Maximum number of connections in the pool.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Minimum number of connections in the pool.
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// Connection timeout in seconds.
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,

    /// Timeout for acquiring a connection from the pool (in seconds).
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout_secs: u64,

    /// Idle timeout for connections in the pool (in seconds).
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_secs: u64,

    /// Maximum lifetime of a connection in the pool (in seconds).
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime_secs: u64,

    /// Whether to enable SQLx logging.
    #[serde(default = "default_sql_logging")]
    pub sql_logging: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://localhost:5432/eywa".to_string(),
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            connect_timeout_secs: default_connect_timeout(),
            acquire_timeout_secs: default_acquire_timeout(),
            idle_timeout_secs: default_idle_timeout(),
            max_lifetime_secs: default_max_lifetime(),
            sql_logging: default_sql_logging(),
        }
    }
}

// Default values
fn default_max_connections() -> u32 {
    100
}

fn default_min_connections() -> u32 {
    5
}

fn default_connect_timeout() -> u64 {
    8
}

fn default_acquire_timeout() -> u64 {
    8
}

fn default_idle_timeout() -> u64 {
    8
}

fn default_max_lifetime() -> u64 {
    8
}

fn default_sql_logging() -> bool {
    true
}

impl DatabaseConfig {
    /// Create a new DatabaseConfig with just the URL.
    ///
    /// All other values use smart defaults.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    /// Get the connect timeout as a Duration.
    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_secs)
    }

    /// Get the acquire timeout as a Duration.
    pub fn acquire_timeout(&self) -> Duration {
        Duration::from_secs(self.acquire_timeout_secs)
    }

    /// Get the idle timeout as a Duration.
    pub fn idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_timeout_secs)
    }

    /// Get the max lifetime as a Duration.
    pub fn max_lifetime(&self) -> Duration {
        Duration::from_secs(self.max_lifetime_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.connect_timeout_secs, 8);
        assert!(config.sql_logging);
    }

    #[test]
    fn test_config_from_url() {
        let config = DatabaseConfig::new("postgres://localhost/test");
        assert_eq!(config.url, "postgres://localhost/test");
        assert_eq!(config.max_connections, 100); // default
    }
}
