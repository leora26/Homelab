pub mod domain;
pub mod db;
pub mod service;
pub mod handler;

use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, your server is running!"
}

pub struct AppState {
    db_pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABSE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    println!("🚀 Server started successfully at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db_pool: pool.clone() }))
            .service(
                web::scope("/api")
                .configure(handler::user_handler::config
                )
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
