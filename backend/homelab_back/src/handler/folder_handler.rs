use crate::AppState;
use actix_web::{delete, get, patch, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::data::search_query::SearchQuery;
use crate::data::update_folder_name_command::UpdateFolderNameCommand;

#[get("/folders/{userId}/root")]
pub async fn get_root_folder(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };
    match app_state.folder_service.get_root(&user_id).await {
        Ok(Some(folder)) => HttpResponse::Ok().json(folder),
        Ok(None) => HttpResponse::NotFound().body(format!(
            "No root folder was found for user with id: {}",
            user_id
        )),
        Err(e) => {
            tracing::error!("Failed to fetch root folder: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/folders/{id}")]
pub async fn get_folder_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let folder_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };
    match app_state.folder_service.get_by_id(&folder_id).await {
        Ok(Some(folder)) => HttpResponse::Ok().json(folder),
        Ok(None) => HttpResponse::NotFound().body(format!(
            "Could not find record of folder with an id of {}",
            folder_id
        )),
        Err(e) => {
            tracing::error!("Failed to find folder: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/folders/{id}/subfolders")]
pub async fn get_all_subfolders(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let folder_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };

    match app_state.folder_service.get_children_by_id(&folder_id).await {
        Ok(folders) => {
            if folders.is_empty() {
                HttpResponse::NotFound().body("No subfolders were found")
            } else {
                HttpResponse::Ok().json(folders)
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch subfolders: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/folders/{id}")]
pub async fn delete_folder(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let folder_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };
    match app_state.folder_service.delete(&folder_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete a folder: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[get("folders/{folderId}/file")]
pub async fn fetch_files_for_folder (
    app_state: web::Data<AppState>,
    path: web::Path<String>
)
    -> impl Responder {
    let folder_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };

    match app_state.folder_service.get_by_folder(&folder_id).await {
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

#[patch("/folders/{id}/name")]
pub async fn rename_folder (
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFolderNameCommand>
) -> impl Responder {
    let command: UpdateFolderNameCommand = req.into_inner();

    let folder_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder id format");
        }
    };

    match app_state.folder_service.update_folder_name(command, folder_id).await {
        Ok(f) => {
            HttpResponse::Ok().json(f)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to update name of a foler: {}", e))
        }
    }
}

#[get("/folders/search")]
pub async fn search_folder (
    app_state: web::Data<AppState>,
    query: web::Query<SearchQuery>
) -> impl Responder {
    let search_term = query.into_inner().q;

    match app_state.folder_service.search_folder(search_term).await {
        Ok(f) => {
            if f.is_empty() {
                HttpResponse::Ok().body("No folders for the given search query")
            } else {
                HttpResponse::Ok().json(f)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to search for a folder: {}", e))
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_root_folder);
    cfg.service(get_folder_by_id);
    cfg.service(get_all_subfolders);
    cfg.service(delete_folder);
    cfg.service(fetch_files_for_folder);
    cfg.service(rename_folder);
    cfg.service(search_folder);
}
