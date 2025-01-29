use crate::types::links::Images;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub url: String,
    pub website_url: String,
    pub images: Images,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub alternate_names: Vec<String>,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string"
    )]
    pub birthday: DateTime,
    pub favorites: u64,
    pub about: String,
    pub anime: Vec<PersonMedia>,
    pub manga: Vec<PersonMedia>,
    pub voices: Vec<PersonVoice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonMedia {
    pub position: String,
    pub media: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonVoice {
    pub role: String,
    pub anime: String,
    pub character: String,
}
