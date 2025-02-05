use crate::models::anime::{Aired, AiredProp, AiredPropFromTo, Anime, Broadcast};
use crate::models::genre::Genre;
use crate::models::producer::Producer;
use crate::types::links::{ExternalLink, Images, Trailer};
use crate::types::title_meta::{MalEntity, Relation, Theme, Title};
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_bson_datetime_as_rfc3339_string,
    serialize_option_hex_string_as_object_id,
};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeDto {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub images: Images,
    pub trailer: Trailer,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub source: String,
    pub episodes: Option<u32>,
    pub status: String,
    pub airing: bool,
    pub aired: Option<AiredDto>,
    pub duration: String,
    pub rating: String,
    pub scored_by: u64,
    pub members: u64,
    pub favorites: u64,
    pub synopsis: String,
    pub background: String,
    pub season: String,
    pub year: Option<u32>,
    pub broadcast: Option<BroadcastDto>,
    pub producers: Vec<Producer>,
    pub licensors: Vec<MalEntity>,
    pub studios: Vec<MalEntity>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<MalEntity>,
    pub themes: Vec<MalEntity>,
    pub demographics: Vec<MalEntity>,
    pub relations: Vec<Relation>,
    pub theme: Option<Theme>,
    pub external: Vec<ExternalLink>,
    pub streaming: Vec<ExternalLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BroadcastDto {
    pub day: String,
    pub time: String,
    pub timezone: String,
    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiredDto {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub from: Option<DateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub to: Option<DateTime>,
    pub prop: AiredPropDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiredPropDto {
    pub from: AiredPropFromToDto,
    pub to: AiredPropFromToDto,
    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiredPropFromToDto {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAnimeDto {
    pub mal_id: u64,
    pub images: Images,
    pub trailer: Trailer,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub source: String,
    pub episodes: Option<u32>,
    pub status: String,
    pub airing: bool,
    pub aired: Option<AiredDto>,
    pub duration: String,
    pub rating: String,
    pub scored_by: u64,
    pub members: u64,
    pub favorites: u64,
    pub synopsis: String,
    pub background: String,
    pub season: String,
    pub year: Option<u32>,
    pub broadcast: Option<BroadcastDto>,
    pub producers: Vec<Producer>,
    pub licensors: Vec<MalEntity>,
    pub studios: Vec<MalEntity>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<MalEntity>,
    pub themes: Vec<MalEntity>,
    pub demographics: Vec<MalEntity>,
    pub relations: Vec<Relation>,
    pub theme: Option<Theme>,
    pub external: Vec<ExternalLink>,
    pub streaming: Vec<ExternalLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAnimeDto {
    pub mal_id: Option<u64>,
    pub images: Option<Images>,
    pub trailer: Option<Trailer>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<AiredDto>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub scored_by: Option<u64>,
    pub members: Option<u64>,
    pub favorites: Option<u64>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Option<BroadcastDto>,
    pub producers: Option<Vec<Producer>>,
    pub licensors: Option<Vec<MalEntity>>,
    pub studios: Option<Vec<MalEntity>>,
    pub genres: Option<Vec<Genre>>,
    pub explicit_genres: Option<Vec<MalEntity>>,
    pub themes: Option<Vec<MalEntity>>,
    pub demographics: Option<Vec<MalEntity>>,
    pub relations: Option<Vec<Relation>>,
    pub theme: Option<Theme>,
    pub external: Option<Vec<ExternalLink>>,
    pub streaming: Option<Vec<ExternalLink>>,
}

impl From<Anime> for AnimeDto {
    fn from(anime: Anime) -> Self {
        Self {
            id: anime.id,
            mal_id: anime.mal_id,
            images: anime.images,
            trailer: anime.trailer,
            approved: anime.approved,
            titles: anime.titles,
            title: anime.title,
            title_english: anime.title_english,
            title_japanese: anime.title_japanese,
            title_synonyms: anime.title_synonyms,
            r#type: anime.r#type,
            source: anime.source,
            episodes: anime.episodes,
            status: anime.status,
            airing: anime.airing,
            aired: anime.aired.map(Into::into),
            duration: anime.duration,
            rating: anime.rating,
            scored_by: anime.scored_by,
            members: anime.members,
            favorites: anime.favorites,
            synopsis: anime.synopsis,
            background: anime.background,
            season: anime.season,
            year: anime.year,
            broadcast: anime.broadcast.map(Into::into),
            producers: anime.producers,
            licensors: anime.licensors,
            studios: anime.studios,
            genres: anime.genres,
            explicit_genres: anime.explicit_genres,
            themes: anime.themes,
            demographics: anime.demographics,
            relations: anime.relations,
            theme: anime.theme,
            external: anime.external,
            streaming: anime.streaming,
        }
    }
}

impl From<CreateAnimeDto> for Anime {
    fn from(dto: CreateAnimeDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            images: dto.images,
            trailer: dto.trailer,
            approved: dto.approved,
            titles: dto.titles,
            title: dto.title,
            title_english: dto.title_english,
            title_japanese: dto.title_japanese,
            title_synonyms: dto.title_synonyms,
            r#type: dto.r#type,
            source: dto.source,
            episodes: dto.episodes,
            status: dto.status,
            airing: dto.airing,
            aired: dto.aired.map(Into::into),
            duration: dto.duration,
            rating: dto.rating,
            scored_by: dto.scored_by,
            members: dto.members,
            favorites: dto.favorites,
            synopsis: dto.synopsis,
            background: dto.background,
            season: dto.season,
            year: dto.year,
            broadcast: dto.broadcast.map(Into::into),
            producers: dto.producers,
            licensors: dto.licensors,
            studios: dto.studios,
            genres: dto.genres,
            explicit_genres: dto.explicit_genres,
            themes: dto.themes,
            demographics: dto.demographics,
            relations: dto.relations,
            theme: dto.theme,
            external: dto.external,
            streaming: dto.streaming,
        }
    }
}

impl From<Broadcast> for BroadcastDto {
    fn from(broadcast: Broadcast) -> Self {
        Self {
            day: broadcast.day,
            time: broadcast.time,
            timezone: broadcast.timezone,
            string: broadcast.string,
        }
    }
}

impl From<BroadcastDto> for Broadcast {
    fn from(dto: BroadcastDto) -> Self {
        Self {
            day: dto.day,
            time: dto.time,
            timezone: dto.timezone,
            string: dto.string,
        }
    }
}

impl From<Aired> for AiredDto {
    fn from(aired: Aired) -> Self {
        Self {
            from: aired.from,
            to: aired.to,
            prop: aired.prop.into(),
        }
    }
}

impl From<AiredDto> for Aired {
    fn from(dto: AiredDto) -> Self {
        Self {
            from: dto.from,
            to: dto.to,
            prop: dto.prop.into(),
        }
    }
}

impl From<AiredProp> for AiredPropDto {
    fn from(prop: AiredProp) -> Self {
        Self {
            from: prop.from.into(),
            to: prop.to.into(),
            string: prop.string,
        }
    }
}

impl From<AiredPropDto> for AiredProp {
    fn from(dto: AiredPropDto) -> Self {
        Self {
            from: dto.from.into(),
            to: dto.to.into(),
            string: dto.string,
        }
    }
}

impl From<AiredPropFromTo> for AiredPropFromToDto {
    fn from(prop: AiredPropFromTo) -> Self {
        Self {
            day: prop.day,
            month: prop.month,
            year: prop.year,
        }
    }
}

impl From<AiredPropFromToDto> for AiredPropFromTo {
    fn from(dto: AiredPropFromToDto) -> Self {
        Self {
            day: dto.day,
            month: dto.month,
            year: dto.year,
        }
    }
}
