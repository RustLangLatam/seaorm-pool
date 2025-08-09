//! # Application Configuration Module
//!
//! This module defines the configuration structures required for the application,
//! focusing on database connectivity through `sea-orm`. It is designed to be
//! flexible, serializable, and easily integrated into various application setups.
//!
//! The primary entry point is the `AppConfig` struct, which encapsulates all
//! other configuration settings, including the `DatabaseConfig` for `sea-orm`
//! connections.
//!
//! All structures derive `serde::Serialize` and `serde::Deserialize` to allow for
//! effortless parsing from configuration files (e.g., TOML, YAML, JSON).
//!
//! ## Module Components
//!
//! - **`AppConfig`**: The root configuration struct for the application.
//! - **`DatabaseConfig`**: Holds all settings related to the database connection,
//!   including credentials, address, and pooling.
//! - **`PoolOptions`**: Specifies the behavior of the database connection pool,
//!   such as connection limits and timeouts.
//!
//! ## Example Usage (TOML File)
//!
//! ```toml
//! # Database connection settings are nested under the 'database' key.
//! [database]
//! host = "localhost"
//! port = 5432
//! username = "user"
//! password = "password"
//! databaseName = "app_db"
//!
//! # Optional: SSL settings for a secure connection.
//! # sslCa = "/path/to/ca.pem"
//!
//! # Connection pool settings.
//! [database.poolOptions]
//! maxConnections = 20
//! minConnections = 5
//! acquireTimeout = "30s"
//! idleTimeout = "10m"
//! maxLifetime = "30m"
//! ```

use std::time::Duration;

/// Represents the main configuration for the application.
///
/// This struct serves as the top-level container, holding all configuration
/// necessary for the application to run. It includes database-specific settings
/// encapsulated within the `database` field.
///
/// # Examples
///
/// An example of how this might be represented in a TOML configuration file:
///
/// ```toml
/// [database]
/// host = "127.0.0.1"
/// port = 4000
/// username = "admin"
/// password = "secret"
/// databaseName = "mydb"
///
/// [database.poolOptions]
/// maxConnections = 10
/// ```
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Configuration for the database connection, using `sea-orm`.
    pub database: DatabaseConfig,
}

/// Defines the configuration for connecting to a database using `sea-orm`.
///
/// This struct contains all necessary parameters to establish and manage
/// database connections. It includes server details, authentication credentials,
/// and nested configuration for connection pooling and SSL.
///
/// # Examples
///
/// An example of how this might be represented in a TOML configuration file:
///
/// ```toml
/// host = "db.example.com"
/// port = 5432
/// username = "db_user"
/// password = "super_secret"
/// databaseName = "production_db"
///
/// [poolOptions]
/// maxConnections = 15
/// minConnections = 2
/// ```
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    /// The hostname or IP address of the database server.
    pub host: String,

    /// The port number of the database server.
    ///
    /// If not specified, this will default to the standard port used by the
    /// `sea-orm` driver (e.g., 5432 for PostgreSQL, 3306 for MySQL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// The username for authenticating to the database server.
    pub username: String,

    /// The password for authenticating to the database server.
    pub password: String,

    /// The name of the specific database to connect to.
    pub database_name: String,

    /// Connection pooling options to manage database connections efficiently.
    ///
    /// If this section is omitted from a configuration file, default pool settings
    /// will be applied.
    #[serde(default)]
    pub pool_options: PoolOptions,

    /// The file path to the SSL Certificate Authority (CA) for establishing a
    /// secure, encrypted connection.
    ///
    /// If this is `None`, SSL/TLS will not be explicitly configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca: Option<String>,
}

impl DatabaseConfig {
    /// Returns the full network address of the database server as a single string.
    ///
    /// If a port is specified, it formats the output as `"host:port"`.
    /// Otherwise, it returns the host alone.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming this module is in a file named `config.rs`
    /// // use crate::config::{DatabaseConfig, PoolOptions};
    /// # use super_project::DatabaseConfig; // This line is for documentation test purposes
    ///
    /// // A config with a specified port
    /// let mut config_with_port = DatabaseConfig::default();
    /// config_with_port.host = "127.0.0.1".to_string();
    /// config_with_port.port = Some(5432);
    /// assert_eq!(config_with_port.get_address(), "127.0.0.1:5432");
    ///
    /// // A config without a specified port
    /// let mut config_without_port = DatabaseConfig::default();
    /// config_without_port.host = "db.example.com".to_string();
    /// assert_eq!(config_without_port.get_address(), "db.example.com");
    /// ```
    pub fn get_address(&self) -> String {
        if let Some(port) = self.port {
            format!("{}:{}", self.host, port)
        } else {
            self.host.clone()
        }
    }
}

