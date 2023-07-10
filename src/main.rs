use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod movies;
use movies::services;

struct AppState {
    movies: Mutex<Vec<Movie>>
}

#[derive(Serialize, Deserialize, Clone)]
struct Movie {
    id: i32,
    title: String
}

#[get("/")]
async fn index() -> String {
    "It is working".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        movies: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(index)
        .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
