use crate::AppState;
use actix_files::NamedFile;
use actix_web::web::{Data, Path, ServiceConfig};
use actix_web::{error, get};
use uuid::Uuid;

#[get("/files/{id}/download")]
async fn download_file(
    file_id: Path<Uuid>,
    app_state: Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let id = file_id.into_inner();

    let path = match app_state.file_service.get_file_for_streaming(id).await {
        Ok(path) => path,
        Err(e) => {
            tracing::error!("Failed to download a file: {:?}", e);
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
}
