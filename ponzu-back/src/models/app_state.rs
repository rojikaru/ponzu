use crate::models::genre::Genre;
use crate::models::producer::Producer;
use crate::models::review::{AnimeReview, MangaReview};
use crate::models::title::{Anime, Manga};
use crate::models::user::User;
use crate::services::db_repo::DatabaseRepository;
use mongodb::Database;

/// The application state.
pub struct AppState {
    /// The CRUD service for the `Genre` model.
    pub genre_service: DatabaseRepository<Genre>,
    /// The CRUD service for the `Producer` model.
    pub producer_service: DatabaseRepository<Producer>,
    /// The CRUD service for the `Anime` model.
    pub anime_service: DatabaseRepository<Anime>,
    /// The CRUD service for the `Manga` model.
    pub manga_service: DatabaseRepository<Manga>,
    /// The CRUD service for the `User` model.
    pub user_service: DatabaseRepository<User>,
    /// The CRUD service for the `AnimeReview` model.
    pub anime_review_service: DatabaseRepository<AnimeReview>,
    /// The CRUD service for the `MangaReview` model.
    pub manga_review_service: DatabaseRepository<MangaReview>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        AppState {
            genre_service: DatabaseRepository::new(db.collection("genres")),
            producer_service: DatabaseRepository::new(db.collection("producers")),
            anime_service: DatabaseRepository::new(db.collection("anime")),
            manga_service: DatabaseRepository::new(db.collection("manga")),
            user_service: DatabaseRepository::new(db.collection("users")),
            anime_review_service: DatabaseRepository::new(db.collection("anime_reviews")),
            manga_review_service: DatabaseRepository::new(db.collection("manga_reviews")),
        }
    }
}
