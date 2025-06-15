use color_eyre::{eyre::WrapErr, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub cosmos: CosmosConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosConfig {
    pub uri: String,
    pub database_name: String,
    pub containers: HashMap<String, ContainerConfig>,
    pub primary_key: String, // For development only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub name: String,
    pub partition_key: String,
    pub throughput: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    /// Creates an `AppConfig` from environment variables.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the required environment variables
    /// are not set:
    /// - `COSMOS_DB_URI` - The URI for the Cosmos DB instance
    /// - `COSMOS_DB_DATABASE` - The name of the Cosmos database
    /// - `COSMOS_DB_KEY` - The primary key for Cosmos DB access
    pub fn from_env() -> Result<Self> {
        let cosmos_db_uri = std::env::var("COSMOS_DB_URI")
            .wrap_err("COSMOS_DB_URI environment variable not set")?;

        let cosmos_database_name = std::env::var("COSMOS_DB_DATABASE")
            .wrap_err("COSMOS_DB_DATABASE environment variable not set")?;

        let cosmos_primary_key = std::env::var("COSMOS_DB_KEY").wrap_err(
            "COSMOS_DB_KEY environment variable not set. This is required for development.",
        )?;

        // Define container configurations
        let mut containers = HashMap::new();

        // Blog posts container
        containers.insert(
            "blogs".to_string(),
            ContainerConfig {
                name: std::env::var("COSMOS_BLOGS_CONTAINER_NAME")
                    .unwrap_or_else(|_| "posts".to_string()),
                partition_key: "author".to_string(),
                throughput: Some(400),
            },
        );

        // Books container
        containers.insert(
            "books".to_string(),
            ContainerConfig {
                name: std::env::var("COSMOS_BOOKS_CONTAINER_NAME")
                    .unwrap_or_else(|_| "books".to_string()),
                partition_key: "category".to_string(),
                throughput: Some(400),
            },
        );

        // Add more containers as needed
        // containers.insert("users".to_string(), ContainerConfig {
        //     name: std::env::var("COSMOS_USERS_CONTAINER_NAME")
        //         .unwrap_or_else(|_| "users".to_string()),
        //     partition_key: "/userId".to_string(),
        //     throughput: Some(400),
        // });

        let cosmos_config = CosmosConfig {
            uri: cosmos_db_uri,
            database_name: cosmos_database_name,
            containers,
            primary_key: cosmos_primary_key,
        };

        let server_config = ServerConfig {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        };

        Ok(AppConfig {
            cosmos: cosmos_config,
            server: server_config,
        })
    }

    #[must_use]
    pub fn get_container_config(&self, container_type: &str) -> Option<&ContainerConfig> {
        self.cosmos.containers.get(container_type)
    }
}

static APP_CONFIG: std::sync::LazyLock<miette::Result<AppConfig>> =
    std::sync::LazyLock::new(|| {
        let config = AppConfig::from_env();
        match config {
            Ok(cfg) => Ok(cfg),
            Err(e) => Err(miette::miette!(format!(
                "Failed to load application configuration: {e}"
            ))),
        }
    });

/// Gets the application configuration.
///
/// # Panics
///
/// Panics if required environment variables are not set:
/// - `COSMOS_DB_URI`
/// - `COSMOS_DB_KEY`
/// - `DEV_SITE_ADMIN_USERNAME`
/// - `DEV_SITE_ADMIN_PASSWORD`
/// - `SESSION_TIMEOUT_HOURS` (if set but not a valid number)
///
pub fn get_config() -> &'static AppConfig {
    match APP_CONFIG.as_ref() {
        Ok(config) => config,
        Err(e) => {
            panic!("{}", format!(
                "Failed to load application configuration. Please ensure all required environment variables are set. Error:\n{e}")
            );
        }
    }
}

/// Initializes the application configuration.
///
/// # Errors
///
/// This function will return an error if the configuration cannot be loaded,
/// which can happen when required environment variables are not set.
pub fn init_config() -> Result<()> {
    get_config();
    Ok(())
}
