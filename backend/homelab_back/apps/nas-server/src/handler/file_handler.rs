use crate::AppState;
use actix_files::NamedFile;
use actix_web::web::{Data, Path, Query, ServiceConfig};
use actix_web::{error, get, HttpRequest};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use homelab_core::auth::bearer::{resolve_caller_id, AuthError};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DownloadQuery {
    pub name: Option<String>,
}

/// Authenticates a REST request and resolves the caller's internal user id. The
/// actix HTTP server is separate from the gRPC auth interceptor, so these endpoints
/// must authenticate themselves. Uses the shared `resolve_caller_id` helper and maps
/// its transport-agnostic error onto an actix error.
async fn authenticate(req: &HttpRequest, app_state: &AppState) -> actix_web::Result<Uuid> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    resolve_caller_id(&app_state.auth_state, &app_state.cached_identity_resolver, header)
        .await
        .map_err(|e| {
            let msg = e.message();
            match e {
                AuthError::MissingToken | AuthError::InvalidToken => error::ErrorUnauthorized(msg),
                AuthError::NotProvisioned => error::ErrorForbidden(msg),
                AuthError::MalformedInternalId => error::ErrorInternalServerError(msg),
            }
        })
}

/// Fetches a file and confirms it belongs to `user_id`, returning 404 otherwise so
/// callers cannot probe for files they do not own.
async fn assert_file_owner(app_state: &AppState, file_id: Uuid, user_id: Uuid) -> actix_web::Result<()> {
    let file = app_state
        .file_read_service
        .get_by_id(file_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load file for ownership check: {:?}", e);
            error::ErrorInternalServerError("Failed to load file")
        })?;

    match file {
        Some(f) if f.owner_id == user_id => Ok(()),
        _ => Err(error::ErrorNotFound("File not found")),
    }
}

#[get("/files/{id}/download")]
async fn download_file(
    req: HttpRequest,
    file_id: Path<Uuid>,
    query: Query<DownloadQuery>,
    app_state: Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let id = file_id.into_inner();

    let user_id = authenticate(&req, &app_state).await?;
    assert_file_owner(&app_state, id, user_id).await?;

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
    req: HttpRequest,
    file_id: Path<Uuid>,
    app_state: Data<AppState>,
) -> actix_web::Result<NamedFile> {
    let id = file_id.into_inner();

    let user_id = authenticate(&req, &app_state).await?;
    assert_file_owner(&app_state, id, user_id).await?;

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