/// Provides a default, non-functional `DatabaseConfig` for convenience.
///
/// The default values are empty or zero-like, and must be overridden with
/// actual configuration before use.
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: None,
            username: String::new(),
            password: String::new(),
            database_name: String::new(),
            pool_options: PoolOptions::default(),
            ssl_ca: None,
        }
    }
}

// Private helper functions to provide default values for `PoolOptions`.
// This is a standard pattern when using `#[serde(default = "...")]`.
fn default_max_connections() -> u32 {
    10
}
fn default_min_connections() -> u32 {
    1
}
fn default_acquire_timeout() -> Duration {
    Duration::from_secs(30)
}
fn default_idle_timeout() -> Duration {
    Duration::from_secs(300)
}
fn default_max_lifetime() -> Duration {
    Duration::from_secs(1800)
}
fn default_is_lazy() -> bool {
    true
}
fn default_statement_cache_capacity() -> usize {
    100
}

/// Configures the behavior of the database connection pool.
///
/// These settings control the lifecycle and allocation of database connections,
/// which is critical for application performance and stability. The options
/// map directly to the underlying `sqlx::PoolOptions`.
///
/// # Examples
///
/// An example of how this might be represented in a TOML configuration file:
///
/// ```toml
/// maxConnections = 20
/// minConnections = 5
/// acquireTimeout = "30s"
/// idleTimeout = "5m"
/// maxLifetime = "30m"
/// isLazy = false
/// statementCacheCapacity = 200
/// ```
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PoolOptions {
    /// The maximum number of connections the pool should maintain.
    ///
    /// This value should be set carefully based on the database server's capacity
    /// and the expected application load.
    ///
    /// **Default**: `10`
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// The minimum number of idle connections the pool should maintain.
    ///
    /// When the pool is created, this many connections will be established. If the number
    /// of idle connections drops below this value, the pool will attempt to create new
    /// ones to replenish it.
    ///
    /// **Default**: `1`
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// The maximum time to wait for a connection to become available.
    ///
    /// If a connection cannot be acquired from the pool within this duration, the
    /// operation will time out and return an error.
    ///
    /// **Default**: `30` seconds
    #[serde(with = "humantime_serde")]
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout: Duration,

    /// The maximum duration a connection can remain idle in the pool.
    ///
    /// Any connection that has been idle for longer than this duration will be
    /// closed and removed from the pool. This helps release resources on the
    /// database server.
    ///
    /// **Default**: `300` seconds (5 minutes)
    #[serde(with = "humantime_serde")]
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: Duration,

    /// The maximum lifetime of any single connection.
    ///
    /// A connection will be gracefully closed once it has been open for this
    /// duration. This is a crucial practice for preventing resource leaks and ensuring
    /// connections are periodically recycled.
    ///
    /// **Default**: `1800` seconds (30 minutes)
    #[serde(with = "humantime_serde")]
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime: Duration,

    /// If `true`, the pool will not establish connections until one is first requested.
    ///
    /// If `false`, the pool will try to establish `min_connections` immediately
    /// upon creation.
    ///
    /// **Default**: `true`
    #[serde(default = "default_is_lazy")]
    pub is_lazy: bool,

    /// The capacity of the prepared statement cache for each connection.
    ///
    /// Caching is managed using an LRU (Least Recently Used) policy. When the number
    /// of cached statements exceeds this capacity, the oldest one is evicted.
    ///
    /// **Default**: `100`
    #[serde(default = "default_statement_cache_capacity")]
    pub statement_cache_capacity: usize,
}

impl Default for PoolOptions {
    /// Creates a `PoolOptions` instance with sensible default values.
    fn default() -> Self {
        Self {
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            acquire_timeout: default_acquire_timeout(),
            idle_timeout: default_idle_timeout(),
            max_lifetime: default_max_lifetime(),
            is_lazy: default_is_lazy(),
            statement_cache_capacity: default_statement_cache_capacity(),
        }
    }
}

