use crate::common::EntityId;
use crate::helpers::mappings::map_file_proto_to_view;
use crate::nas::file_chunk::Data;
use crate::nas::file_service_client::FileServiceClient;
use crate::nas::{
    DeleteChosenFilesRequest, DeleteFileRequest, FileChunk, GetDeletedFilesRequest,
    InitFileRequest, MoveFileRequest, RemoveAllDeletedFilesRequest, RemoveDeletedFileRequest,
    RenameFileRequest, UndeleteFileRequest,
};
use crate::types::model::FileView;
use crate::AppState;
use async_stream::stream;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tonic::Request;

#[tauri::command]
pub async fn init_file(
    state: tauri::State<'_, AppState>,
    name: String,
    destination: String,
    owner_id: String,
    local_path: String,
    is_global: bool,
) -> Result<FileView, String> {
    let metadata = fs::metadata(&local_path)
        .await
        .map_err(|e| format!("Failed to read file metadata for size: {}", e))?;

    let size = metadata.len() as i64;

    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(InitFileRequest {
        name,
        destination: Some(EntityId { value: destination }),
        owner_id: Some(EntityId { value: owner_id }),
        size,
        is_global,
    });

    let response = client
        .init_file(request)
        .await
        .map_err(|e| format!("gRPC InitFile failed: {}", e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn upload_content(
    state: tauri::State<'_, AppState>,
    file_id: String,
    local_path: String,
) -> Result<(), String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let mut file = File::open(&local_path)
        .await
        .map_err(|e| format!("open file failed: {}", e))?;

    let outbound_stream = stream! {
        yield FileChunk {
            data: Some(Data::FileId(EntityId { value: file_id.clone() })),
        };

        let mut buffer = vec![0; 64 * 1024];

        loop {
            match file.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => {
                    yield FileChunk {
                        data: Some(Data::Content(buffer[..n].to_vec())),
                    };
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
        .map_err(|e| format!("Upload stream failed: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_file(state: tauri::State<'_, AppState>, file_id: String) -> Result<(), String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(DeleteFileRequest {
        id: Some(EntityId {
            value: file_id.clone(),
        }),
    });

    let response = client
        .delete_file(request)
        .await
        .map_err(|e| format!("Delete file failed: {}", e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn rename_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
    new_name: String,
) -> Result<FileView, String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(RenameFileRequest {
        id: Some(EntityId { value: file_id }),
        new_name: new_name.clone(),
    });

    let response = client
        .rename_file(request)
        .await
        .map_err(|e| format!("Rename file failed: {}", e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn get_deleted_files(
    state: tauri::State<'_, AppState>,
    user_id: String,
) -> Result<Vec<FileView>, String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(GetDeletedFilesRequest {
        user_id: Some(EntityId { value: user_id }),
    });

    let response = client
        .get_deleted_files(request)
        .await
        .map_err(|e| format!("Get deleted files failed: {}", e))?;

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
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(UndeleteFileRequest {
        id: Some(EntityId { value: file_id }),
    });

    let response = client
        .undelete_file(request)
        .await
        .map_err(|e| format!("Undelete file failed: {}", e))?;

    let file_resp = response.into_inner();

    Ok(map_file_proto_to_view(file_resp))
}

#[tauri::command]
pub async fn delete_chosen_file(
    state: tauri::State<'_, AppState>,
    file_id: Vec<String>,
) -> Result<(), String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(DeleteChosenFilesRequest {
        file_ids: file_id.into_iter().map(|f| EntityId { value: f }).collect(),
    });

    let response = client
        .delete_chosen_files(request)
        .await
        .map_err(|e| format!("Delete chosen files failed: {}", e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn empty_trash(state: tauri::State<'_, AppState>, user_id: String) -> Result<(), String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(RemoveAllDeletedFilesRequest {
        user_id: Some(EntityId { value: user_id }),
    });

    let response = client
        .remove_all_deleted_files(request)
        .await
        .map_err(|e| format!("Remove all deleted files failed: {}", e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn remove_deleted_file(
    state: tauri::State<'_, AppState>,
    file_id: String,
) -> Result<(), String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(RemoveDeletedFileRequest {
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .remove_delete_file(request)
        .await
        .map_err(|e| format!("Remove delete file failed: {}", e))?;

    let _ = response.into_inner();

    Ok(())
}

#[tauri::command]
pub async fn move_file(
    state: tauri::State<'_, AppState>,
    folder_id: String,
    file_id: String,
) -> Result<FileView, String> {
    let mut client = FileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(MoveFileRequest {
        folder_id: Some(EntityId { value: folder_id }),
        file_id: Some(EntityId { value: file_id }),
    });

    let response = client
        .move_file(request)
        .await
        .map_err(|e| format!("Move file failed: {}", e))?;
    
    let file_resp = response.into_inner();
    
    Ok(map_file_proto_to_view(file_resp))
}
