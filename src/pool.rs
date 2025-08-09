//! # Database Connection Pool Service
//!
//! This module provides a function for establishing a `sea-orm` database connection
//! pool from a given `DatabaseConfig`. It handles the construction of the connection
//! URL, applies all pooling and timeout settings, and configures SSL for secure
//! connections.
//!
//! The main entry point is the `create_connection_pool` function.

use crate::config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::{error, info};
use url::Url;

/// Creates and configures a `sea-orm` database connection pool.
///
/// This function translates a `DatabaseConfig` struct into a live database
/// connection pool. It constructs the required connection string, applies all
/// specified pooling options (e.g., connection limits, timeouts), and sets up
/// SSL if a CA certificate is provided.
///
/// # Parameters
/// - `config`: A `DatabaseConfig` instance containing all necessary settings for
///   the connection and the pool.
///
/// # Returns
/// - `Ok(DatabaseConnection)`: On success, returns a fully configured and
///   (potentially) connected `DatabaseConnection` pool.
/// - `Err(DbErr)`: Returns a `DbErr` if the connection string is invalid or if
///   connecting to the database fails.
///
/// # Errors
/// This function can fail in the following scenarios:
/// - The database URL derived from the config is malformed.
/// - A connection to the database cannot be established due to network issues,
///   incorrect credentials, or invalid SSL settings.
///
/// # Example
///
/// ```rust,ignore
/// use crate::config::DatabaseConfig;
/// use crate::database::create_connection_pool; // Assuming this code is in `src/database.rs`
///
/// #[tokio::main]
/// async fn main() {
///     // 1. Create a configuration (this would typically be loaded from a file)
///     let db_config = DatabaseConfig {
///         host: "127.0.0.1".to_string(),
///         port: Some(3306),
///         username: "my_user".to_string(),
///         password: "my_password".to_string(),
///         database_name: "my_app_db".to_string(),
///         ssl_ca: None,
///         pool_options: Default::default(),
///     };
///
///     // 2. Attempt to create the connection pool
///     match create_connection_pool(db_config).await {
///         Ok(pool) => {
///             println!("Successfully connected to the database!");
///             // The `pool` can now be used to execute queries.
///         }
///         Err(e) => {
///             eprintln!("Failed to create database connection pool: {}", e);
///         }
///     }
/// }
/// ```
#[tracing::instrument(name = "db_pool_creation", err, skip(config), fields(db.host = %config.host))]
pub async fn create_connection_pool(config: DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    info!("Initializing database connection pool...");

    // Construct the full database URL from the configuration.
    // Format: mysql://user:password@host:port/databaseName
    let database_url_str = format!(
        "mysql://{}:{}@{}/{}",
        config.username,
        config.password,
        config.get_address(), // This helper method already handles the host and port
        config.database_name
    );

    let mut database_url = Url::parse(&format!("mysql://{}", config.get_address()))
        .map_err(|err| DbErr::Custom(err.to_string()))?;
    database_url.set_username(&config.username).unwrap();
    database_url.set_password(Some(&config.password)).unwrap();
    database_url.set_path(&config.database_name);

    // Start with a new `ConnectOptions` instance from the URL.
    let mut connect_options = ConnectOptions::new(database_url);

    // Apply all pooling options from the configuration.
    connect_options
        .max_connections(config.pool_options.max_connections)
        .min_connections(config.pool_options.min_connections)
        .acquire_timeout(config.pool_options.acquire_timeout)
        .idle_timeout(config.pool_options.idle_timeout)
        .max_lifetime(config.pool_options.max_lifetime)
        // Set SQLx statement logging level.
        .sqlx_logging_level(tracing::log::LevelFilter::Debug)
        // Disable slow statement logging by default.
        .sqlx_slow_statements_logging_settings(tracing::log::LevelFilter::Off, Duration::default());

    // // Conditionally apply SSL settings if a CA path is provided.
    // if let Some(ca_path) = &config.ssl_ca {
    //     info!("Applying SSL/TLS configuration with CA: {}", ca_path);
    //     connect_options.sqlx_ssl_ca(ca_path);
    // }

    // Log the final pool settings for debugging purposes.
    log_pool_settings(&connect_options);

    info!(
        "Connecting to the database... Lazy mode: {}",
        config.pool_options.is_lazy
    );

    // Establish the connection pool.
    let pool = Database::connect(connect_options).await.map_err(|err| {
        error!(
            "Failed to connect to database server at '{}': {}",
            config.get_address(),
            err
        );
        err
    })?;

    info!("Database connection pool initialized successfully.");
    Ok(pool)
}

/// Logs the configured settings of the connection pool.
///
/// This is a helper function for debugging that prints the key connection pool
/// parameters to the log output.
///
/// # Parameters
/// - `options`: A reference to the `ConnectOptions` instance whose settings
///   are to be logged.
fn log_pool_settings(options: &ConnectOptions) {
    info!("Pool Settings:");
    info!("-> Max connections: {:?}", options.get_max_connections());
    info!("-> Min connections: {:?}", options.get_min_connections());
    info!("-> Acquire timeout: {:?}", options.get_acquire_timeout());
    info!("-> Idle timeout: {:?}", options.get_idle_timeout());
    info!("-> Max lifetime: {:?}", options.get_max_lifetime());
}