// The tests module
#[cfg(test)]
mod tests {
    use super::*;
    // Import everything from the outer module.

    /// Test 1: Full deserialization of an AppConfig from a TOML string.
    /// Verifies that all fields, including nested ones, are correctly parsed.
    #[test]
    fn test_full_app_config_deserialization() {
        let toml_str = r#"
            [database]
            host = "prod.db.internal"
            port = 5433
            username = "prod_user"
            password = "prod_password"
            databaseName = "prod_db"
            sslCa = "/etc/ssl/certs/ca-certificates.crt"

            [database.poolOptions]
            maxConnections = 50
            minConnections = 10
            acquireTimeout = "60s"
            idleTimeout = "10m"
            maxLifetime = "1h"
            isLazy = false
            statementCacheCapacity = 250
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to parse TOML");

        // Assert database config
        assert_eq!(config.database.host, "prod.db.internal");
        assert_eq!(config.database.port, Some(5433));
        assert_eq!(config.database.username, "prod_user");
        assert_eq!(config.database.password, "prod_password");
        assert_eq!(config.database.database_name, "prod_db");
        assert_eq!(
            config.database.ssl_ca,
            Some("/etc/ssl/certs/ca-certificates.crt".to_string())
        );

        // Assert pool options
        assert_eq!(config.database.pool_options.max_connections, 50);
        assert_eq!(config.database.pool_options.min_connections, 10);
        assert_eq!(
            config.database.pool_options.acquire_timeout,
            Duration::from_secs(60)
        );
        assert_eq!(
            config.database.pool_options.idle_timeout,
            Duration::from_secs(600)
        );
        assert_eq!(
            config.database.pool_options.max_lifetime,
            Duration::from_secs(3600)
        );
        assert_eq!(config.database.pool_options.is_lazy, false);
        assert_eq!(config.database.pool_options.statement_cache_capacity, 250);
    }

    /// Test 2: Deserialization with minimal configuration.
    /// Verifies that default values are applied correctly for omitted fields.
    #[test]
    fn test_minimal_config_uses_defaults() {
        let toml_str = r#"
            [database]
            host = "test.db"
            username = "test_user"
            password = "test_password"
            databaseName = "test_db"
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to parse TOML");

        // Assert provided values
        assert_eq!(config.database.host, "test.db");
        assert_eq!(config.database.database_name, "test_db");

        // Assert default values
        assert_eq!(config.database.port, None);
        assert_eq!(config.database.ssl_ca, None);
        assert_eq!(config.database.pool_options, PoolOptions::default());
    }

    /// Test 3: Partial pool options deserialization. (CORRECTED)
    /// Verifies that specified pool options override defaults, while others remain default.
    #[test]
    fn test_partial_pool_options_deserialization() {
        // This TOML must represent a valid AppConfig to ensure parsing succeeds.
        // We provide the necessary database fields and a partial poolOptions table.
        let toml_str = r#"
            [database]
            host = "dummy"
            username = "dummy"
            password = "dummy"
            databaseName = "dummy"

            # Only specify a subset of pool options
            [database.poolOptions]
            maxConnections = 5
            acquireTimeout = "5s"
        "#;

        // Parse the full AppConfig. This should now succeed.
        let config: AppConfig =
            toml::from_str(toml_str).expect("Failed to parse TOML with partial pool options");

        let pool_opts = &config.database.pool_options;

        // Assert that the specified values have been overridden
        assert_eq!(pool_opts.max_connections, 5); // Overridden, this will now pass
        assert_eq!(pool_opts.acquire_timeout, Duration::from_secs(5)); // Overridden

        // Assert that the unspecified values correctly fall back to their defaults
        assert_eq!(pool_opts.min_connections, default_min_connections()); // Default
        assert_eq!(pool_opts.idle_timeout, default_idle_timeout()); // Default
        assert_eq!(pool_opts.max_lifetime, default_max_lifetime()); // Default
    }

