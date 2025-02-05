use crate::models::character::{Character, CharacterMedia, CharacterVoice};
use crate::types::links::Images;
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};
/// A simplified character object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterReference {
    pub mal_id: u64,
    pub url: String,
    pub images: Images,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterDto {
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
    pub anime: Vec<CharacterMediaDto>,
    pub manga: Vec<CharacterMediaDto>,
    pub voices: Vec<CharacterVoiceDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterMediaDto {
    pub role: String,
    pub media: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterVoiceDto {
    pub language: String,
    pub person: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCharacterDto {
    pub mal_id: u64,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub name_kanji: String,
    pub nicknames: Vec<String>,
    pub favorites: u64,
    pub about: String,
    pub anime: Vec<CharacterMediaDto>,
    pub manga: Vec<CharacterMediaDto>,
    pub voices: Vec<CharacterVoiceDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCharacterDto {
    pub mal_id: Option<u64>,
    pub url: Option<String>,
    pub images: Option<Images>,
    pub name: Option<String>,
    pub name_kanji: Option<String>,
    pub nicknames: Option<Vec<String>>,
    pub favorites: Option<u64>,
    pub about: Option<String>,
    pub anime: Option<Vec<CharacterMediaDto>>,
    pub manga: Option<Vec<CharacterMediaDto>>,
    pub voices: Option<Vec<CharacterVoiceDto>>,
}

impl From<Character> for CharacterDto {
    fn from(character: Character) -> Self {
        Self {
            id: character.id,
            mal_id: character.mal_id,
            url: character.url,
            images: character.images,
            name: character.name,
            name_kanji: character.name_kanji,
            nicknames: character.nicknames,
            favorites: character.favorites,
            about: character.about,
            anime: character.anime.into_iter().map(Into::into).collect(),
            manga: character.manga.into_iter().map(Into::into).collect(),
            voices: character.voices.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CreateCharacterDto> for Character {
    fn from(dto: CreateCharacterDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            url: dto.url,
            images: dto.images,
            name: dto.name,
            name_kanji: dto.name_kanji,
            nicknames: dto.nicknames,
            favorites: dto.favorites,
            about: dto.about,
            anime: dto.anime.into_iter().map(Into::into).collect(),
            manga: dto.manga.into_iter().map(Into::into).collect(),
            voices: dto.voices.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CharacterMedia> for CharacterMediaDto {
    fn from(media: CharacterMedia) -> Self {
        Self {
            role: media.role,
            media: media.media,
        }
    }
}

impl From<CharacterMediaDto> for CharacterMedia {
    fn from(dto: CharacterMediaDto) -> Self {
        Self {
            role: dto.role,
            media: dto.media,
        }
    }
}

impl From<CharacterVoice> for CharacterVoiceDto {
    fn from(voice: CharacterVoice) -> Self {
        Self {
            language: voice.language,
            person: voice.person,
        }
    }
}

impl From<CharacterVoiceDto> for CharacterVoice {
    fn from(dto: CharacterVoiceDto) -> Self {
        Self {
            language: dto.language,
            person: dto.person,
        }
    }
}
