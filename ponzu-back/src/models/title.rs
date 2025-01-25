use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::genre::Genre;
use crate::models::producer::Producer;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Title {
    mal_id: i32,
    title: String,
    title_english: String,
    title_japanese: String,
    title_synonyms: Vec<String>,
    r#type: String,
    episodes: i32,
    genres: Vec<Genre>,
    demographics: Vec<Genre>,
    synopsis: String,
    status: String,
    producers: Vec<Producer>,
    year: i32,
    popularity: Option<f32>,
    score: f32,
    rank: i32,
    images: Value,
}

// Inherit Title struct for Anime
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Anime {
    #[serde(flatten)]
    title: Title
}

#[derive(PartialEq, Debug, Serialize, Deserialize,)]
pub struct Manga {
    #[serde(flatten)]
    title: Title,
}
