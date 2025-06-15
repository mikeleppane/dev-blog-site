use azure_core::credentials::Secret;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::CosmosClient;
use color_eyre::{eyre::WrapErr, Result};
use std::sync::Arc;
use std::{collections::HashMap, sync::OnceLock};

use crate::services::config::{AppConfig, ContainerConfig};

pub struct CosmosClientManager {
    client: CosmosClient,
    database_name: String,
    containers: HashMap<String, ContainerConfig>,
}

impl CosmosClientManager {
    /// Creates a new Cosmos client manager from the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the Cosmos client cannot be created with the provided credentials.
    pub fn new(config: &AppConfig) -> Result<Self> {
        let client = CosmosClient::with_key(
            &config.cosmos.uri,
            Secret::from(config.cosmos.primary_key.clone()),
            None,
        )
        .wrap_err("Failed to create Cosmos client")?;

        Ok(Self {
            client,
            database_name: config.cosmos.database_name.clone(),
            containers: config.cosmos.containers.clone(),
        })
    }

    /// Gets a container client for the specified container type.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified container type is not configured.
    pub fn get_container(&self, container_type: &str) -> Result<ContainerClient> {
        let container_config = self.containers.get(container_type).ok_or_else(|| {
            color_eyre::eyre::eyre!("Container type '{}' not configured", container_type)
        })?;

        let database_client = self.client.database_client(&self.database_name);
        let container_client = database_client.container_client(&container_config.name);

        Ok(container_client)
    }

    #[must_use]
    pub fn list_container_types(&self) -> Vec<String> {
        self.containers.keys().cloned().collect()
    }
}

// Global client manager instance
static CLIENT_MANAGER: OnceLock<Arc<CosmosClientManager>> = OnceLock::new();

/// Gets the global Cosmos client manager instance.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to load app configuration from environment
/// - Failed to create Cosmos client manager
pub fn get_cosmos_client_manager() -> Result<Arc<CosmosClientManager>> {
    if let Some(manager) = CLIENT_MANAGER.get() {
        return Ok(manager.clone());
    }

    let config = AppConfig::from_env().wrap_err("Failed to load app configuration")?;
    let manager = Arc::new(
        CosmosClientManager::new(&config).wrap_err("Failed to create Cosmos client manager")?,
    );

    match CLIENT_MANAGER.set(manager.clone()) {
        Ok(()) => Ok(manager),
        Err(_) => CLIENT_MANAGER
            .get()
            .map(std::clone::Clone::clone)
            .ok_or_else(|| {
                color_eyre::eyre::eyre!("Failed to get client manager after set failure")
            }),
    }
}

/// Gets a container client for the specified container type.
///
/// # Errors
///
/// Returns an error if:
/// - Failed to get the Cosmos client manager
/// - The specified container type is not configured
pub fn get_container_client(container_type: &str) -> Result<ContainerClient> {
    let manager = get_cosmos_client_manager().wrap_err("Failed to get Cosmos client manager")?;
    manager.get_container(container_type)
}
