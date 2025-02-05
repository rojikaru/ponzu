use crate::models::user::User;
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_bson_datetime_as_rfc3339_string,
    serialize_option_hex_string_as_object_id,
};
use mongodb::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use mongodb::bson::{to_bson, Document};
use mongodb::options::UpdateModifications;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDto {
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
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub updated_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub last_online: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub images: Option<String>,
    pub bio: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub birth_date: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub images: Option<String>,
    pub bio: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub birth_date: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            is_active: user.is_active,
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
            images: user.images,
            bio: user.bio,
            birth_date: user.birth_date,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_online: user.last_online,
        }
    }
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        Self {
            id: dto.id,
            username: dto.username,
            email: dto.email,
            password: dto.password,
            is_active: dto.is_active,
            is_staff: dto.is_staff,
            is_superuser: dto.is_superuser,
            images: dto.images,
            bio: dto.bio,
            birth_date: dto.birth_date,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            last_online: dto.last_online,
        }
    }
}

impl From<RegisterUserDto> for User {
    fn from(dto: RegisterUserDto) -> Self {
        Self {
            id: None,
            username: dto.username,
            email: dto.email,
            password: dto.password,
            is_active: false,
            is_staff: false,
            is_superuser: false,
            images: dto.images,
            bio: dto.bio,
            birth_date: dto.birth_date,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            last_online: DateTime::now(),
        }
    }
}

impl From<UpdateUserDto> for UpdateModifications {
    fn from(dto: UpdateUserDto) -> Self {
        let mut doc = Document::new();

        if let Some(username) = dto.username {
            doc.insert(
                "username",
                to_bson(&username).expect("Failed to convert username to bson"),
            );
        }
        if let Some(email) = dto.email {
            doc.insert(
                "email",
                to_bson(&email).expect("Failed to convert email to bson"),
            );
        }
        if let Some(password) = dto.password {
            doc.insert(
                "password",
                to_bson(&password).expect("Failed to convert password to bson"),
            );
        }
        if let Some(is_active) = dto.is_active {
            doc.insert(
                "is_active",
                to_bson(&is_active).expect("Failed to convert is_active to bson"),
            );
        }
        if let Some(is_staff) = dto.is_staff {
            doc.insert(
                "is_staff",
                to_bson(&is_staff).expect("Failed to convert is_staff to bson"),
            );
        }
        if let Some(is_superuser) = dto.is_superuser {
            doc.insert(
                "is_superuser",
                to_bson(&is_superuser).expect("Failed to convert is_superuser to bson"),
            );
        }
        if let Some(images) = dto.images {
            doc.insert(
                "images",
                to_bson(&images).expect("Failed to convert images to bson"),
            );
        }
        if let Some(bio) = dto.bio {
            doc.insert("bio", to_bson(&bio).expect("Failed to convert bio to bson"));
        }
        if let Some(birth_date) = dto.birth_date {
            doc.insert(
                "birth_date",
                to_bson(&birth_date).expect("Failed to convert birth_date to bson"),
            );
        }

        UpdateModifications::Document(doc)
    }
}
