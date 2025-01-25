use crate::models::genre::Genre;
use crate::models::producer::Producer;
use crate::models::bson_utils::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: i32,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub episodes: i32,
    pub genres: Vec<Genre>,
    pub demographics: Vec<Genre>,
    pub synopsis: String,
    pub status: String,
    pub producers: Vec<Producer>,
    pub year: i32,
    pub popularity: Option<f32>,
    pub score: f32,
    pub rank: i32,
    pub images: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
    #[serde(flatten)]
    pub title: Title,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manga {
    #[serde(flatten)]
    pub title: Title,
}

impl Title {
    pub fn new(
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
    ) -> Self {
        Self {
            id: None,
            mal_id,
            title,
            title_english,
            title_japanese,
            title_synonyms,
            r#type,
            episodes,
            genres,
            demographics,
            synopsis,
            status,
            producers,
            year,
            popularity,
            score,
            rank,
            images,
        }
    }
}

impl Anime {
    pub fn new(
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
    ) -> Self {
        Self {
            title: Title::new(
                mal_id,
                title,
                title_english,
                title_japanese,
                title_synonyms,
                r#type,
                episodes,
                genres,
                demographics,
                synopsis,
                status,
                producers,
                year,
                popularity,
                score,
                rank,
                images,
            ),
        }
    }
}

impl Manga {
    pub fn new(
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
    ) -> Self {
        Self {
            title: Title::new(
                mal_id,
                title,
                title_english,
                title_japanese,
                title_synonyms,
                r#type,
                episodes,
                genres,
                demographics,
                synopsis,
                status,
                producers,
                year,
                popularity,
                score,
                rank,
                images,
            ),
        }
    }
}
