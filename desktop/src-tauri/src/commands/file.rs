use crate::helpers::grpc_error::grpc_message;
use crate::common::EntityId;
use crate::helpers::mappings::map_file_proto_to_view;
use crate::helpers::with_auth::with_auth;
use crate::nas::file_chunk::Data;
use crate::nas::file_service_client::FileServiceClient;
use crate::nas::{ArchiveFileRequest, CopyFileRequest, DeleteChosenFilesRequest, DeleteFileRequest, FileChunk, GetDeletedFilesRequest, InitFileRequest, MoveFileRequest, RemoveDeletedFileRequest, RenameFileRequest, SearchFilesRequest, UnarchiveFileRequest, UndeleteFileRequest};
use crate::types::model::{FileView, UploadProgress};
use crate::AppState;
use async_stream::stream;
use tauri::{AppHandle, Emitter};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tonic::Request;
use crate::helpers::auth_token::auth_token;

#[tauri::command]
pub async fn init_file(
    state: tauri::State<'_, AppState>,
    name: String,
    destination: String,
    local_path: String,
) -> Result<FileView, String> {
    let metadata = fs::metadata(&local_path)
        .await
        .map_err(|e| format!("Couldn't read {}: {}", local_path, e))?;

    let size = metadata.len() as i64;

    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = Request::new(InitFileRequest {
        name,
        destination: Some(EntityId { value: destination }),
        owner_id: None,
        size,
    });

    let response = client
        .init_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

/// Emit progress at most every 256 KB. One event per 64 KB chunk floods the webview on a
/// large file and the UI can't render that fast anyway.
const PROGRESS_EVERY_BYTES: i64 = 256 * 1024;

#[tauri::command]
pub async fn upload_content(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    file_id: String,
    local_path: String,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let total_bytes = fs::metadata(&local_path)
        .await
        .map(|m| m.len() as i64)
        .unwrap_or(0);

    let mut file = File::open(&local_path)
        .await
        .map_err(|e| format!("Couldn't open {}: {}", local_path, e))?;

    let progress_id = file_id.clone();

    let outbound_stream = stream! {
        yield FileChunk {
            data: Some(Data::FileId(EntityId { value: file_id.clone() })),
        };

        let mut buffer = vec![0; 64 * 1024];
        let mut sent: i64 = 0;
        let mut last_emitted: i64 = 0;

        loop {
            match file.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    yield FileChunk {
                        data: Some(Data::Content(buffer[..n].to_vec())),
                    };

                    sent += n as i64;

                    // Always emit the final chunk so the bar lands on 100%.
                    if sent - last_emitted >= PROGRESS_EVERY_BYTES || sent >= total_bytes {
                        last_emitted = sent;
                        let _ = app.emit(
                            "upload_progress",
                            UploadProgress {
                                file_id: progress_id.clone(),
                                bytes_sent: sent,
                                total_bytes,
                            },
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    break;
                }
            }
        }
    };

    let request = Request::new(outbound_stream);

    client
        .upload_content(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_file(state: tauri::State<'_, AppState>, file_id: String) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(DeleteFileRequest {
        id: Some(EntityId {
            value: file_id.clone(),
        }),
    });

    let response = client
        .delete_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn rename_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
    new_name: String,
) -> Result<FileView, String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(RenameFileRequest {
        id: Some(EntityId { value: file_id }),
        new_name: new_name.clone(),
    });

    let response = client
        .rename_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn get_deleted_files(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = Request::new(GetDeletedFilesRequest { user_id: None });

    let response = client
        .get_deleted_files(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let file_response = response.into_inner();

    let files = file_response
        .files
        .into_iter()
        .map(|f| map_file_proto_to_view(f))
        .collect();

    Ok(files)
}

#[tauri::command]
pub async fn restore_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<FileView, String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(UndeleteFileRequest {
        id: Some(EntityId { value: file_id }),
    });

    let response = client
        .undelete_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn delete_chosen_file(
    state: tauri::State<'_, AppState>,
    file_id: Vec<String>,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(DeleteChosenFilesRequest {
        file_ids: file_id.into_iter().map(|f| EntityId { value: f }).collect(),
    });

    let response = client
        .delete_chosen_files(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn remove_deleted_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(RemoveDeletedFileRequest {
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .remove_delete_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn move_file(
    state: tauri::State<'_, AppState>,
    folder_id: String,
    file_id: String,
) -> Result<FileView, String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(MoveFileRequest {
        folder_id: Some(EntityId { value: folder_id }),
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .move_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}


#[tauri::command]
pub async fn copy_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
    target_folder_id: String,
) -> Result<FileView, String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(CopyFileRequest {
        file_id: Some(EntityId { value: file_id }),
        target_folder_id: Some(EntityId { value: target_folder_id }),
    });

    let response = client
        .copy_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn archive_file (
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<(), String> {
    println!("arhciving file: {}", file_id);
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(ArchiveFileRequest {
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .archive_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn unarchive_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(UnarchiveFileRequest {
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .unarchive_file(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn search_files(
    state: tauri::State<'_, AppState>,
    name: Option<String>,
    label_ids: Vec<String>,
    updated_after: Option<i64>,
    updated_before: Option<i64>,
) -> Result<Vec<FileView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(SearchFilesRequest {
        name,
        label_ids: label_ids
            .into_iter()
            .map(|value| EntityId { value })
            .collect(),
        updated_after: updated_after.map(|seconds| prost_types::Timestamp { seconds, nanos: 0 }),
        updated_before: updated_before.map(|seconds| prost_types::Timestamp { seconds, nanos: 0 }),
    });

    let response = client
        .search_files(request)
        .await
        .map_err(|e| grpc_message(&e))?;

    let files = response
        .into_inner()
        .files
        .into_iter()
        .map(map_file_proto_to_view)
        .collect();

    Ok(files)
}
