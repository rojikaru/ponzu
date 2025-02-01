use actix_web::web::{scope};

pub fn create_anime_scope() -> actix_web::Scope {
    scope("/anime")
        // .service(get_all_anime_titles)
        // .service(get_anime_title)
}

// #[get("")]
// pub async fn get_all_anime_titles(data: Data<AppState>) -> impl Responder {
//     match data.anime_service.find(None, None).await {
//         Ok(anime) => Ok(HttpResponse::Ok().json(anime)),
//         Err(e) => Err(e),
//     }
// }

// #[get("{id}")]
// pub async fn get_anime_title(path: Path<String>, data: Data<AppState>) -> impl Responder {
//     let id = path.into_inner();
//     match data.anime_service.find_one(doc! {"_id": id.as_str()}).await {
//         Ok(anime) => match anime {
//             Some(a) => Ok(HttpResponse::Ok().json(a)),
//             None => Err(AppError::NotFound("Anime not found".to_string())),
//         },
//         Err(e) => Err(e),
//     }
// }

// #[post("")]
// pub async fn create_anime_title(form: Form<Anime>, data: Data<AppState>) -> impl Responder {
//     match data.anime_service.insert_one(form.into_inner()).await {
//         Ok(anime) => Ok(HttpResponse::Created().json(anime)),
//         Err(e) => Err(e),
//     }
// }

// #[patch("{id}")]
// pub async fn update_anime_title(
//     path: Path<String>,
//     form: Form<Anime>,
//     data: Data<AppState>
// ) -> impl Responder {
//     let id = path.into_inner();
//     match data.anime_service.update_by_id(id.as_str(), form.into_inner()).await {
//         Ok(anime) => Ok(HttpResponse::Ok().json(anime)),
//         Err(e) => Err(e),
//     }
// }

// #[delete("{id}")]
// pub async fn delete_anime_title(path: Path<String>, data: Data<AppState>) -> impl Responder {
//     let id = path.into_inner();
//     match data
//         .anime_service
//         .delete_one(doc! {"_id": id.as_str()})
//         .await
//     {
//         Ok(_) => Ok(HttpResponse::NoContent().finish()),
//         Err(e) => Err(e),
//     }
// }
