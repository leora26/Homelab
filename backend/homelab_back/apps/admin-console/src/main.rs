use std::env;
use std::error::Error;
use std::sync::Arc;
use actix_web::{web};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;
use crate::db::user_repo::UserRepoImpl;
use crate::db::wlu_repo::WluRepoImpl;

pub mod data;
pub mod db;
pub mod helpers;
pub mod events;
pub mod service;

pub struct AppState {
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dotenv().ok();

    let server_mode = env::var("SERVER_MODE")
        .unwrap_or_else(|_| "hybrid".to_string())
        .to_lowercase();

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set in .env file");

    let root_folder_path =
        env::var("ROOT_FOLDER_PATH").expect("ROOT_FOLDER_PATH must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let console_user_repo = Arc::new(UserRepoImpl::new(pool.clone()));
    let console_wlu_repo = Arc::new(WluRepoImpl::new(pool.clone()));

    let rabbit_url = env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://admin:password@localhost:5672".to_string());

    let app_state = web::Data::new(AppState {});

    let grpc_addr: std::net::SocketAddr = "[::1]:50053".parse().unwrap();

    println!(
        "System starting in [{}] mode...",
        server_mode.to_uppercase()
    );

    match server_mode.as_str() {
        "grpc" => {
            println!("🚀 Starting gRPC Server only at {}", grpc_addr);
            let app_state_arc = app_state.clone().into_inner();

            // Server::builder()
            //     .add_service(UserServiceServer::new(user_impl))
            //     .serve(grpc_addr)
            //     .await?;
        }
        _ => panic!(
            "Invalid SERVER_MODE: {}. Use 'rest', 'grpc', or 'hybrid'",
            server_mode
        ),
    }

    Ok(())
}