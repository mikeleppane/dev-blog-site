use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub image_url: Option<String>,
}

impl BlogPost {
    #[must_use]
    pub fn new(
        title: String,
        content: String,
        author: String,
        tags: Vec<String>,
        image_url: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            author,
            published: false,
            tags,
            created_at: now,
            updated_at: now,
            image_url,
        }
    }
}
