use leptos::{prelude::ServerFnError, server};

use crate::models::blog::BlogPost;

#[server(GetBlogs, "/api")]
pub async fn get_blogs_server() -> Result<Vec<BlogPost>, ServerFnError> {
    use crate::services::cosmos::blog_repository::get_blog_service;
    use leptos::logging;

    // Get the blog service
    let blog_service = get_blog_service()
        .map_err(|e| ServerFnError::new(format!("Failed to get Cosmos service: {e}")))?;

    // Fetch blog posts with proper error handling
    let cosmos_blogs = blog_service
        .list_blog_posts() // Fetch up to 50 posts
        .await
        .map_err(|e| {
            logging::error!("Failed to fetch blogs from Cosmos DB: {:?}", e);
            ServerFnError::new(format!("Failed to get blogs: {e}"))
        })?;

    logging::log!("Retrieved {} blogs from Cosmos DB", cosmos_blogs.len());

    // Convert to BlogPost models
    let blog_posts: Vec<BlogPost> = cosmos_blogs.into_iter().collect();

    Ok(blog_posts)
}

#[server(GetBlog, "/api")]
pub async fn get_blog_server(id: String) -> Result<BlogPost, ServerFnError> {
    use crate::services::cosmos::blog_repository::get_blog_service;
    use leptos::logging;

    // Get the blog service
    let blog_service = get_blog_service()
        .map_err(|e| ServerFnError::new(format!("Failed to get Cosmos service: {e}")))?;

    // Fetch the blog post by ID with proper error handling
    let cosmos_blog = blog_service.get_blog_post(&id).await.map_err(|e| {
        logging::error!("Failed to fetch blog post from Cosmos DB: {:?}", e);
        ServerFnError::new(format!("Failed to get blog post: {e}"))
    })?;

    logging::log!("Retrieved blog post with ID: {}", id);

    // Convert to BlogPost model
    Ok(cosmos_blog)
}

#[server(GetLatestBlogs, "/api")]
pub async fn get_latest_blogs_server(limit: i32) -> Result<Vec<BlogPost>, ServerFnError> {
    use crate::services::cosmos::blog_repository::get_blog_service;
    use leptos::logging;

    // Get the blog service
    let blog_service = get_blog_service()
        .map_err(|e| ServerFnError::new(format!("Failed to get blog service: {}", e)))?;

    // Fetch latest blogs with improved error handling
    let blogs = blog_service
        .list_latest_blog_posts(limit)
        .await
        .map_err(|e| {
            logging::error!("Failed to fetch latest blogs from Cosmos DB: {:?}", e);
            ServerFnError::new(format!("Failed to get latest blogs: {}", e))
        })?;

    logging::log!("Retrieved {} latest blogs from Cosmos DB", blogs.len());
    Ok(blogs)
}
