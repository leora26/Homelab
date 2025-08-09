use actix_web::{get, web, HttpResponse, Responder};
use tracing_subscriber::fmt::format;
use uuid::Uuid;
use crate::AppState;
use crate::service::file_service;

#[get("/files/{id}")]
pub async fn get_file(
    app_state: web::Data<AppState>,
    path: web::Path<String>
)
-> impl Responder {
    let file_id = Uuid::parse_str(&path.into_inner()).unwrap();

    match file_service::get_file_by_id(&app_state.db_pool, &file_id).await {
        Ok(Some(file)) => HttpResponse::Ok().json(file),
        Ok(None) => HttpResponse::NotFound().body(format!("Was not able to find file with a given id: {}", file_id)),
        Err(e) => {
            tracing::error!("Failed to fetch a file: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/files/folder/{folderId}")]
pub async fn fetch_files (
    app_state: web::Data<AppState>,
    path: web::Path<String>
)
-> impl Responder {
    let folder_id = Uuid::parse_str(&path.into_inner()).unwrap();

    match file_service::get_files_by_folder(&app_state.db_pool, &folder_id).await {
        Ok(files) => {
            if files.is_empty() {
                HttpResponse::NotFound().body(format!("There were no files found for the given folder with id: {}", folder_id))
            } else {
                HttpResponse::Ok().json(files)
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch files inside a folder: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }

    }
}


pub fn config (c: &mut web::ServiceConfig) {
    c.service(fetch_files);
    c.service(get_file);
}