    /// Test 4: `PoolOptions::default()` constructor.
    /// Verifies that the default constructor for PoolOptions provides the correct values.
    #[test]
    fn test_pool_options_default_values() {
        let defaults = PoolOptions::default();
        assert_eq!(defaults.max_connections, 10);
        assert_eq!(defaults.min_connections, 1);
        assert_eq!(defaults.acquire_timeout, Duration::from_secs(30));
        assert_eq!(defaults.idle_timeout, Duration::from_secs(300));
        assert_eq!(defaults.max_lifetime, Duration::from_secs(1800));
        assert_eq!(defaults.is_lazy, true);
        assert_eq!(defaults.statement_cache_capacity, 100);
    }

    /// Test 5: `DatabaseConfig::default()` constructor.
    /// Verifies the default values for the main database configuration struct.
    #[test]
    fn test_database_config_default_values() {
        let defaults = DatabaseConfig::default();
        assert_eq!(defaults.host, "localhost");
        assert_eq!(defaults.port, None);
        assert!(defaults.username.is_empty());
        assert!(defaults.password.is_empty());
        assert!(defaults.database_name.is_empty());
        assert_eq!(defaults.ssl_ca, None);
        assert_eq!(defaults.pool_options, PoolOptions::default());
    }

    /// Test 6: `get_address()` method with a port specified.
    #[test]
    fn test_get_address_with_port() {
        let mut config = DatabaseConfig::default();
        config.host = "127.0.0.1".to_string();
        config.port = Some(5432);
        assert_eq!(config.get_address(), "127.0.0.1:5432");
    }

    /// Test 7: `get_address()` method without a port specified.
    #[test]
    fn test_get_address_without_port() {
        let mut config = DatabaseConfig::default();
        config.host = "database.service.local".to_string();
        config.port = None;
        assert_eq!(config.get_address(), "database.service.local");
    }

    /// Test 8: Serialization and deserialization roundtrip.
    /// Ensures that a configuration can be serialized and then deserialized back
    /// into an identical structure. Uses JSON for simplicity.
    #[test]
    fn test_json_serialization_roundtrip() {
        let original_config = AppConfig {
            database: DatabaseConfig {
                host: "roundtrip.db".to_string(),
                port: Some(1234),
                username: "rt_user".to_string(),
                password: "rt_password".to_string(),
                database_name: "rt_db".to_string(),
                ssl_ca: Some("/tmp/ca.pem".to_string()),
                pool_options: PoolOptions {
                    max_connections: 99,
                    ..Default::default()
                },
            },
        };

        let json_string = serde_json::to_string(&original_config).expect("Serialization failed");
        let deserialized_config: AppConfig =
            serde_json::from_str(&json_string).expect("Deserialization failed");

        assert_eq!(original_config, deserialized_config);
    }

    /// Test 9: `skip_serializing_if` attribute behavior.
    /// Verifies that optional fields with `None` values are omitted from the
    /// serialized output.
    #[test]
    fn test_skip_serializing_if_none() {
        let config = DatabaseConfig {
            port: None,
            ssl_ca: None,
            ..DatabaseConfig::default()
        };

        let toml_string = toml::to_string(&config).expect("Failed to serialize to TOML");

        // Check that the keys for `None` values are not present in the output
        assert!(!toml_string.contains("port ="));
        assert!(!toml_string.contains("sslCa ="));
    }

    /// Test 10: `camelCase` naming convention is correctly handled.
    /// This is implicitly tested in other tests, but this one focuses on it.
    #[test]
    fn test_camel_case_field_names() {
        let toml_str = r#"
            [database]
            host = "dummy"
            username = "dummy"
            password = "dummy"
            databaseName = "camel_case_db"

            [database.poolOptions]
            maxConnections = 123
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to parse");

        assert_eq!(config.database.database_name, "camel_case_db");
        assert_eq!(config.database.pool_options.max_connections, 123);
    }

    /// Test 11: Deserializing human-readable durations.
    /// Verifies that `humantime_serde` correctly parses string representations of time.
    #[test]
    fn test_humantime_duration_parsing() {
        let toml_str = r#"
            acquireTimeout = "1m"
            idleTimeout = "2h"
            maxLifetime = "3d"
        "#;
        let pool_opts: PoolOptions = toml::from_str(toml_str).expect("Failed to parse humantime");

        assert_eq!(pool_opts.acquire_timeout, Duration::from_secs(60));
        assert_eq!(pool_opts.idle_timeout, Duration::from_secs(2 * 3600));
        assert_eq!(pool_opts.max_lifetime, Duration::from_secs(3 * 24 * 3600));
    }
}
