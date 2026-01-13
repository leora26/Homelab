use crate::db::user_repository::UserRepositoryImpl;
use crate::db::white_listed_user_repository::WhiteListedUserRepositoryImpl;
use crate::grpc::user_grpc_service::GrpcUserService;
use crate::service::user_service::{UserService, UserServiceImpl};
use crate::service::white_listed_user_service::{WhiteListedServiceImpl, WhiteListedUserService};
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use homelab_proto::user::user_service_server::UserServiceServer;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tonic::transport::Server;
use tracing_subscriber::EnvFilter;
use crate::events::rabbitmq::RabbitMqPublisher;

pub mod data;
pub mod events;
pub mod db;
pub mod grpc;
pub mod handler;
pub mod helpers;
pub mod service;

pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub white_listed_user_service: Arc<dyn WhiteListedUserService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dotenv().ok();

    let server_mode = env::var("SERVER_MODE")
        .unwrap_or_else(|_| "hybrid".to_string())
        .to_lowercase();

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("🚀 Server started successfully at http://127.0.0.1:8081");

    let rabbit_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://admin:password@localhost:5672".to_string());

    let publisher = Arc::new(RabbitMqPublisher::new(&rabbit_url).await?);

    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let wlu_repo = Arc::new(WhiteListedUserRepositoryImpl::new(pool.clone()));

    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone(), publisher.clone()));
    let white_listed_user_service = Arc::new(WhiteListedServiceImpl::new(
        wlu_repo.clone(),
        user_repo.clone(),
    ));

    let app_state = web::Data::new(AppState {
        user_service,
        white_listed_user_service,
    });

    let rest_addr = ("0.0.0.0", 8081);
    let grpc_addr: std::net::SocketAddr = "[::1]:50052".parse().unwrap();

    println!(
        "System starting in [{}] mode...",
        server_mode.to_uppercase()
    );

    match server_mode.as_str() {
        "rest" => {
            println!(
                "🚀 Starting REST Server only at http://{}:{}",
                rest_addr.0, rest_addr.1
            );
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .configure(handler_config)
            })
            .bind(rest_addr)?
            .run()
            .await?;
        }
        "grpc" => {
            println!("🚀 Starting gRPC Server only at {}", grpc_addr);
            let app_state_arc = app_state.clone().into_inner();

            let user_impl = GrpcUserService::new(app_state_arc.clone());

            Server::builder()
                .add_service(UserServiceServer::new(user_impl))
                .serve(grpc_addr)
                .await?;
        }
        "hybrid" => {
            println!("🚀 Starting Hybrid Mode (REST + gRPC)");

            let app_state_arc = app_state.clone().into_inner();

            let user_impl = GrpcUserService::new(app_state_arc.clone());

            let grpc_handle = Server::builder()
                .add_service(UserServiceServer::new(user_impl))
                .serve(grpc_addr);

            println!(
                "   - REST listening at http://{}:{}",
                rest_addr.0, rest_addr.1
            );
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .configure(handler_config)
            })
            .bind(rest_addr)?
            .run()
            .await?;

            let _ = grpc_handle.await;
        }
        _ => panic!(
            "Invalid SERVER_MODE: {}. Use 'rest', 'grpc', or 'hybrid'",
            server_mode
        ),
    }

    Ok(())
}

fn handler_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(handler::user_handler::config)
            .configure(handler::white_listed_user_handler::config),
    );
}
