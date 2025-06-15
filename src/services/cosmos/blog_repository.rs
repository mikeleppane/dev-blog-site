use azure_core::error::Error as AzureError;
use azure_data_cosmos::PartitionKey;
use color_eyre::{eyre::WrapErr, Result};
use futures::TryStreamExt;
use leptos::leptos_dom::logging;

const PARTITION_KEY: &str = "Mikko Leppänen";

use crate::{models::blog::BlogPost, services::cosmos::CosmosClientManager};

pub struct BlogService {
    client: CosmosClientManager,
}

impl BlogService {
    /// Creates a new instance of the Cosmos DB service.
    ///
    /// # Errors
    ///
    /// Returns an error if the app configuration cannot be retrieved or if the Cosmos client
    /// cannot be initialized with the provided connection details.
    pub fn new(
        client: CosmosClientManager,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self { client })
    }

    /// Creates a new todo item in the Cosmos DB container.
    ///
    /// # Errors
    ///
    /// Returns an `AzureError` if the creation operation fails or if there's an issue
    /// connecting to the Cosmos DB service.
    pub async fn create_blog(
        &self,
        blog: BlogPost,
    ) -> Result<BlogPost, Box<dyn std::error::Error + Send + Sync>> {
        let blog_cloned = blog.clone();
        let partition_key = PartitionKey::from(PARTITION_KEY);
        match self
            .client
            .get_container("blogs")?
            .create_item(partition_key, blog, None)
            .await
        {
            Ok(_) => {
                logging::console_log(&format!("Created blog in Cosmos DB: {blog_cloned:#?}",));
                Ok(blog_cloned)
            }
            Err(e) => {
                logging::console_error("ERROR");
                eprintln!("Error creating blog in Cosmos DB: {e}");
                Err(Box::new(e))
            }
        }
    }

    /// Retrieves a blog post by its ID from the Cosmos DB container.
    ///
    /// # Errors
    ///
    /// Returns an `AzureError` if the read operation fails, if there's an issue
    /// connecting to the Cosmos DB service, or if the response body cannot be parsed.
    pub async fn get_blog_post(&self, id: &str) -> azure_core::Result<BlogPost> {
        let partition_key = PartitionKey::from(PARTITION_KEY);

        logging::console_log(&format!("Fetching blog post with ID: {id}"));
        let result = self
            .client
            .get_container("blogs")
            .map_err(|e| {
                logging::console_error(&format!("Error getting container: {e}"));
                azure_core::Error::new(azure_core::error::ErrorKind::Io, e)
            })?
            .read_item(partition_key, id, None)
            .await;
        logging::console_log(&format!("Read item result: {result:?}"));

        match result {
            Ok(response) => {
                logging::console_log("Successfully read item from Cosmos DB");
                let blog_post: BlogPost = response.into_json_body().await.map_err(|e| {
                    logging::console_error(&format!("Error parsing response body: {e}"));
                    azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
                })?;
                logging::console_log(&format!("Retrieved blog post: {blog_post:#?}"));
                Ok(blog_post)
            }
            Err(e) => {
                logging::console_error(&format!("Error reading item: {e}"));
                Err(e)
            }
        }
    }

    /// Retrieves a list of todo items from the Cosmos DB container for a specific todo ID.
    ///
    /// # Errors
    ///
    /// Returns an `AzureError` if the query operation fails or if there's an issue
    /// connecting to the Cosmos DB service.
    pub async fn list_blog_posts(
        &self,
    ) -> Result<Vec<BlogPost>, Box<dyn std::error::Error + Send + Sync>> {
        // Use a more explicit query approach
        let query = "SELECT * FROM c ORDER BY c.created_at DESC";
        let partition_key = PartitionKey::from(PARTITION_KEY);

        logging::console_log("Starting Cosmos DB query for blog posts...");

        let mut blogs = Vec::new();

        // Create the query stream
        let query_result = self
            .client
            .get_container("blogs")
            .map_err(|e| {
                logging::console_error(&format!("Failed to get container for blogs: {e}"));
                e
            })?
            .query_items::<BlogPost>(query, partition_key, None);

        match query_result {
            Ok(mut query_stream) => {
                logging::console_log("Query stream created successfully");

                // Process the stream more carefully
                loop {
                    match query_stream.try_next().await {
                        Ok(Some(feed_page)) => {
                            logging::console_log(&format!(
                                "Received feed page with {} items",
                                feed_page.items().len()
                            ));

                            for item in feed_page.items() {
                                logging::console_log(&format!("Processing item: {item:#?}"));
                                blogs.push(item.clone());
                            }
                        }
                        Ok(None) => {
                            break; // No more pages
                        }
                        Err(e) => {
                            logging::console_error(&format!(
                                "Error reading from query stream: {e}"
                            ));
                            return Err(Box::new(e));
                        }
                    }
                }
            }
            Err(e) => {
                logging::console_error(&format!("Error creating query stream: {e}"));
                return Err(Box::new(e));
            }
        }

        logging::console_log(&format!("Retrieved {} todos from Cosmos DB", blogs.len()));
        Ok(blogs)
    }

    /// Deletes a todo item from the Cosmos DB container
    ///
    /// # Errors
    ///
    /// Returns an `AzureError` if the deletion operation fails or if there's an issue
    /// connecting to the Cosmos DB service.
    pub async fn delete_blog(&self, blog_id: &str) -> Result<(), AzureError> {
        let partition_key = PartitionKey::from(PARTITION_KEY);

        self.client
            .get_container("blogs")
            .map_err(|e| {
                logging::console_error(&format!("Error getting container: {e}"));
                AzureError::new(azure_core::error::ErrorKind::Io, e)
            })?
            .delete_item(partition_key, blog_id, None)
            .await?;

        Ok(())
    }

    /// Retrieves the latest blog posts from the Cosmos DB container.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of blog posts to retrieve
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The query operation fails
    /// - There's an issue connecting to the Cosmos DB service
    /// - The response cannot be parsed
    pub async fn list_latest_blog_posts(&self, limit: i32) -> Result<Vec<BlogPost>> {
        let query = format!(
            "SELECT * FROM c WHERE c.published = true ORDER BY c.created_at DESC OFFSET 0 LIMIT {limit}"
        );
        let partition_key = PartitionKey::from(PARTITION_KEY);

        let mut blogs = Vec::new();
        let query_result = self
            .client
            .get_container("blogs")
            .map_err(|e| {
                logging::console_error(&format!("Failed to get container for blogs: {e}"));
                e
            })?
            .query_items::<BlogPost>(query, partition_key, None);

        match query_result {
            Ok(mut query_stream) => {
                logging::console_log("Query stream created successfully");

                // Process the stream more carefully
                loop {
                    match query_stream.try_next().await {
                        Ok(Some(feed_page)) => {
                            logging::console_log(&format!(
                                "Received feed page with {} items",
                                feed_page.items().len()
                            ));

                            for item in feed_page.items() {
                                logging::console_log(&format!("Processing item: {item:#?}"));
                                blogs.push(item.clone());
                            }
                        }
                        Ok(None) => {
                            break; // No more pages
                        }
                        Err(e) => {
                            logging::console_error(&format!(
                                "Error reading from query stream: {e}"
                            ));
                            return Err(e).wrap_err("Failed to read from query stream");
                        }
                    }
                }
            }
            Err(e) => {
                logging::console_error(&format!("Error creating query stream: {e}"));
                return Err(e).wrap_err("Failed to create query stream");
            }
        }

        logging::console_log(&format!("Retrieved {} todos from Cosmos DB", blogs.len()));
        Ok(blogs)
    }
}

// Global lazy-initialized instance
#[allow(clippy::redundant_closure)]
static BLOG_SERVICE: std::sync::LazyLock<
    Result<
        BlogService,
        Box<dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static>,
    >,
> = std::sync::LazyLock::new(|| {
    use crate::services::config::get_config;

    let app_config = get_config();
    let client = CosmosClientManager::new(app_config)
        .map_err(|e| format!("Failed to create Cosmos DB client: {e}"))?;
    Ok(BlogService::new(client).map_err(|e| format!("Failed to create Cosmos service: {e}"))?)
});

// Helper function to get the global instance
/// Returns a reference to the global Cosmos DB service instance.
///
/// # Errors
///
/// Returns an error if the Cosmos DB service failed to initialize.
#[allow(clippy::borrowed_box)]
pub fn get_blog_service() -> Result<
    &'static BlogService,
    &'static Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>,
> {
    BLOG_SERVICE.as_ref()
}

/// Initialize the database and container on first access
///
/// # Errors
///
/// Returns an error if the Cosmos DB service cannot be initialized or accessed.
pub fn initialize_blogs_db() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    get_blog_service().map_err(|e| format!("Failed to get Cosmos service: {e}"))?;

    Ok(())
}
