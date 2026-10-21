use actix_web::{
    App, HttpResponse, HttpServer,
    web::{self},
};
mod db;
mod models;
mod schema;
// use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use redis::Commands;
use serde::Deserialize;

use crate::{db::establish_connection, models::User};


#[derive(Deserialize)]
struct Submission {
    name: String,
    language: String,
    code: String,
}

struct AppState {
    redis: redis::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_connection = &mut establish_connection();

    dotenvy::dotenv().ok();
    let client = redis::Client::open(std::env::var("REDIS_URL").expect("REDIS_URL must be set"))
        .expect("Failed to create Redis client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                redis: client.clone()
            }))
            .route("/submission", web::post().to(create_submission))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn create_submission(
    body: web::Json<Submission>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let mut con = state.redis.get_connection().unwrap();

    let submission = serde_json::json!({
        "name": body.name,
        "language": body.language,
        "code": body.code,
    })
    .to_string();

    let _: () = con.lpush("submission-queue", submission.clone()).unwrap();

    // HttpResponse::Ok().body("Submission added to queue");
    // HttpResponse::Ok().body(submission)
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Submission added to queue",
        "data": submission
    }))
}

async fn create_user(body: web::Json<User>) {
    let user = serde_json::json!({
        "username": body.username,
        "password": body.password
    }).to_string();
}

// async fn get_submission( ) {

// }
