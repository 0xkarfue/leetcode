use actix_web::{
    App, HttpResponse, HttpServer, Responder, post,
    web::{self},
};
mod db;
mod models;
mod schema;
use diesel::RunQueryDsl;
// use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use redis::Commands;
use serde::Deserialize;

use crate::{
    db::{DbPool, establish_connection, redis_db},
    models::{NewUser},
    schema::users,
};

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
    let pool = establish_connection();
    let redis_client = redis_db();
    let app_state = web::Data::new(AppState {
        redis: redis_client,
    });

    let pool_data = web::Data::new(pool);

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone()) // Register the pool data
            .app_data(app_state.clone())
            .service(create_user)
            .service(create_submission) // Register the route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/signin")]
async fn create_user(pool: web::Data<DbPool>, body: web::Json<NewUser>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let new_user = NewUser {
        username: body.username.clone(),
        password: body.password.clone(),
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().body("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[post("/submission")]
async fn create_submission(
    body: web::Json<Submission>,
    state: web::Data<AppState>,
) -> impl Responder {
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
