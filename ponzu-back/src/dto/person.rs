use crate::models::person::{Person, PersonMedia, PersonVoice};
use crate::types::links::Images;
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
pub struct PersonDto {
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
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub birthday: DateTime,
    pub favorites: u64,
    pub about: String,
    pub anime: Vec<PersonMediaDto>,
    pub manga: Vec<PersonMediaDto>,
    pub voices: Vec<PersonVoiceDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonMediaDto {
    pub position: String,
    pub media: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonVoiceDto {
    pub role: String,
    pub anime: String,
    pub character: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePersonDto {
    pub mal_id: u64,
    pub url: String,
    pub website_url: String,
    pub images: Images,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub alternate_names: Vec<String>,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub birthday: DateTime,
    pub favorites: u64,
    pub about: String,
    pub anime: Vec<PersonMediaDto>,
    pub manga: Vec<PersonMediaDto>,
    pub voices: Vec<PersonVoiceDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePersonDto {
    pub mal_id: Option<u64>,
    pub url: Option<String>,
    pub website_url: Option<String>,
    pub images: Option<Images>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub birthday: Option<DateTime>,
    pub favorites: Option<u64>,
    pub about: Option<String>,
    pub anime: Option<Vec<PersonMediaDto>>,
    pub manga: Option<Vec<PersonMediaDto>>,
    pub voices: Option<Vec<PersonVoiceDto>>,
}

impl From<Person> for PersonDto {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            mal_id: person.mal_id,
            url: person.url,
            website_url: person.website_url,
            images: person.images,
            name: person.name,
            given_name: person.given_name,
            family_name: person.family_name,
            alternate_names: person.alternate_names,
            birthday: person.birthday,
            favorites: person.favorites,
            about: person.about,
            anime: person.anime.into_iter().map(Into::into).collect(),
            manga: person.manga.into_iter().map(Into::into).collect(),
            voices: person.voices.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CreatePersonDto> for Person {
    fn from(dto: CreatePersonDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            url: dto.url,
            website_url: dto.website_url,
            images: dto.images,
            name: dto.name,
            given_name: dto.given_name,
            family_name: dto.family_name,
            alternate_names: dto.alternate_names,
            birthday: dto.birthday,
            favorites: dto.favorites,
            about: dto.about,
            anime: dto.anime.into_iter().map(Into::into).collect(),
            manga: dto.manga.into_iter().map(Into::into).collect(),
            voices: dto.voices.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<UpdatePersonDto> for UpdateModifications {
    fn from(dto: UpdatePersonDto) -> Self {
        let mut doc = Document::new();

        if let Some(mal_id) = dto.mal_id {
            doc.insert(
                "mal_id",
                to_bson(&mal_id).expect("Failed to convert mal_id to bson"),
            );
        }
        if let Some(url) = dto.url {
            doc.insert("url", to_bson(&url).expect("Failed to convert url to bson"));
        }
        if let Some(website_url) = dto.website_url {
            doc.insert(
                "website_url",
                to_bson(&website_url).expect("Failed to convert website_url to bson"),
            );
        }
        if let Some(images) = dto.images {
            doc.insert(
                "images",
                to_bson(&images).expect("Failed to convert images to bson"),
            );
        }
        if let Some(name) = dto.name {
            doc.insert(
                "name",
                to_bson(&name).expect("Failed to convert name to bson"),
            );
        }
        if let Some(given_name) = dto.given_name {
            doc.insert(
                "given_name",
                to_bson(&given_name).expect("Failed to convert given_name to bson"),
            );
        }
        if let Some(family_name) = dto.family_name {
            doc.insert(
                "family_name",
                to_bson(&family_name).expect("Failed to convert family_name to bson"),
            );
        }
        if let Some(alternate_names) = dto.alternate_names {
            doc.insert(
                "alternate_names",
                to_bson(&alternate_names).expect("Failed to convert alternate_names to bson"),
            );
        }
        if let Some(birthday) = dto.birthday {
            doc.insert(
                "birthday",
                to_bson(&birthday).expect("Failed to convert birthday to bson"),
            );
        }
        if let Some(favorites) = dto.favorites {
            doc.insert(
                "favorites",
                to_bson(&favorites).expect("Failed to convert favorites to bson"),
            );
        }
        if let Some(about) = dto.about {
            doc.insert(
                "about",
                to_bson(&about).expect("Failed to convert about to bson"),
            );
        }
        if let Some(anime) = dto.anime {
            doc.insert(
                "anime",
                to_bson(&anime).expect("Failed to convert anime to bson"),
            );
        }
        if let Some(manga) = dto.manga {
            doc.insert(
                "manga",
                to_bson(&manga).expect("Failed to convert manga to bson"),
            );
        }
        if let Some(voices) = dto.voices {
            doc.insert(
                "voices",
                to_bson(&voices).expect("Failed to convert voices to bson"),
            );
        }

        UpdateModifications::Document(doc)
    }
}

impl From<PersonMedia> for PersonMediaDto {
    fn from(media: PersonMedia) -> Self {
        Self {
            position: media.position,
            media: media.media,
        }
    }
}

impl From<PersonMediaDto> for PersonMedia {
    fn from(dto: PersonMediaDto) -> Self {
        Self {
            position: dto.position,
            media: dto.media,
        }
    }
}

impl From<PersonVoice> for PersonVoiceDto {
    fn from(voice: PersonVoice) -> Self {
        Self {
            role: voice.role,
            anime: voice.anime,
            character: voice.character,
        }
    }
}

impl From<PersonVoiceDto> for PersonVoice {
    fn from(dto: PersonVoiceDto) -> Self {
        Self {
            role: dto.role,
            anime: dto.anime,
            character: dto.character,
        }
    }
}
