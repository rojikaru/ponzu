use crate::models::club::Club;
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClubDto {
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
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateClubDto {
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<String>,
    pub access: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateClubDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub members: Option<Vec<String>>,
    pub access: Option<String>,
    pub category: Option<String>,
}

impl From<Club> for ClubDto {
    fn from(club: Club) -> Self {
        Self {
            id: club.id,
            name: club.name,
            description: club.description,
            members: club.members,
            access: club.access,
            category: club.category,
            created_at: club.created_at,
            updated_at: club.updated_at,
        }
    }
}

impl From<CreateClubDto> for Club {
    fn from(dto: CreateClubDto) -> Self {
        let now = DateTime::now();
        Self {
            id: None,
            name: dto.name,
            description: dto.description,
            members: dto.members,
            access: dto.access,
            category: dto.category,
            created_at: now,
            updated_at: now,
        }
    }
}
