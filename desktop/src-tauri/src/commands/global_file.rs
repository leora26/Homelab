use crate::common::EntityId;
use crate::helpers::mappings::map_global_file_proto_to_view;
use crate::helpers::with_auth::with_auth;
use crate::nas::global_file_service_client::GlobalFileServiceClient;
use crate::nas::GlobalFileCommand;
use crate::types::model::GlobalFileView;
use crate::AppState;
use tonic::Request;
use crate::helpers::auth_token::auth_token;

/// Lists every file published as global. Any authenticated user sees the full list;
/// each entry carries the underlying file's metadata so the UI can render it directly.
#[tauri::command]
pub async fn get_global_files(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<GlobalFileView>, String> {
    let token = auth_token(&state).await?;
    let mut client = GlobalFileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let response = client
        .get_all(Request::new(()))
        .await
        .map_err(|e| format!("gRPC GetAll global files failed: {}", e))?;

    let list = response.into_inner();

    let global_files = list
        .global_files
        .into_iter()
        .map(map_global_file_proto_to_view)
        .collect();

    Ok(global_files)
}

/// Publishes a file as global so every authenticated user can see and download it.
/// Only the owner may do this (enforced on the backend).
#[tauri::command]
pub async fn make_file_global(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = GlobalFileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(GlobalFileCommand {
        file_id: Some(EntityId { value: file_id }),
    });

    client
        .make_global(request)
        .await
        .map_err(|e| format!("gRPC MakeGlobal failed: {}", e))?;

    Ok(())
}

/// Un-publishes a file. Only the owner may do this (enforced on the backend); the
/// underlying file itself is left untouched.
#[tauri::command]
pub async fn make_file_private(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = GlobalFileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(GlobalFileCommand {
        file_id: Some(EntityId { value: file_id }),
    });

    client
        .make_private(request)
        .await
        .map_err(|e| format!("gRPC MakePrivate failed: {}", e))?;

    Ok(())
}

/// Whether a file is currently published as global. Used to render the correct
/// publish/un-publish action in the UI.
#[tauri::command]
pub async fn is_file_global(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<bool, String> {
    let token = auth_token(&state).await?;
    let mut client = GlobalFileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(GlobalFileCommand {
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .is_global(request)
        .await
        .map_err(|e| format!("gRPC IsGlobal failed: {}", e))?;

    Ok(response.into_inner().is_global)
}
