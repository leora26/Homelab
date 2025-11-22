use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query};
use uuid::Uuid;
use crate::AppState;
use crate::data::search_query::SearchQuery;
use crate::data::update_file_name_command::UpdateFileNameCommand;
use crate::data::upload_file_command::UploadFileCommand;
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

#[post("/files")]
pub async fn upload_file(
    app_state: Data<AppState>,
    req: Json<UploadFileCommand>,
) -> impl Responder {
    let command = req.into_inner();

    match app_state.file_service.upload(command).await {
        Ok(file) => {
            HttpResponse::Created().json(file)
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

pub fn config(c: &mut web::ServiceConfig) {
    c.service(get_file);
    c.service(delete_file);
    c.service(upload_file);
    c.service(rename_file);
    c.service(search_file);
}