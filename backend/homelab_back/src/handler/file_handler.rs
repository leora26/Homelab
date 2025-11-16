use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::AppState;
use crate::data::search_query::SearchQuery;
use crate::data::update_file_name_command::UpdateFileNameCommand;
use crate::data::upload_file_command::UploadFileCommand;

#[get("/files/{id}")]
pub async fn get_file(
    app_state: web::Data<AppState>,
    path: web::Path<String>
)
-> impl Responder {
    let file_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };

    match app_state.file_service.get_by_id(&file_id).await {
        Ok(Some(file)) => HttpResponse::Ok().json(file),
        Ok(None) => HttpResponse::NotFound().body(format!("Was not able to find file with a given id: {}", file_id)),
        Err(e) => {
            tracing::error!("Failed to fetch a file: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/files/{id}")]
pub async fn delete_file (
    app_state: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let file_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid folder ID format");
        }
    };
    
    match app_state.file_service.delete(&file_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete a file: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/files")]
pub async fn upload_file (
    app_state: web::Data<AppState>,
    req: web::Json<UploadFileCommand>
) -> impl Responder {
    let command = req.into_inner();

    match app_state.file_service.upload(command).await {
        Ok(file) => {
            HttpResponse::Created().json(file)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed while creating a file: {}", e))
        }
    }
}

#[patch("/files/{fileId}/name")]
pub async fn rename_file (
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdateFileNameCommand>
) -> impl Responder {
    let command =  req.into_inner();

    let file_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid file ID format");
        }
    };

    match app_state.file_service.update_file_name(command, file_id).await {
        Ok(file) => {
            HttpResponse::Ok().json(file)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to update name of a file: {}", e))
        }
    }
}

#[get("/files/search")]
pub async fn search_file (
    app_state: web::Data<AppState>,
    query: web::Query<SearchQuery>
) -> impl Responder {
    let search_term = query.into_inner().q;

    match app_state.file_service.search_file(search_term).await {
        Ok(f) => {
            if f.is_empty() {
                HttpResponse::Ok().body("No files for the given search query")
            } else {
                HttpResponse::Ok().json(f)
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to search for a file: {}", e))
        }
    }
}

pub fn config (c: &mut web::ServiceConfig) {
    c.service(get_file);
    c.service(delete_file);
    c.service(upload_file);
    c.service(rename_file);
    c.service(search_file);
}