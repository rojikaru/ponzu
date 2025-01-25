use mongodm::{field, CollectionConfig, Index, Indexes, Model};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Demographic {
    pub mal_id: i32,
    pub name: String,
    pub r#type: String,
}

pub struct DemographicCollConf;

impl Demographic {
    pub fn new(mal_id: i32, name: String, r#type: String) -> Self {
        Self {
            mal_id,
            name,
            r#type,
        }
    }
}

impl Model for Demographic {
    type CollConf = DemographicCollConf;
}

impl CollectionConfig for DemographicCollConf {
    fn collection_name() -> &'static str {
        "demographic"
    }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new(field!(mal_id in Demographic)))
            .with(Index::new(field!(name in Demographic)))
    }
}
