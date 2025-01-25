use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: i32,
    pub name: String,
}

impl Genre {
    pub fn new(mal_id: i32, name: String) -> Self {
        Self {
            id: None,
            mal_id,
            name,
        }
    }
}
