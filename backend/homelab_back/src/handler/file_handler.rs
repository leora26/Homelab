use actix_web::{delete, get, patch, post, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use uuid::Uuid;
use crate::AppState;
use crate::data::file_folder::delete_chosen_files_command::DeleteChosenFilesCommand;
use crate::data::file_folder::search_query::SearchQuery;
use crate::data::file_folder::update_file_name_command::UpdateFileNameCommand;
use crate::data::file_folder::init_file_command::InitFileCommand;
use crate::helpers::error_mapping::map_data_err_to_http;

#[get("/files/{id}")]
pub async fn get_file(
    app_state: Data<AppState>,
    file_id: Path<Uuid>,
)
    -> impl Responder {
    let id = file_id.into_inner();

    match app_state.file_service.get_by_id(id).await {
        Ok(Some(file)) => HttpResponse::Ok().json(file),
        Ok(None) => HttpResponse::NotFound().body(format!("Was not able to find file with a given id: {}", id)),
        Err(e) => {
            tracing::error!("Failed to fetch a file: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/files/search")]
pub async fn search_file(
    app_state: Data<AppState>,
    query: Query<SearchQuery>,
) -> impl Responder {
    let search_term = query.into_inner().q;

    match app_state.file_service.search_file(search_term).await {
        Ok(f) => {
            HttpResponse::Ok().json(f)
        }
        Err(e) => {
            tracing::error!("Failed to search for a file: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/files/deleted")]
pub async fn get_all_deleted_files(
    app_state: Data<AppState>
) -> impl Responder {
    match app_state.file_service.get_all_deleted_files().await {
        Ok(f) => {
            if f.is_empty() {
                HttpResponse::NoContent().body("No deleted files were found")
            } else {
                HttpResponse::Ok().json(f)
            }
        }
        Err(e) => {
            tracing::error!("Failed to get all deleted files: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[post("/files")]
pub async fn init_file(
    app_state: Data<AppState>,
    req: Json<InitFileCommand>,
) -> impl Responder {
    match app_state.file_service.upload(req.into_inner()).await {
        Ok(file) => {
            HttpResponse::Created().json(file.id)
        }
        Err(e) => {
            tracing::error!("Failed while creating a file: {}", e);
            map_data_err_to_http(e)
        }
    }
}


#[patch("/files/{fileId}")]
pub async fn rename_file(
    app_state: Data<AppState>,
    file_id: Path<Uuid>,
    req: Json<UpdateFileNameCommand>,
) -> impl Responder {
    let command = req.into_inner();

    match app_state.file_service.update_file_name(command, file_id.into_inner()).await {
        Ok(file) => {
            HttpResponse::Ok().json(file)
        }
        Err(e) => {
            tracing::error!("Failed to rename a file: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[patch("/files/{id}/undelete")]
pub async fn undelete_file(
    app_state: Data<AppState>,
    id: Path<Uuid>,
) -> impl Responder {
    match app_state.file_service.update_deleted_file(id.into_inner()).await {
        Ok(f) => HttpResponse::Ok().json(f),
        Err(e) => {
            tracing::error!("Failed to undelete a file");
            map_data_err_to_http(e)
        }
    }
}


#[delete("/files/all")]
pub async fn delete_chosen_files(
    app_state: Data<AppState>,
    req: Json<DeleteChosenFilesCommand>,
) -> impl Responder {
    let command: DeleteChosenFilesCommand = req.into_inner();

    match app_state.file_service.delete_chosen_files(&command.files_ids).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete chosen files: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[delete("/files/{id}")]
pub async fn delete_file(
    app_state: Data<AppState>,
    file_id: Path<Uuid>,
) -> impl Responder {
    match app_state.file_service.delete(file_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete a file: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}


pub fn config(c: &mut ServiceConfig) {
    c.service(get_file);
    c.service(delete_file);
    c.service(init_file);
    c.service(rename_file);
    c.service(search_file);
    c.service(get_all_deleted_files);
}