use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub published_date: DateTime<Utc>,
    pub is_available: bool,
}

impl Book {
    pub fn new(id: u32, title: &str, author: &str) -> Self {
        Book {
            id,
            title: title.to_string(),
            author: author.to_string(),
            published_date: Utc::now(),
            is_available: true,
        }
    }
}