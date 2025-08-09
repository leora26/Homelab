use actix_web::{get, web, HttpResponse, Responder};
use tracing_subscriber::fmt::format;
use uuid::Uuid;
use crate::AppState;
use crate::service::folder_service;

#[get("/folders/{userId}/root")]
pub async fn get_foot_folder(
    app_state: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let user_id = Uuid::parse_str(&path.into_inner()).unwrap();


    match folder_service::find_root_folder(&app_state.db_pool, &user_id).await {
        Ok(Some(folder)) => HttpResponse::Ok().json(folder),
        Ok(None) => HttpResponse::NotFound().body(format!("No root folder was found for user with id: {}", user_id)),
        Err(e) => {
            tracing::error!("Failed to fetch root folder: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}

#[get("/folders/{id}")]
pub async fn get_folder_by_id (
    app_state: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let folder_id = Uuid::parse_str(&path.into_inner()).unwrap();

    match folder_service::find_folder_by_id(&app_state.db_pool, &folder_id).await {
        Ok(Some(folder)) => HttpResponse::Ok().json(folder),
        Ok(None) => HttpResponse::NotFound().body(format!("Could not find record of folder with an id of {}", folder_id)),
        Err(e) => {
            tracing::error!("Failed to find folder: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/folders/{id}/subfolders")]
pub async fn get_all_subfolders (
    app_state: web::Data<AppState>,
    path: web::Path<String>
)
-> impl Responder {
    let folder_id = Uuid::parse_str(&path.into_inner()).unwrap();

    match folder_service::find_all_children_folder(&app_state.db_pool, &folder_id).await {
        Ok(folders) => {
            if folders.is_empty() {
                HttpResponse::NotFound().body("No subfolders were found")
            } else {
                HttpResponse::Ok().json(folders)
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch subfolders: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_foot_folder);
    cfg.service(get_folder_by_id);
    cfg.service(get_all_subfolders);
}