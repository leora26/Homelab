use crate::AppState;
use actix_files::NamedFile;
use actix_web::web::{Data, Path, Query, ServiceConfig};
use actix_web::{error, get};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DownloadQuery {
    pub name: Option<String>,
}

#[get("/files/{id}/download")]
async fn download_file(
    file_id: Path<Uuid>,
    query: Query<DownloadQuery>,
    app_state: Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let id = file_id.into_inner();

    let path = match app_state.file_read_service.get_file_for_streaming(id).await {
        Ok(path) => path,
        Err(e) => {
            tracing::error!("Failed to download a file: {:?}", e);
            return Err(error::ErrorNotFound("File not found or access denied"));
        }
    };

    let mut named_file = NamedFile::open(path).map_err(|e| {
        eprintln!("File exists in DB but not on disk: {:?}", e);
        error::ErrorNotFound("File content is missing")
    })?;

    let filename = query.name.clone().unwrap_or_else(|| "download".to_string());

    named_file = named_file.set_content_disposition(ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(filename)],
    });

    Ok(named_file)
}

#[get("/files/{id}/preview")]
async fn preview_file(
    file_id: Path<Uuid>,
    app_state: Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let id = file_id.into_inner();

    let path = match app_state.file_read_service.get_file_preview_for_streaming(id).await {
        Ok(path) => path,
        Err(e) => {
            tracing::error!("Failed to download preview for a file: {:?}", e);
            return Err(error::ErrorNotFound("File not found or access denied"));
        }
    };

    let named_file = NamedFile::open(path).map_err(|e| {
        eprintln!("File exists in DB but not on disk: {:?}", e);
        error::ErrorNotFound("File content is missing")
    })?;

    Ok(named_file)
}

pub fn config(c: &mut ServiceConfig) {
    c.service(download_file);
    c.service(preview_file);
}
