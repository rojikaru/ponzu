use crate::models::links::{ExternalLink, Images};
use crate::models::title_meta::MalEntity;
use serde::{Deserialize, Serialize};

/// Commonly-used types for title metadata with type, name, and URL fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producer {
    pub mal_id: u64,
    pub titles: Vec<MalEntity>,
    pub images: Option<Images>,
    pub favorites: u64,
    pub count: u64,
    pub established: String,
    pub about: String,
    pub external: Vec<ExternalLink>,
}
