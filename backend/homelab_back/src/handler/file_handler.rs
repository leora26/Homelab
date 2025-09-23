use actix_web::{delete, get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::AppState;
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

#[get("/files/folder/{folderId}")]
pub async fn fetch_files (
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

    match app_state.file_service.get_by_folder(&folder_id).await {
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


pub fn config (c: &mut web::ServiceConfig) {
    c.service(fetch_files);
    c.service(get_file);
    c.service(delete_file);
    c.service(upload_file);
}