use crate::types::links::Images;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
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
    pub name: String,
    pub name_kanji: String,
    pub nicknames: Vec<String>,
    pub favorites: u64,
    pub about: String,
    pub anime: Vec<CharacterMedia>,
    pub manga: Vec<CharacterMedia>,
    pub voices: Vec<CharacterVoice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterMedia {
    pub role: String,
    pub media: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterVoice {
    pub language: String,
    pub person: String,
}
