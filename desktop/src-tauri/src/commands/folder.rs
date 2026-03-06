use crate::common::EntityId;
use crate::nas::folder_service_client::FolderServiceClient;
use crate::nas::{GetFilesForFolderRequest, GetRootFolderRequest};
use crate::types::model::{FileView, FolderView};
use crate::utils::format_timestamp;
use crate::AppState;
use tonic::Request;
use crate::helpers::mappings::map_file_proto_to_view;

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

    Ok(FolderView {
        id: root_folder.id.map(|i| i.value).unwrap_or_default(),

        parent_folder_id: root_folder.parent_folder_id.map(|id| id.value),

        name: root_folder.name,

        owner_id: root_folder.owner_id.map(|i| i.value).unwrap_or_default(),
        created_at: format_timestamp(root_folder.created_at),
    })
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
