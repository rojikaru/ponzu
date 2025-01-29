use crate::models::genre::Genre;
use crate::models::links::{ExternalLink, Images};
use crate::models::title_meta::{MalEntity, Relation, Title};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// Manga model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manga {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: Published,
    // pub score: f64,
    pub scored_by: u64,
    // pub rank: u64,
    // pub popularity: u64,
    pub members: u64,
    pub favorites: u64,
    pub synopsis: String,
    pub background: String,
    pub authors: Vec<MalEntity>,
    pub serializations: Vec<MalEntity>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<MalEntity>,
    pub themes: Vec<MalEntity>,
    pub demographics: Vec<MalEntity>,
    pub relations: Vec<Relation>,
    pub external: Vec<ExternalLink>,
}

/// Aired information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Published {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub from: Option<DateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub to: Option<DateTime>,
    pub prop: PublishedProp,
}

/// Published information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishedProp {
    pub from: PublishedPropFromTo,
    pub to: PublishedPropFromTo,
    pub string: String,
}

/// Published information from/to objects
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishedPropFromTo {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}
