use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub category: BookCategory,
    pub rating: f32, // 1.0 to 5.0
    pub isbn: Option<String>,
    pub amazon_url: Option<String>,
    pub goodreads_url: Option<String>,
    pub cover_image_url: Option<String>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub personal_review: Option<String>,
    pub key_takeaways: Vec<String>,
    pub recommended_for: Vec<String>, // e.g., ["beginners", "intermediate", "advanced"]
    pub tags: Vec<String>,
    pub added_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BookCategory {
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "code-craftsmanship-essentials")]
    CodeCraftsmanshipEssentials,
    #[serde(rename = "docker-kubernetes")]
    DockerKubernetes,
    #[serde(rename = "event-driven-architecture")]
    EventDrivenArchitecture,
    #[serde(rename = "modern-devops-and-delivery")]
    ModernDevOpsAndDelivery,
    #[serde(rename = "software-architectures-and-patterns")]
    SoftwareArchitecturesAndPatterns,
}

impl BookCategory {
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            BookCategory::Rust => "Rust Programming Professional",
            BookCategory::Python => "Python Expert",
            BookCategory::CodeCraftsmanshipEssentials => "Code Craftsmanship Essentials",
            BookCategory::DockerKubernetes => "Docker & Kubernetes",
            BookCategory::EventDrivenArchitecture => "Event-Driven Architecture",
            BookCategory::ModernDevOpsAndDelivery => "Modern DevOps and Delivery",
            BookCategory::SoftwareArchitecturesAndPatterns => "Software Architectures and Patterns",
        }
    }

    #[must_use]
    pub fn db_name(&self) -> &'static str {
        match self {
            BookCategory::Rust => "rust",
            BookCategory::Python => "python",
            BookCategory::CodeCraftsmanshipEssentials => "code-craftsmanship-essentials",
            BookCategory::DockerKubernetes => "docker-kubernetes",
            BookCategory::EventDrivenArchitecture => "event-driven-architecture",
            BookCategory::ModernDevOpsAndDelivery => "modern-devops-and-delivery",
            BookCategory::SoftwareArchitecturesAndPatterns => "software-architectures-and-patterns",
        }
    }

    #[must_use]
    pub fn color_class(&self) -> &'static str {
        match self {
            BookCategory::Rust => "bg-orange-100 text-orange-800",
            BookCategory::Python => "bg-blue-100 text-blue-800",
            BookCategory::CodeCraftsmanshipEssentials => "bg-green-100 text-green-800",
            BookCategory::DockerKubernetes => "bg-purple-100 text-purple-800",
            BookCategory::EventDrivenArchitecture => "bg-yellow-100 text-yellow-800",
            BookCategory::ModernDevOpsAndDelivery => "bg-teal-100 text-teal-800",
            BookCategory::SoftwareArchitecturesAndPatterns => "bg-gray-100 text-gray-800",
        }
    }

    #[must_use]
    pub fn all_categories() -> Vec<BookCategory> {
        vec![
            BookCategory::Rust,
            BookCategory::Python,
            BookCategory::CodeCraftsmanshipEssentials,
            BookCategory::DockerKubernetes,
            BookCategory::EventDrivenArchitecture,
            BookCategory::ModernDevOpsAndDelivery,
            BookCategory::SoftwareArchitecturesAndPatterns,
        ]
    }
}

impl Iterator for BookCategory {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let categories = Self::all_categories();
        let current_index = categories.iter().position(|c| c == self)?;
        let next_index = (current_index + 1) % categories.len();
        Some(categories[next_index].clone())
    }
}

impl Default for Book {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: String::new(),
            author: String::new(),
            description: String::new(),
            category: BookCategory::Python,
            rating: 0.0,
            isbn: None,
            amazon_url: None,
            goodreads_url: None,
            cover_image_url: None,
            publication_year: None,
            pages: None,
            personal_review: None,
            key_takeaways: Vec::new(),
            recommended_for: Vec::new(),
            tags: Vec::new(),
            added_date: Utc::now(),
            last_updated: Utc::now(),
        }
    }
}
