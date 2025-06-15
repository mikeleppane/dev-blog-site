use leptos::prelude::ServerFnError;
use leptos::server;

use crate::models::books::Book;

#[server(GetBooks, "/api")]
pub async fn get_books_server() -> Result<Vec<Book>, ServerFnError> {
    use crate::services::cosmos::book_repository::get_book_service;
    use leptos::logging;
    // Get the book repository
    let book_repo = get_book_service()
        .map_err(|e| ServerFnError::new(format!("Failed to get book repository: {e}")))?;

    // Fetch books with proper error handling
    let books = book_repo.get_all_books().await.map_err(|e| {
        logging::error!("Failed to fetch books from Cosmos DB: {:?}", e);
        ServerFnError::new(format!("Failed to get books: {e}"))
    })?;

    logging::log!("Retrieved {} books from Cosmos DB", books.len());

    Ok(books)
}
