use crate::models::links::{ExternalLink, Images, Trailer};
use crate::models::title_meta::{MalEntity, Relation, Theme, Title};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// Anime model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Anime {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub url: String,
    pub images: Images,
    pub trailer: Trailer,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub source: String,
    pub episodes: Option<u32>,
    pub status: String,
    pub airing: bool,
    pub aired: Option<Aired>,
    pub duration: String,
    pub rating: String,
    pub synopsis: String,
    pub background: String,
    pub season: String,
    pub year: Option<i32>,
    pub broadcast: Option<Broadcast>,
    pub producers: Vec<MalEntity>,
    pub licensors: Vec<MalEntity>,
    pub studios: Vec<MalEntity>,
    pub genres: Vec<MalEntity>,
    pub explicit_genres: Vec<MalEntity>,
    pub themes: Vec<MalEntity>,
    pub demographics: Vec<MalEntity>,
    pub relations: Vec<Relation>,
    pub theme: Option<Theme>,
    pub external: Vec<ExternalLink>,
    pub streaming: Vec<ExternalLink>,
}

/// Broadcast information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Broadcast {
    pub day: String,
    pub time: String,
    pub timezone: String,
    pub string: String,
}

/// Aired information object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aired {
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
    pub prop: AiredProp,
}

/// Aired information properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiredProp {
    pub from: AiredPropFromTo,
    pub to: AiredPropFromTo,
    pub string: String,
}

/// Aired information from/to objects
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiredPropFromTo {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}
