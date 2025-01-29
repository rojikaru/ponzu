use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_bson_datetime_as_rfc3339_string,
    serialize_option_hex_string_as_object_id,
};
use mongodb::bson::serde_helpers::{
    deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string,
};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// User model
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub images: Option<String>,
    pub bio: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub birth_date: Option<DateTime>,
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
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string"
    )]
    pub last_online: DateTime,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
        images: Option<String>,
        bio: Option<String>,
        birth_date: Option<DateTime>,
    ) -> Self {
        Self {
            id: None,
            username,
            email,
            password,
            is_active,
            is_staff,
            is_superuser,
            images,
            bio,
            birth_date,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            last_online: DateTime::now(),
        }
    }
}
