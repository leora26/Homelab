use crate::common::EntityId;
use crate::nas::folder_service_client::FolderServiceClient;
use crate::nas::{GetAllSubfoldersRequest, GetFilesForFolderRequest, GetRootFolderRequest};
use crate::types::model::{FileView, FolderView};
use crate::AppState;
use tonic::Request;
use crate::helpers::mappings::{map_file_proto_to_view, map_folder_proto_to_view};

#[tauri::command]
pub async fn get_root_folder(
    user_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<FolderView, String> {
    let mut client = FolderServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(GetRootFolderRequest {
        user_id: Some(EntityId { value: user_id }),
    });

    let response = client.get_root_folder(request).await.map_err(|e| {
        eprintln!(
            "🛑 gRPC Error Code when fetching root folder: {:?}",
            e.code()
        );
        format!(
            "gRPC error details when fetching root folder: [{:?}] {}",
            e.code(),
            e.message()
        )
    });

    let root_folder = response?.into_inner();

    Ok(map_folder_proto_to_view(root_folder))
}

#[tauri::command]
pub async fn get_files_for_folder(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileView>, String> {
    let mut client = FolderServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(GetFilesForFolderRequest {
        id: Some(EntityId { value: folder_id }),
    });

    let response = client.get_files_for_folder(request).await.map_err(|e| {
        eprintln!("🛑 gRPC Error Code when fetching files: {:?}", e.code());
        format!(
            "gRPC error details when fetching files: [{:?}] {}",
            e.code(),
            e.message()
        )
    });

    let files = response?.into_inner();

    let mapped_files = files.files.into_iter()
        .map(|f| map_file_proto_to_view(f)).collect();

    Ok(mapped_files)
}


#[tauri::command]
pub async fn get_subfolders (
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FolderView>, String> {

    let mut client = FolderServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(GetAllSubfoldersRequest {
        id: Some(EntityId { value: folder_id }),
    });

    let response = client.get_subfolders(request).await.map_err(|e| {
        eprintln!("🛑 gRPC Error Code when fetching subfolders: {:?}", e.code());
        format!(
            "gRPC error details when fetching subfolders: [{:?}] {}",
            e.code(),
            e.message()
        )
    });

    let subfolders = response?.into_inner();

    let mapped_folders = subfolders.folders.into_iter()
        .map(|f| map_folder_proto_to_view(f)).collect();

    Ok(mapped_folders)
}