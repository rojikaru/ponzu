use serde::{Deserialize, Serialize};

/// External links for the title.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalLink {
    pub name: String,
    pub url: String,
}

/// Images for the title in different formats.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub jpg: ImageUrls,
    pub webp: ImageUrls,
}

/// Image URLs for the title.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrls {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

/// Trailer for the title.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trailer {
    pub youtube_id: String,
    pub url: String,
    pub embed_url: String,
}
