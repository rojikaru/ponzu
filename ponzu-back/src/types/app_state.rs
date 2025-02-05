use crate::dto::anime::{AnimeDto, CreateAnimeDto, UpdateAnimeDto};
use crate::dto::character::{CharacterDto, CreateCharacterDto, UpdateCharacterDto};
use crate::dto::club::{ClubDto, CreateClubDto, UpdateClubDto};
use crate::dto::genre::{CreateGenreDto, GenreDto, UpdateGenreDto};
use crate::dto::magazine::{CreateMagazineDto, MagazineDto, UpdateMagazineDto};
use crate::dto::manga::{CreateMangaDto, MangaDto, UpdateMangaDto};
use crate::dto::person::{CreatePersonDto, PersonDto, UpdatePersonDto};
use crate::dto::producer::{CreateProducerDto, ProducerDto, UpdateProducerDto};
use crate::dto::review::{CreateReviewDto, ReviewDto, UpdateReviewDto};
use crate::dto::user::{RegisterUserDto, UpdateUserDto, UserDto};
use crate::models::anime::Anime;
use crate::models::character::Character;
use crate::models::club::Club;
use crate::models::genre::Genre;
use crate::models::magazine::Magazine;
use crate::models::manga::Manga;
use crate::models::person::Person;
use crate::models::producer::Producer;
use crate::models::review::Review;
use crate::models::user::User;
use crate::services::crud::{CrudService, CrudServiceImpl};
use crate::services::db_repo::DatabaseRepository;
use mongodb::Database;
use std::sync::Arc;

/// The application state.
pub struct AppState {
    anime_service: CrudServiceImpl<Anime, AnimeDto, CreateAnimeDto, UpdateAnimeDto>,
    character_service:
        CrudServiceImpl<Character, CharacterDto, CreateCharacterDto, UpdateCharacterDto>,
    club_service: CrudServiceImpl<Club, ClubDto, CreateClubDto, UpdateClubDto>,
    genre_service: CrudServiceImpl<Genre, GenreDto, CreateGenreDto, UpdateGenreDto>,
    magazine_service: CrudServiceImpl<Magazine, MagazineDto, CreateMagazineDto, UpdateMagazineDto>,
    manga_service: CrudServiceImpl<Manga, MangaDto, CreateMangaDto, UpdateMangaDto>,
    people_service: CrudServiceImpl<Person, PersonDto, CreatePersonDto, UpdatePersonDto>,
    producer_service: CrudServiceImpl<Producer, ProducerDto, CreateProducerDto, UpdateProducerDto>,
    review_service: CrudServiceImpl<Review, ReviewDto, CreateReviewDto, UpdateReviewDto>,
    user_service: CrudServiceImpl<User, UserDto, RegisterUserDto, UpdateUserDto>,
}

impl AppState {
    pub fn new(db: Database) -> AppState {
        AppState {
            anime_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("anime"),
            ))),
            character_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("characters"),
            ))),
            club_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("clubs"),
            ))),
            genre_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("genres"),
            ))),
            magazine_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("magazines"),
            ))),
            manga_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("manga"),
            ))),
            people_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("people"),
            ))),
            producer_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("producers"),
            ))),
            review_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("reviews"),
            ))),
            user_service: CrudServiceImpl::new(Arc::from(DatabaseRepository::new(
                db.collection("users"),
            ))),
        }
    }
}

