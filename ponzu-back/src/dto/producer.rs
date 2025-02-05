use crate::models::producer::Producer;
use crate::types::links::{ExternalLink, Images};
use crate::types::title_meta::MalEntity;
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
