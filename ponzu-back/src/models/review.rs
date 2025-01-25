use serde::{Deserialize, Serialize};
use crate::models::title::{Anime, Manga};

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub user: i32,
    pub score: i32,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeReview {
    #[serde(flatten)]
    review: Review,
    anime: Anime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MangaReview {
    #[serde(flatten)]
    review: Review,
    manga: Manga,
}