use crate::types::links::Images;
use serde::{Deserialize, Serialize};

/// A simplified character object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterReference {
    pub mal_id: u64,
    pub url: String,
    pub images: Images,
    pub name: String,
}
