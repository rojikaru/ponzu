use crate::types::links::Images;
use serde::{Deserialize, Serialize};

/// A lightweight representation of either an anime or manga entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryDto {
    pub id: String,
    pub mal_id: u64,
    pub url: String,
    pub images: Images,
    pub title: String,
}
