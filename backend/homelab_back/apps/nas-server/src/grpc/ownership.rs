// Request-scoped ownership guards.
//
// These are enforced at the gRPC boundary (not in the repository layer) because
// the repositories are also driven by system-wide background jobs (the cleanup
// scheduler and RabbitMQ consumers) that must not be constrained to a single owner.

use crate::AppState;
use homelab_core::nas_domain::file::File;
use homelab_core::nas_domain::folder::Folder;
use tonic::Status;
use uuid::Uuid;

/// Loads a folder and ensures it belongs to `user_id`.
///
/// Returns `not_found` both when the folder does not exist and when it is owned by
/// someone else, so a caller cannot probe for the existence of resources they do
/// not own.
pub async fn folder_owned_by(
    app_state: &AppState,
    folder_id: Uuid,
    user_id: Uuid,
) -> Result<Folder, Status> {
    app_state
        .folder_read_service
        .get_by_id(folder_id)
        .await?
        .filter(|folder| folder.owner_id == user_id)
        .ok_or_else(|| Status::not_found("Folder not found"))
}

/// Loads a file (regardless of trash state) and ensures it belongs to `user_id`.
///
/// Uses a state-agnostic lookup so it also guards operations on trashed files
/// (restore, permanent delete). Returns `not_found` on a missing or non-owned file.
pub async fn file_owned_by(
    app_state: &AppState,
    file_id: Uuid,
    user_id: Uuid,
) -> Result<File, Status> {
    app_state
        .file_repo
        .get_deleted_by_id(file_id)
        .await?
        .filter(|file| file.owner_id == user_id)
        .ok_or_else(|| Status::not_found("File not found"))
}
