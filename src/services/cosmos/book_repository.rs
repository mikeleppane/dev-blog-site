use azure_data_cosmos::PartitionKey;
use color_eyre::{eyre::WrapErr, Result};
use futures::TryStreamExt;
use leptos::leptos_dom::logging;

use crate::{
    models::books::{Book, BookCategory},
    services::cosmos::CosmosClientManager,
};
pub struct BookService {
    client: CosmosClientManager,
}

impl BookService {
    /// Creates a new instance of the Cosmos DB service.
    ///
    /// # Errors
    ///
    /// Returns an error if the app configuration cannot be retrieved or if the Cosmos client
    /// cannot be initialized with the provided connection details.
    pub fn new(client: CosmosClientManager) -> Result<Self> {
        Ok(Self { client })
    }

    /// Retrieves all books from all categories in the Cosmos DB.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to get books for any category
    /// - The Cosmos DB container cannot be accessed
    /// - The query execution fails for any category
    pub async fn get_all_books(&self) -> Result<Vec<Book>> {
        // Use a more explicit query approach
        let mut all_books = Vec::new();

        for category in BookCategory::all_categories() {
            logging::console_log(&format!(
                "Fetching books for category: {}",
                category.display_name()
            ));

            let books = self
                .get_books_by_category(&category)
                .await
                .wrap_err(format!(
                    "Failed to get books for category: {}",
                    category.display_name()
                ))?;

            logging::console_log(&format!(
                "Found {} books in category: {}",
                books.len(),
                category.display_name()
            ));

            all_books.extend(books);
        }
        logging::console_log(&format!(
            "Total books retrieved across all categories: {}",
            all_books.len()
        ));
        Ok(all_books)
    }

    /// Retrieves all books for a specific category from the Cosmos DB.
    ///
    /// # Arguments
    ///
    /// * `category` - The book category to filter by
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Cosmos DB container cannot be accessed
    /// - The query execution fails
    /// - There are issues deserializing the book data
    pub async fn get_books_by_category(&self, category: &BookCategory) -> Result<Vec<Book>> {
        let query = format!(
            "SELECT * FROM c WHERE c.category = '{}' ORDER BY c.publication_year DESC",
            category.db_name()
        );
        let partition_key = PartitionKey::from("python");

        logging::console_log(&format!(
            "Starting Cosmos DB query for books in category: {}",
            category.display_name()
        ));

        let query_result = self
            .client
            .get_container("books")
            .map_err(|e| {
                logging::console_log(&format!("Failed to get container for books: {e}"));
                e
            })?
            .query_items::<Book>(&query, partition_key, None);

        match query_result {
            Ok(mut query_stream) => {
                let mut books = Vec::new();

                while let Some(feed_page) = query_stream.try_next().await? {
                    logging::console_log(&format!(
                        "Received feed page with {} items",
                        feed_page.items().len()
                    ));

                    for item in feed_page.items() {
                        logging::console_log(&format!("Processing item: {item:#?}"));
                        books.push(item.clone());
                    }
                }

                logging::console_log(&format!("Retrieved {} books", books.len()));
                Ok(books)
            }
            Err(e) => {
                logging::console_log(&format!("Error querying Cosmos DB for books: {e}"));
                Err(color_eyre::eyre::eyre!(
                    "Error querying Cosmos DB for books: {}",
                    e
                ))
            }
        }
    }
}

// Global lazy-initialized instance
#[allow(clippy::redundant_closure)]
static BOOK_SERVICE: std::sync::LazyLock<Result<BookService>> = std::sync::LazyLock::new(|| {
    use crate::services::config::get_config;

    let app_config = get_config();

    let client =
        CosmosClientManager::new(app_config).wrap_err("Failed to create Cosmos client manager")?;
    BookService::new(client).wrap_err("Failed to create Cosmos service")
});

// Helper function to get the global instance
/// Returns a reference to the global Cosmos DB service instance.
///
/// # Errors
///
/// Returns an error if the Cosmos DB service failed to initialize.
#[allow(clippy::borrowed_box)]
pub fn get_book_service() -> Result<&'static BookService> {
    BOOK_SERVICE
        .as_ref()
        .map_err(|e| color_eyre::eyre::eyre!(format!("Failed to get book service: {e}")))
}

/// Initialize the database and container on first access
///
/// # Errors
///
/// Returns an error if the Cosmos DB service cannot be initialized or accessed.
pub fn initialize_books_db() -> Result<()> {
    get_book_service()?;

    Ok(())
}
