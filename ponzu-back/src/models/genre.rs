use mongodm::{field, CollectionConfig, Index, Indexes, Model};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Genre {
    pub mal_id: i32,
    pub name: String,
    pub r#type: String,
}

pub struct GenreCollConf;

impl Genre {
    pub fn new(mal_id: i32, name: String, r#type: String) -> Self {
        Self {
            mal_id,
            name,
            r#type,
        }
    }
}

impl Model for Genre {
    type CollConf = GenreCollConf;
}

impl CollectionConfig for GenreCollConf {
    fn collection_name() -> &'static str {
        "genre"
    }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new(field!(mal_id in Genre)))
            .with(Index::new(field!(name in Genre)))
    }
}
