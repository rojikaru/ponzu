use crate::models::magazine::Magazine;
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MagazineDto {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMagazineDto {
    pub mal_id: u64,
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMagazineDto {
    pub mal_id: Option<u64>,
    pub name: Option<String>,
    pub count: Option<u64>,
}

impl From<Magazine> for MagazineDto {
    fn from(magazine: Magazine) -> Self {
        Self {
            id: magazine.id,
            mal_id: magazine.mal_id,
            name: magazine.name,
            count: magazine.count,
        }
    }
}

impl From<CreateMagazineDto> for Magazine {
    fn from(dto: CreateMagazineDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            name: dto.name,
            count: dto.count,
        }
    }
}
