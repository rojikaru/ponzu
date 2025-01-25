use crate::models::bson_utils::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use mongodb::bson::serde_helpers::{
    deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string,
};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub user: String, // Reference to User ID
    pub score: i32,
    pub content: String,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string"
    )]
    pub created_at: DateTime,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string"
    )]
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeReview {
    #[serde(flatten)]
    pub review: Review,
    pub anime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MangaReview {
    #[serde(flatten)]
    pub review: Review,
    pub manga: String,
}

impl AnimeReview {
    pub fn new(user_id: String, anime_id: String, score: i32, content: String) -> Self {
        Self {
            review: Review {
                id: None,
                user: user_id,
                score,
                content,
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            },
            anime: anime_id,
        }
    }
}

impl MangaReview {
    pub fn new(user_id: String, anime_id: String, score: i32, content: String) -> Self {
        Self {
            review: Review {
                id: None,
                user: user_id,
                score,
                content,
                created_at: DateTime::now(),
                updated_at: DateTime::now(),
            },
            manga: anime_id,
        }
    }
}
