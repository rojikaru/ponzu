use crate::models::genre::Genre;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenreDto {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub r#type: String,
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateGenreDto {
    pub mal_id: u64,
    pub r#type: String,
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateGenreDto {
    pub mal_id: Option<u64>,
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub count: Option<u64>,
}

impl From<Genre> for GenreDto {
    fn from(genre: Genre) -> Self {
        Self {
            id: genre.id,
            mal_id: genre.mal_id,
            r#type: genre.r#type,
            name: genre.name,
            count: genre.count,
        }
    }
}

impl From<CreateGenreDto> for Genre {
    fn from(dto: CreateGenreDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            r#type: dto.r#type,
            name: dto.name,
            count: dto.count,
        }
    }
}
