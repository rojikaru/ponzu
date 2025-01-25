use mongodm::{field, CollectionConfig, Index, Indexes, Model};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub image: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UserCollConf;

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        is_active: bool,
        is_staff: bool,
        is_superuser: bool,
        image: Option<String>,
        bio: Option<String>,
        birth_date: Option<chrono::DateTime<chrono::Utc>>,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            username,
            email,
            password,
            is_active,
            is_staff,
            is_superuser,
            image,
            bio,
            birth_date,
            created_at,
            updated_at,
        }
    }
}

impl Model for User {
    type CollConf = UserCollConf;
}

impl CollectionConfig for UserCollConf {
    fn collection_name() -> &'static str {
        "users"
    }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new(field!(username in User)))
            .with(Index::new(field!(email in User)))
    }
}
