use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, Movie};
use super::models::{CreateMovie, UpdateMovie};

#[get("/movies")]
async fn get_movies(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.movies.lock().unwrap().to_vec())
}

#[post("/movies")]
async fn create_movie(data: web::Data<AppState>, param_obj: web::Json<CreateMovie>) -> impl Responder {
    let mut movies = data.movies.lock().unwrap();
    let mut max_id: i32 = 0;

    for i in 0..movies.len() {
        if movies[i].id > max_id {
            max_id = movies[i].id;
        }
    }

    movies.push(Movie {
        id: max_id + 1,
        title: param_obj.title.clone()
    });

    HttpResponse::Ok().json(movies.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_movies)
    .service(create_movie);
}