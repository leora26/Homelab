pub mod domain;
pub mod db;
pub mod service;
pub mod handler;
pub mod data;
pub mod exception;
pub mod types;
pub mod helpers;
mod constants;

use std::env;
use std::path::Path;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::db::file_repository::{FileRepository, FileRepositoryImpl};
use crate::db::white_listed_user_repository::WhiteListedUserRepositoryImpl;
use crate::db::folder_repository::FolderRepositoryImpl;
use crate::db::shared_file_repository::SharedFileRepositoryImpl;
use crate::db::user_repository::UserRepositoryImpl;
use crate::service::file_service::{FileService, FileServiceImpl};
use crate::service::folder_service::{FolderService, FolderServiceImpl};
use crate::service::shared_file_service::{SharedFileService, SharedFileServiceImpl};
use crate::service::user_service::{UserService, UserServiceImpl};
use crate::service::white_listed_user_service::{WhiteListedServiceImpl, WhiteListedUserService};

pub struct AppState {
    pub file_service: Arc<dyn FileService>,
    pub folder_service: Arc<dyn FolderService>,
    pub user_service: Arc<dyn UserService>,
    pub white_listed_user_service: Arc<dyn WhiteListedUserService>,
    pub shared_file_service: Arc<dyn SharedFileService>,
    pub file_repo: Arc<dyn FileRepository>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABSE_URL must be set in .env file");

    let root_folder_path = env::var("ROOT_FOLDER_PATH")
        .expect("ROOT_FOLDER_PATH must be set in .env file");


    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    println!("🚀 Server started successfully at http://127.0.0.1:8080");

    let root_path = Path::new(&root_folder_path);

    if !root_path.exists() {
        if let Err(e) = std::fs::create_dir_all(root_path) {
            panic!("Failed to create root directory at {}: {}", &root_folder_path, e);
        } else {
            println!("Root folder was created.");
        }
    }

    let file_repo = Arc::new(FileRepositoryImpl::new(pool.clone()));
    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let folder_repo = Arc::new(FolderRepositoryImpl::new(pool.clone()));
    let wlu_repo = Arc::new(WhiteListedUserRepositoryImpl::new(pool.clone()));
    let share_file_repo = Arc::new(SharedFileRepositoryImpl::new(pool.clone()));

    let folder_service = Arc::new(FolderServiceImpl::new(folder_repo.clone()));
    let file_service = Arc::new(FileServiceImpl::new(file_repo.clone(), folder_repo.clone(), user_repo.clone()));
    let user_service = Arc::new(UserServiceImpl::new(user_repo.clone()));
    let wlu_service = Arc::new(WhiteListedServiceImpl::new(wlu_repo.clone(), user_repo.clone()));
    let shared_file_service = Arc::new(SharedFileServiceImpl::new(share_file_repo.clone(), user_repo.clone(), file_repo.clone()));

    let app_state = web::Data::new(
        AppState {
            file_service,
            folder_service,
            user_service,
            white_listed_user_service: wlu_service,
            shared_file_service,
            file_repo: file_repo.clone(),
        });


    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .configure(handler::user_handler::config)
                    .configure(handler::folder_handler::config)
                    .configure(handler::file_handler::config)
                    .configure(handler::white_listed_user_handler::config)
                    .configure(handler::shared_file_handler::config)
                    .configure(handler::test::config)
            )
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
