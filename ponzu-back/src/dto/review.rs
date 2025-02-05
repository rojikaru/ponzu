use crate::models::review::{Reactions, Review};
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
pub struct ReviewDto {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_hex_string_as_object_id",
        deserialize_with = "deserialize_option_hex_string_from_object_id"
    )]
    pub id: Option<String>,
    pub mal_id: u64,
    pub url: String,
    pub r#type: String,
    pub reactions: ReactionsDto,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub review: String,
    pub score: u8,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes_watched: Option<u64>,
    pub entry: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionsDto {
    pub overall: u64,
    pub nice: u64,
    pub love_it: u64,
    pub funny: u64,
    pub confusing: u64,
    pub informative: u64,
    pub well_written: u64,
    pub creative: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateReviewDto {
    pub mal_id: u64,
    pub url: String,
    pub r#type: String,
    pub reactions: ReactionsDto,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub review: String,
    pub score: u8,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes_watched: Option<u64>,
    pub entry: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateReviewDto {
    pub mal_id: Option<u64>,
    pub url: Option<String>,
    pub r#type: Option<String>,
    pub reactions: Option<ReactionsDto>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_bson_datetime_as_rfc3339_string"
    )]
    pub date: Option<DateTime>,
    pub review: Option<String>,
    pub score: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub is_spoiler: Option<bool>,
    pub is_preliminary: Option<bool>,
    pub episodes_watched: Option<u64>,
    pub entry: Option<String>,
    pub user: Option<String>,
}

impl From<Review> for ReviewDto {
    fn from(review: Review) -> Self {
        Self {
            id: review.id,
            mal_id: review.mal_id,
            url: review.url,
            r#type: review.r#type,
            reactions: review.reactions.into(),
            date: review.date,
            review: review.review,
            score: review.score,
            tags: review.tags,
            is_spoiler: review.is_spoiler,
            is_preliminary: review.is_preliminary,
            episodes_watched: review.episodes_watched,
            entry: review.entry,
            user: review.user,
        }
    }
}

impl From<ReviewDto> for Review {
    fn from(dto: ReviewDto) -> Self {
        Self {
            id: dto.id,
            mal_id: dto.mal_id,
            url: dto.url,
            r#type: dto.r#type,
            reactions: dto.reactions.into(),
            date: dto.date,
            review: dto.review,
            score: dto.score,
            tags: dto.tags,
            is_spoiler: dto.is_spoiler,
            is_preliminary: dto.is_preliminary,
            episodes_watched: dto.episodes_watched,
            entry: dto.entry,
            user: dto.user,
        }
    }
}

impl From<CreateReviewDto> for Review {
    fn from(dto: CreateReviewDto) -> Self {
        Self {
            id: None,
            mal_id: dto.mal_id,
            url: dto.url,
            r#type: dto.r#type,
            reactions: dto.reactions.into(),
            date: dto.date,
            review: dto.review,
            score: dto.score,
            tags: dto.tags,
            is_spoiler: dto.is_spoiler,
            is_preliminary: dto.is_preliminary,
            episodes_watched: dto.episodes_watched,
            entry: dto.entry,
            user: dto.user,
        }
    }
}

impl From<UpdateReviewDto> for UpdateModifications {
    fn from(dto: UpdateReviewDto) -> Self {
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
        if let Some(r#type) = dto.r#type {
            doc.insert(
                "type",
                to_bson(&r#type).expect("Failed to convert type to bson"),
            );
        }
        if let Some(reactions) = dto.reactions {
            doc.insert(
                "reactions",
                to_bson(&reactions).expect("Failed to convert reactions to bson"),
            );
        }
        if let Some(date) = dto.date {
            doc.insert(
                "date",
                to_bson(&date).expect("Failed to convert date to bson"),
            );
        }
        if let Some(review) = dto.review {
            doc.insert(
                "review",
                to_bson(&review).expect("Failed to convert review to bson"),
            );
        }
        if let Some(score) = dto.score {
            doc.insert(
                "score",
                to_bson(&score).expect("Failed to convert score to bson"),
            );
        }
        if let Some(tags) = dto.tags {
            doc.insert(
                "tags",
                to_bson(&tags).expect("Failed to convert tags to bson"),
            );
        }
        if let Some(is_spoiler) = dto.is_spoiler {
            doc.insert(
                "is_spoiler",
                to_bson(&is_spoiler).expect("Failed to convert is_spoiler to bson"),
            );
        }
        if let Some(is_preliminary) = dto.is_preliminary {
            doc.insert(
                "is_preliminary",
                to_bson(&is_preliminary).expect("Failed to convert is_preliminary to bson"),
            );
        }
        if let Some(episodes_watched) = dto.episodes_watched {
            doc.insert(
                "episodes_watched",
                to_bson(&episodes_watched).expect("Failed to convert episodes_watched to bson"),
            );
        }
        if let Some(entry) = dto.entry {
            doc.insert(
                "entry",
                to_bson(&entry).expect("Failed to convert entry to bson"),
            );
        }
        if let Some(user) = dto.user {
            doc.insert(
                "user",
                to_bson(&user).expect("Failed to convert user to bson"),
            );
        }

        UpdateModifications::Document(doc)
    }
}

impl From<Reactions> for ReactionsDto {
    fn from(reactions: Reactions) -> Self {
        Self {
            overall: reactions.overall,
            nice: reactions.nice,
            love_it: reactions.love_it,
            funny: reactions.funny,
            confusing: reactions.confusing,
            informative: reactions.informative,
            well_written: reactions.well_written,
            creative: reactions.creative,
        }
    }
}

impl From<ReactionsDto> for Reactions {
    fn from(dto: ReactionsDto) -> Self {
        Self {
            overall: dto.overall,
            nice: dto.nice,
            love_it: dto.love_it,
            funny: dto.funny,
            confusing: dto.confusing,
            informative: dto.informative,
            well_written: dto.well_written,
            creative: dto.creative,
        }
    }
}
