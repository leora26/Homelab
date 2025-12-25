pub mod constants;
pub mod data;
pub mod db;
pub mod domain;
pub mod exception;
pub mod grpc;
pub mod handler;
pub mod helpers;
pub mod pb;
pub mod service;
pub mod types;

use crate::db::file_repository::{FileRepository, FileRepositoryImpl};
use crate::db::folder_repository::FolderRepositoryImpl;
use crate::db::global_file_repository::GlobalFileRepositoryImpl;
use crate::db::shared_file_repository::SharedFileRepositoryImpl;
use crate::db::user_repository::UserRepositoryImpl;
use crate::db::white_listed_user_repository::WhiteListedUserRepositoryImpl;
use crate::grpc::user_grpc_service::GrpcUserService;
use crate::grpc::white_listed_user_grpc_service::GrpcWhiteListedUserService;
use crate::pb::user_service_server::UserServiceServer;
use crate::pb::white_listed_user_service_server::WhiteListedUserServiceServer;
use crate::service::file_service::{FileService, FileServiceImpl};
use crate::service::folder_service::{FolderService, FolderServiceImpl};
use crate::service::shared_file_service::{SharedFileService, SharedFileServiceImpl};
use crate::service::user_service::{UserService, UserServiceImpl};
use crate::service::white_listed_user_service::{WhiteListedServiceImpl, WhiteListedUserService};
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tonic::transport::Server;
use crate::service::global_file_service::{GlobalFileService, GlobalFileServiceImpl};

pub struct AppState {
    pub file_service: Arc<dyn FileService>,
    pub folder_service: Arc<dyn FolderService>,
    pub user_service: Arc<dyn UserService>,
    pub white_listed_user_service: Arc<dyn WhiteListedUserService>,
    pub shared_file_service: Arc<dyn SharedFileService>,
    pub file_repo: Arc<dyn FileRepository>,
    pub global_file_service: Arc<dyn GlobalFileService>
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
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

    println!("🚀 Server started successfully at http://127.0.0.1:8080");

    let mut root_path = PathBuf::new();
    root_path.push(root_folder_path);

    if !root_path.exists() {
        if let Err(e) = std::fs::create_dir_all(&root_path) {
            panic!("Failed to create root directory: {}", e);
        } else {
            println!("Root folder was created.");
        }
    }

    let file_repo = Arc::new(FileRepositoryImpl::new(pool.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let folder_repo = Arc::new(FolderRepositoryImpl::new(pool.clone()));
    let wlu_repo = Arc::new(WhiteListedUserRepositoryImpl::new(pool.clone()));
    let share_file_repo = Arc::new(SharedFileRepositoryImpl::new(pool.clone()));
    let global_file_repo = Arc::new(GlobalFileRepositoryImpl::new(pool.clone()));

    let folder_service = Arc::new(FolderServiceImpl::new(folder_repo.clone()));
    let file_service = Arc::new(FileServiceImpl::new(
        file_repo.clone(),
        folder_repo.clone(),
        user_repo.clone(),
        root_path.to_path_buf(),
        global_file_repo.clone(),
    ));
    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let wlu_service = Arc::new(WhiteListedServiceImpl::new(
        wlu_repo.clone(),
        user_repo.clone(),
    ));
    let shared_file_service = Arc::new(SharedFileServiceImpl::new(
        share_file_repo.clone(),
        user_repo.clone(),
        file_repo.clone(),
    ));
    let global_file_service = Arc::new(GlobalFileServiceImpl::new(global_file_repo.clone()));

    let app_state = web::Data::new(AppState {
        file_service,
        folder_service,
        user_service,
        white_listed_user_service: wlu_service,
        shared_file_service,
        file_repo: file_repo.clone(),
        global_file_service
    });

    let rest_addr = ("0.0.0.0", 8080);
    let grpc_addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();

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

            let wlu_impl = GrpcWhiteListedUserService::new(app_state_arc.clone());
            let user_impl = GrpcUserService::new(app_state_arc.clone());

            Server::builder()
                .add_service(WhiteListedUserServiceServer::new(wlu_impl))
                .add_service(UserServiceServer::new(user_impl))
                .serve(grpc_addr)
                .await?;
        }
        "hybrid" => {
            println!("🚀 Starting Hybrid Mode (REST + gRPC)");

            let app_state_arc = app_state.clone().into_inner();

            let wlu_impl = GrpcWhiteListedUserService::new(app_state_arc.clone());
            let user_impl = GrpcUserService::new(app_state_arc.clone());

            let grpc_handle = Server::builder()
                .add_service(WhiteListedUserServiceServer::new(wlu_impl))
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
            .configure(crate::handler::user_handler::config)
            .configure(crate::handler::folder_handler::config)
            .configure(crate::handler::file_handler::config)
            .configure(crate::handler::white_listed_user_handler::config)
            .configure(crate::handler::shared_file_handler::config),
    );
}
