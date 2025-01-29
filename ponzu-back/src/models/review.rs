use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

/// Review model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,

    pub mal_id: u64,
    pub url: String,
    pub r#type: String,
    pub reactions: Reactions,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string"
    )]
    pub date: DateTime,

    pub review: String,
    pub score: u8,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes_watched: Option<u64>,

    pub entry: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reactions {
    pub overall: u64,
    pub nice: u64,
    pub love_it: u64,
    pub funny: u64,
    pub confusing: u64,
    pub informative: u64,
    pub well_written: u64,
    pub creative: u64,
}
