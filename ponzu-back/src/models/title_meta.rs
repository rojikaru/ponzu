use serde::{Deserialize, Serialize};

/// Commonly-used types for title metadata with type, name, and URL fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MalEntity {
    pub mal_id: u64,
    pub r#type: String,
    pub name: String,
}

/// Title object with type and title fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Title {
    pub r#type: String,
    pub title: String,
}

/// Theme object representing openings and endings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

/// Relation object with relation and entry fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<MalEntity>,
}
