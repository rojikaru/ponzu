use serde::{Deserialize, Serialize};

/// Genre object with extra count field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub mal_id: u64,
    pub r#type: String,
    pub name: String,
    pub count: u64,
}
