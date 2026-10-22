use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use redis::Client;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    match dotenvy::dotenv() {
        Ok(_) => println!("ENV loaded"),
        Err(e) => println!("env error {}", e),
    }

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => panic!("DATABASE_URL not found in environment: {}", e),
    };

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn redis_db() -> Client {

    match dotenvy::dotenv() {
        Ok(_) => println!("ENV loaded"),
        Err(e) => println!("env error {}", e),
    }

    let redis_url = match env::var("REDIS_URL") {
        Ok(url) => url,
        Err(e) => panic!("REDIS_URL not found in environment: {}", e),
    };

    let client = redis::Client::open(redis_url).expect("REDIS_URL must be set");
    client
}
