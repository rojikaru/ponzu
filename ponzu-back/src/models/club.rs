use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use mongodb::bson::serde_helpers::{
    deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string,
};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// Club model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Club {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<String>,
    pub access: String,
    pub category: String,
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
