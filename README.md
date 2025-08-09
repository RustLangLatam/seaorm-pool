# SeaORM Connection Pool (`seaorm-pool`)

[![Rust 2024](https://img.shields.io/badge/Rust-2024-93450a?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/actions/workflow/status/RustLangLatam/seaorm-pool/rust.yml?branch=main&style=for-the-badge&logo=githubactions)](https://github.com/[RustLangLatam/seaorm-pool/actions)
[![Crates.io](https://img.shields.io/crates/v/seaorm-pool?label=seaorm-pool&style=for-the-badge&logo=rust)](https://crates.io/crates/seaorm-pool)
[![SeaORM](https://img.shields.io/badge/SeaORM-1.1-00758F?style=for-the-badge&logo=rust)](https://www.sea-orm.com/)
[![TiDB](https://img.shields.io/badge/Supports-TiDB-4E64E7?style=for-the-badge&logo=tidb)](https://www.pingcap.com/tidb/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue?style=for-the-badge)](https://opensource.org/licenses/MIT)

A robust and easy-to-use utility for creating a `sea-orm` connection pool from a configuration file. Designed for async Rust applications connecting to MySQL-compatible databases like TiDB.

## Overview

`seaorm-pool` eliminates the boilerplate of setting up a database connection. It provides a clean, configuration-driven function that creates a fully configured `sea-orm` connection pool, ready to use in your application.

It is built on top of industry-standard crates to ensure reliability and performance:

- **[SeaORM](https://www.sea-orm.com/)**: The core async ORM for Rust.
- **[Serde](https://serde.rs/)**: For effortless serialization and deserialization of your configuration.
- **[Tokio](https://tokio.rs/)**: As the underlying async runtime.
- **[Tracing](https://github.com/tokio-rs/tracing)**: For structured, context-aware logging.

## ✨ Key Features

- **Configuration-Driven**: Define your entire database setup in a single TOML file.
- **Robust Connection Pooling**: Fine-tune the connection pool with options for `max_connections`, `min_connections`, `idle_timeout`, `max_lifetime`, and more.
- **Async Ready**: A single async function call is all you need.
- **Secure by Default**: Easily configure SSL/TLS for encrypted database connections.
- **Structured Logging**: Integrated with `tracing` to provide clear insight into the connection lifecycle.

## 🚀 Installation

Add `seaorm-pool` and its required peer dependencies to your `Cargo.toml`:

```toml
## ⚙️ Usage Quick Start

### 1. Create a Configuration File

Create a `Settings.toml` file in your project's root directory. **Note that the configuration must be under a `[database]` table** to match the crate's `AppConfig` struct.

```toml
# Settings.toml

# The `[database]` table is required.
[database]
host = "gateway01.eu-central-1.prod.aws.tidbcloud.com"
port = 4000
username = "your_username"
password = "your_password"
databaseName = "your_db"
# Optional: Path to your SSL CA certificate for secure connections.
# sslCa = "/path/to/your/ca.pem"

# Nested table for connection pool options.
[database.poolOptions]
maxConnections = 10
minConnections = 5
acquireTimeout = "30s"
idleTimeout = "10m"
maxLifetime = "30m"
isLazy = true
```

### 2. Connect in Your Application

Use the `create_connection_pool` function to establish the database connection.

```rust
// src/main.rs

use sea_orm::{DatabaseConnection, Statement, DatabaseBackend};
use seaorm_pool::{config::AppConfig, create_connection_pool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load configuration using the `config` crate.
    let settings = config::Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()?;

    let app_config: AppConfig = settings.try_deserialize()?;

    // 2. Create the database connection pool from the config.
    println!("Establishing connection pool...");
    let pool: DatabaseConnection = create_connection_pool(app_config.database).await?;
    println!("Connection pool established successfully!");

    // 3. Use the pool to execute a query.
    let result = pool.execute(Statement::from_string(
        DatabaseBackend::MySql,
        "SELECT 'Connection successful!' as message;".to_string(),
    )).await?;

    println!("Query executed. Rows affected: {}", result.rows_affected());

    Ok(())
}
```

## 📋 Configuration Details

The crate is configured through the `AppConfig` struct, which contains a `database` field of type `DatabaseConfig`.

### Main Settings (`[database]`)

| Field          | Type           | Required | Description                                                                |
| -------------- | -------------- | -------- | -------------------------------------------------------------------------- |
| `host`         | String         | Yes      | Hostname or IP address of the TiDB/MySQL server.                           |
| `port`         | `u16`          | No       | Server port. Defaults to `4000` if connecting to TiDB.                     |
| `username`     | String         | Yes      | Username for database authentication.                                      |
| `password`     | String         | Yes      | Password for database authentication.                                      |
| `databaseName` | String         | Yes      | The specific database (schema) to connect to.                              |
| `sslCa`        | String         | No       | Path to the SSL Certificate Authority (CA) file for enabling TLS.          |

### Pool Options (`[database.poolOptions]`)

| Field                  | Type      | Default      | Description                                                                    |
| ---------------------- | --------- | ------------ | ------------------------------------------------------------------------------ |
| `maxConnections`       | `u32`     | `10`         | Maximum number of concurrent connections the pool can open.                    |
| `minConnections`       | `u32`     | `1`          | Minimum number of idle connections to maintain in the pool.                    |
| `acquireTimeout`       | `String`  | `"30s"`      | Time to wait for a connection before timing out (e.g., "5s", "1m").            |
| `idleTimeout`          | `String`  | `"5m"`       | Time a connection can be idle before it is closed (e.g., "10m", "1h").         |
| `maxLifetime`          | `String`  | `"30m"`      | Maximum lifetime of a single connection before it is recycled.                 |
| `isLazy`               | `bool`    | `true`       | If `true`, connections are established only when first needed.                 |
| `statementCacheCapacity` | `usize` | `100`        | The number of prepared statements to cache per connection.                     |


## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request for any improvements or bug fixes.