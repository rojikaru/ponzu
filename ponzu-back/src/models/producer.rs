/*
from typing import Annotated

from beanie import Document, Indexed


class Producer(Document):
    mal_id: Annotated[int, Indexed(unique=True)]
    name: str
    type: str
    url: str

    class Settings:
        name: str = "producer"

    def __str__(self):
        return f'{self.name} ({self.type}, {self.url})'

 */

use mongodm::{field, CollectionConfig, Index, Indexes, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Producer {
    pub mal_id: i32,
    pub name: String,
    pub r#type: String,
    pub url: String,
}

pub struct ProducerCollConf;

impl Producer {
    pub fn new(mal_id: i32, name: String, r#type: String, url: String) -> Self {
        Self {
            mal_id,
            name,
            r#type,
            url,
        }
    }
}

impl Model for Producer {
    type CollConf = ProducerCollConf;
}

impl CollectionConfig for ProducerCollConf {
    fn collection_name() -> &'static str {
        "producer"
    }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new(field!(mal_id in Producer)))
            .with(Index::new(field!(name in Producer)))
    }
}
