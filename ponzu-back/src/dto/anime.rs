use crate::models::anime::{Aired, AiredProp, AiredPropFromTo, Anime, Broadcast};
use crate::models::genre::Genre;
use crate::models::producer::Producer;
use crate::types::links::{ExternalLink, Images, Trailer};
use crate::types::title_meta::{MalEntity, Relation, Theme, Title};
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_bson_datetime_as_rfc3339_string,
    serialize_option_hex_string_as_object_id,
};
use mongodb::bson::{doc, to_bson, DateTime, Document};
use mongodb::options::UpdateModifications;
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

impl From<UpdateAnimeDto> for UpdateModifications {
    fn from(dto: UpdateAnimeDto) -> Self {
        let mut doc = Document::new();

        if let Some(mal_id) = dto.mal_id {
            doc.insert(
                "mal_id",
                to_bson(&mal_id).expect("Failed to convert mal_id to bson"),
            );
        }
        if let Some(images) = dto.images {
            doc.insert(
                "images",
                to_bson(&images).expect("Failed to convert images to bson"),
            );
        }
        if let Some(trailer) = dto.trailer {
            doc.insert(
                "trailer",
                to_bson(&trailer).expect("Failed to convert trailer to bson"),
            );
        }
        if let Some(approved) = dto.approved {
            doc.insert(
                "approved",
                to_bson(&approved).expect("Failed to convert approved to bson"),
            );
        }
        if let Some(titles) = dto.titles {
            doc.insert(
                "titles",
                to_bson(&titles).expect("Failed to convert titles to bson"),
            );
        }
        if let Some(title) = dto.title {
            doc.insert(
                "title",
                to_bson(&title).expect("Failed to convert title to bson"),
            );
        }
        if let Some(title_english) = dto.title_english {
            doc.insert(
                "title_english",
                to_bson(&title_english).expect("Failed to convert title_english to bson"),
            );
        }
        if let Some(title_japanese) = dto.title_japanese {
            doc.insert(
                "title_japanese",
                to_bson(&title_japanese).expect("Failed to convert title_japanese to bson"),
            );
        }
        if let Some(title_synonyms) = dto.title_synonyms {
            doc.insert(
                "title_synonyms",
                to_bson(&title_synonyms).expect("Failed to convert title_synonyms to bson"),
            );
        }
        if let Some(r#type) = dto.r#type {
            doc.insert(
                "type",
                to_bson(&r#type).expect("Failed to convert type to bson"),
            );
        }
        if let Some(source) = dto.source {
            doc.insert(
                "source",
                to_bson(&source).expect("Failed to convert source to bson"),
            );
        }
        if let Some(episodes) = dto.episodes {
            doc.insert(
                "episodes",
                to_bson(&episodes).expect("Failed to convert episodes to bson"),
            );
        }
        if let Some(status) = dto.status {
            doc.insert(
                "status",
                to_bson(&status).expect("Failed to convert status to bson"),
            );
        }
        if let Some(airing) = dto.airing {
            doc.insert(
                "airing",
                to_bson(&airing).expect("Failed to convert airing to bson"),
            );
        }
        if let Some(aired) = dto.aired {
            doc.insert(
                "aired",
                to_bson(&aired).expect("Failed to convert aired to bson"),
            );
        }
        if let Some(duration) = dto.duration {
            doc.insert(
                "duration",
                to_bson(&duration).expect("Failed to convert duration to bson"),
            );
        }
        if let Some(rating) = dto.rating {
            doc.insert(
                "rating",
                to_bson(&rating).expect("Failed to convert rating to bson"),
            );
        }
        if let Some(scored_by) = dto.scored_by {
            doc.insert(
                "scored_by",
                to_bson(&scored_by).expect("Failed to convert scored_by to bson"),
            );
        }
        if let Some(members) = dto.members {
            doc.insert(
                "members",
                to_bson(&members).expect("Failed to convert members to bson"),
            );
        }
        if let Some(favorites) = dto.favorites {
            doc.insert(
                "favorites",
                to_bson(&favorites).expect("Failed to convert favorites to bson"),
            );
        }
        if let Some(synopsis) = dto.synopsis {
            doc.insert(
                "synopsis",
                to_bson(&synopsis).expect("Failed to convert synopsis to bson"),
            );
        }
        if let Some(background) = dto.background {
            doc.insert(
                "background",
                to_bson(&background).expect("Failed to convert background to bson"),
            );
        }
        if let Some(season) = dto.season {
            doc.insert(
                "season",
                to_bson(&season).expect("Failed to convert season to bson"),
            );
        }
        if let Some(year) = dto.year {
            doc.insert(
                "year",
                to_bson(&year).expect("Failed to convert year to bson"),
            );
        }
        if let Some(broadcast) = dto.broadcast {
            doc.insert(
                "broadcast",
                to_bson(&broadcast).expect("Failed to convert broadcast to bson"),
            );
        }
        if let Some(producers) = dto.producers {
            doc.insert(
                "producers",
                to_bson(&producers).expect("Failed to convert producers to bson"),
            );
        }
        if let Some(licensors) = dto.licensors {
            doc.insert(
                "licensors",
                to_bson(&licensors).expect("Failed to convert licensors to bson"),
            );
        }
        if let Some(studios) = dto.studios {
            doc.insert(
                "studios",
                to_bson(&studios).expect("Failed to convert studios to bson"),
            );
        }
        if let Some(genres) = dto.genres {
            doc.insert(
                "genres",
                to_bson(&genres).expect("Failed to convert genres to bson"),
            );
        }
        if let Some(explicit_genres) = dto.explicit_genres {
            doc.insert(
                "explicit_genres",
                to_bson(&explicit_genres).expect("Failed to convert explicit_genres to bson"),
            );
        }
        if let Some(themes) = dto.themes {
            doc.insert(
                "themes",
                to_bson(&themes).expect("Failed to convert themes to bson"),
            );
        }
        if let Some(demographics) = dto.demographics {
            doc.insert(
                "demographics",
                to_bson(&demographics).expect("Failed to convert demographics to bson"),
            );
        }
        if let Some(relations) = dto.relations {
            doc.insert(
                "relations",
                to_bson(&relations).expect("Failed to convert relations to bson"),
            );
        }
        if let Some(theme) = dto.theme {
            doc.insert(
                "theme",
                to_bson(&theme).expect("Failed to convert theme to bson"),
            );
        }
        if let Some(external) = dto.external {
            doc.insert(
                "external",
                to_bson(&external).expect("Failed to convert external to bson"),
            );
        }
        if let Some(streaming) = dto.streaming {
            doc.insert(
                "streaming",
                to_bson(&streaming).expect("Failed to convert streaming to bson"),
            );
        }

        UpdateModifications::Document(doc)
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
