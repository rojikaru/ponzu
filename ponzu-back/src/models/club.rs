use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// Club model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Club {
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
