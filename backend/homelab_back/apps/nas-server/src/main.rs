pub mod data;
pub mod db;
pub mod exception;
pub mod grpc;
pub mod handler;
pub mod helpers;
pub mod service;
pub mod jobs;

use crate::db::file_label_repository::FileLabelRepositoryImpl;
use crate::db::file_repository::{FileRepository, FileRepositoryImpl};
use crate::db::folder_repository::FolderRepositoryImpl;
use crate::db::global_file_repository::GlobalFileRepositoryImpl;
use crate::db::label_repository::LabelRepositoryImpl;
use crate::db::shared_file_repository::SharedFileRepositoryImpl;
use crate::db::user_repository::UserRepositoryImpl;

// TODO: add all other Grpc servers
use homelab_proto::nas::file_service_server::FileServiceServer;

use crate::service::file_label_service::{FileLabelService, FileLabelServiceImpl};
use crate::service::file_service::{FileService, FileServiceImpl};
use crate::service::folder_service::{FolderService, FolderServiceImpl};
use crate::service::global_file_service::{GlobalFileService, GlobalFileServiceImpl};
use crate::service::label_service::{LabelService, LabelServiceImpl};
use crate::service::shared_file_service::{SharedFileService, SharedFileServiceImpl};

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tonic::transport::Server;
use crate::jobs::delete_cron_job::init_delete_job;
use tracing_subscriber::EnvFilter;
use crate::grpc::file_grpc_service::GrpcFileService;

pub struct AppState {
    pub file_service: Arc<dyn FileService>,
    pub folder_service: Arc<dyn FolderService>,
    pub shared_file_service: Arc<dyn SharedFileService>,
    pub file_repo: Arc<dyn FileRepository>,
    pub global_file_service: Arc<dyn GlobalFileService>,
    pub label_service: Arc<dyn LabelService>,
    pub file_label_service: Arc<dyn FileLabelService>,
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
    let share_file_repo = Arc::new(SharedFileRepositoryImpl::new(pool.clone()));
    let global_file_repo = Arc::new(GlobalFileRepositoryImpl::new(pool.clone()));
    let label_repo = Arc::new(LabelRepositoryImpl::new(pool.clone()));
    let file_label_repo = Arc::new(FileLabelRepositoryImpl::new(pool.clone()));

    let folder_service = Arc::new(FolderServiceImpl::new(folder_repo.clone()));
    let file_service = Arc::new(FileServiceImpl::new(
        file_repo.clone(),
        folder_repo.clone(),
        user_repo.clone(),
        root_path.to_path_buf(),
        global_file_repo.clone(),
    ));
    let shared_file_service = Arc::new(SharedFileServiceImpl::new(
        share_file_repo.clone(),
        user_repo.clone(),
        file_repo.clone(),
    ));
    let global_file_service = Arc::new(GlobalFileServiceImpl::new(global_file_repo.clone()));
    let label_service = Arc::new(LabelServiceImpl::new(label_repo.clone(), user_repo.clone()));
    let file_label_service = Arc::new(FileLabelServiceImpl::new(
        label_repo.clone(),
        file_repo.clone(),
        file_label_repo.clone(),
        user_repo.clone(),
    ));

    let _cleanup_scheduler = init_delete_job(file_service.clone()).await;

    let app_state = web::Data::new(AppState {
        file_service,
        folder_service,
        shared_file_service,
        file_repo: file_repo.clone(),
        global_file_service,
        label_service,
        file_label_service
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

            let file_impl = GrpcFileService::new(app_state_arc.clone());

            Server::builder()
                .add_service(FileServiceServer::new(file_impl))
                .serve(grpc_addr)
                .await?;
        }
        "hybrid" => {
            println!("🚀 Starting Hybrid Mode (REST + gRPC)");

            let app_state_arc = app_state.clone().into_inner();
            
            let file_impl = GrpcFileService::new(app_state_arc.clone());

            let grpc_handle = Server::builder()
                .add_service(FileServiceServer::new(file_impl))
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
            .configure(handler::folder_handler::config)
            .configure(handler::file_handler::config)
            .configure(handler::shared_file_handler::config),
    );
}
