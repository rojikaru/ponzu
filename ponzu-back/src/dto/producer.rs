use crate::models::producer::Producer;
use crate::types::links::{ExternalLink, Images};
use crate::types::title_meta::MalEntity;
use mongodb::bson::{to_bson, Document};
use mongodb::options::UpdateModifications;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProducerDto {
    pub mal_id: u64,
    pub titles: Vec<MalEntity>,
    pub images: Option<Images>,
    pub favorites: u64,
    pub count: u64,
    pub established: String,
    pub about: String,
    pub external: Vec<ExternalLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProducerDto {
    pub mal_id: u64,
    pub titles: Vec<MalEntity>,
    pub images: Option<Images>,
    pub favorites: u64,
    pub count: u64,
    pub established: String,
    pub about: String,
    pub external: Vec<ExternalLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProducerDto {
    pub mal_id: Option<u64>,
    pub titles: Option<Vec<MalEntity>>,
    pub images: Option<Images>,
    pub favorites: Option<u64>,
    pub count: Option<u64>,
    pub established: Option<String>,
    pub about: Option<String>,
    pub external: Option<Vec<ExternalLink>>,
}

impl From<Producer> for ProducerDto {
    fn from(producer: Producer) -> Self {
        Self {
            mal_id: producer.mal_id,
            titles: producer.titles,
            images: producer.images,
            favorites: producer.favorites,
            count: producer.count,
            established: producer.established,
            about: producer.about,
            external: producer.external,
        }
    }
}

impl From<ProducerDto> for Producer {
    fn from(dto: ProducerDto) -> Self {
        Self {
            mal_id: dto.mal_id,
            titles: dto.titles,
            images: dto.images,
            favorites: dto.favorites,
            count: dto.count,
            established: dto.established,
            about: dto.about,
            external: dto.external,
        }
    }
}

impl From<CreateProducerDto> for Producer {
    fn from(dto: CreateProducerDto) -> Self {
        Self {
            mal_id: dto.mal_id,
            titles: dto.titles,
            images: dto.images,
            favorites: dto.favorites,
            count: dto.count,
            established: dto.established,
            about: dto.about,
            external: dto.external,
        }
    }
}

impl From<UpdateProducerDto> for UpdateModifications {
    fn from(dto: UpdateProducerDto) -> Self {
        let mut doc = Document::new();

        if let Some(mal_id) = dto.mal_id {
            doc.insert(
                "mal_id",
                to_bson(&mal_id).expect("Failed to convert mal_id to bson"),
            );
        }
        if let Some(titles) = dto.titles {
            doc.insert(
                "titles",
                to_bson(&titles).expect("Failed to convert titles to bson"),
            );
        }
        if let Some(images) = dto.images {
            doc.insert(
                "images",
                to_bson(&images).expect("Failed to convert images to bson"),
            );
        }
        if let Some(favorites) = dto.favorites {
            doc.insert(
                "favorites",
                to_bson(&favorites).expect("Failed to convert favorites to bson"),
            );
        }
        if let Some(count) = dto.count {
            doc.insert(
                "count",
                to_bson(&count).expect("Failed to convert count to bson"),
            );
        }
        if let Some(established) = dto.established {
            doc.insert(
                "established",
                to_bson(&established).expect("Failed to convert established to bson"),
            );
        }
        if let Some(about) = dto.about {
            doc.insert(
                "about",
                to_bson(&about).expect("Failed to convert about to bson"),
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
