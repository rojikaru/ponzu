use crate::models::genre::Genre;
use crate::models::manga::{Manga, Published, PublishedProp, PublishedPropFromTo};
use crate::types::links::{ExternalLink, Images};
use crate::types::title_meta::{MalEntity, Relation, Title};
use crate::utils::bson::{
    deserialize_option_hex_string_from_object_id, serialize_option_bson_datetime_as_rfc3339_string,
    serialize_option_hex_string_as_object_id,
};
use mongodb::bson::DateTime;
use mongodb::bson::{to_bson, Document};
use mongodb::options::UpdateModifications;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MangaDto {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: PublishedDto,
    pub scored_by: u64,
    pub members: u64,
    pub favorites: u64,
    pub synopsis: String,
    pub background: String,
    pub authors: Vec<MalEntity>,
    pub serializations: Vec<MalEntity>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<MalEntity>,
    pub themes: Vec<MalEntity>,
    pub demographics: Vec<MalEntity>,
    pub relations: Vec<Relation>,
    pub external: Vec<ExternalLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishedDto {
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
    pub prop: PublishedPropDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishedPropDto {
    pub from: PublishedPropFromToDto,
    pub to: PublishedPropFromToDto,
    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishedPropFromToDto {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateMangaDto {
    pub mal_id: u64,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: PublishedDto,
    pub scored_by: u64,
    pub members: u64,
    pub favorites: u64,
    pub synopsis: String,
    pub background: String,
    pub authors: Vec<MalEntity>,
    pub serializations: Vec<MalEntity>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<MalEntity>,
    pub themes: Vec<MalEntity>,
    pub demographics: Vec<MalEntity>,
    pub relations: Vec<Relation>,
    pub external: Vec<ExternalLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMangaDto {
    pub mal_id: Option<u64>,
    pub images: Option<Images>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: Option<String>,
    pub publishing: Option<bool>,
    pub published: Option<PublishedDto>,
    pub scored_by: Option<u64>,
    pub members: Option<u64>,
    pub favorites: Option<u64>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Option<Vec<MalEntity>>,
    pub serializations: Option<Vec<MalEntity>>,
    pub genres: Option<Vec<Genre>>,
    pub explicit_genres: Option<Vec<MalEntity>>,
    pub themes: Option<Vec<MalEntity>>,
    pub demographics: Option<Vec<MalEntity>>,
    pub relations: Option<Vec<Relation>>,
    pub external: Option<Vec<ExternalLink>>,
}

impl From<Manga> for MangaDto {
    fn from(manga: Manga) -> Self {
        Self {
            id: manga.id,
            mal_id: manga.mal_id,
            images: manga.images,
            approved: manga.approved,
            titles: manga.titles,
            title: manga.title,
            title_english: manga.title_english,
            title_japanese: manga.title_japanese,
            title_synonyms: manga.title_synonyms,
            r#type: manga.r#type,
            chapters: manga.chapters,
            volumes: manga.volumes,
            status: manga.status,
            publishing: manga.publishing,
            published: manga.published.into(),
            scored_by: manga.scored_by,
            members: manga.members,
            favorites: manga.favorites,
            synopsis: manga.synopsis,
            background: manga.background,
            authors: manga.authors,
            serializations: manga.serializations,
            genres: manga.genres,
            explicit_genres: manga.explicit_genres,
            themes: manga.themes,
            demographics: manga.demographics,
            relations: manga.relations,
            external: manga.external,
        }
    }
}

impl From<CreateMangaDto> for Manga {
    fn from(dto: CreateMangaDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            images: dto.images,
            approved: dto.approved,
            titles: dto.titles,
            title: dto.title,
            title_english: dto.title_english,
            title_japanese: dto.title_japanese,
            title_synonyms: dto.title_synonyms,
            r#type: dto.r#type,
            chapters: dto.chapters,
            volumes: dto.volumes,
            status: dto.status,
            publishing: dto.publishing,
            published: dto.published.into(),
            scored_by: dto.scored_by,
            members: dto.members,
            favorites: dto.favorites,
            synopsis: dto.synopsis,
            background: dto.background,
            authors: dto.authors,
            serializations: dto.serializations,
            genres: dto.genres,
            explicit_genres: dto.explicit_genres,
            themes: dto.themes,
            demographics: dto.demographics,
            relations: dto.relations,
            external: dto.external,
        }
    }
}

impl From<UpdateMangaDto> for UpdateModifications {
    fn from(dto: UpdateMangaDto) -> Self {
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
        if let Some(chapters) = dto.chapters {
            doc.insert(
                "chapters",
                to_bson(&chapters).expect("Failed to convert chapters to bson"),
            );
        }
        if let Some(volumes) = dto.volumes {
            doc.insert(
                "volumes",
                to_bson(&volumes).expect("Failed to convert volumes to bson"),
            );
        }
        if let Some(status) = dto.status {
            doc.insert(
                "status",
                to_bson(&status).expect("Failed to convert status to bson"),
            );
        }
        if let Some(publishing) = dto.publishing {
            doc.insert(
                "publishing",
                to_bson(&publishing).expect("Failed to convert publishing to bson"),
            );
        }
        if let Some(published) = dto.published {
            doc.insert(
                "published",
                to_bson(&published).expect("Failed to convert published to bson"),
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
        if let Some(authors) = dto.authors {
            doc.insert(
                "authors",
                to_bson(&authors).expect("Failed to convert authors to bson"),
            );
        }
        if let Some(serializations) = dto.serializations {
            doc.insert(
                "serializations",
                to_bson(&serializations).expect("Failed to convert serializations to bson"),
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
        if let Some(external) = dto.external {
            doc.insert(
                "external",
                to_bson(&external).expect("Failed to convert external to bson"),
            );
        }

        UpdateModifications::Document(doc)
    }
}

impl From<Published> for PublishedDto {
    fn from(published: Published) -> Self {
        Self {
            from: published.from,
            to: published.to,
            prop: published.prop.into(),
        }
    }
}

impl From<PublishedDto> for Published {
    fn from(dto: PublishedDto) -> Self {
        Self {
            from: dto.from,
            to: dto.to,
            prop: dto.prop.into(),
        }
    }
}

impl From<PublishedProp> for PublishedPropDto {
    fn from(prop: PublishedProp) -> Self {
        Self {
            from: prop.from.into(),
            to: prop.to.into(),
            string: prop.string,
        }
    }
}

impl From<PublishedPropDto> for PublishedProp {
    fn from(dto: PublishedPropDto) -> Self {
        Self {
            from: dto.from.into(),
            to: dto.to.into(),
            string: dto.string,
        }
    }
}

impl From<PublishedPropFromTo> for PublishedPropFromToDto {
    fn from(prop: PublishedPropFromTo) -> Self {
        Self {
            day: prop.day,
            month: prop.month,
            year: prop.year,
        }
    }
}

impl From<PublishedPropFromToDto> for PublishedPropFromTo {
    fn from(dto: PublishedPropFromToDto) -> Self {
        Self {
            day: dto.day,
            month: dto.month,
            year: dto.year,
        }
    }
}
