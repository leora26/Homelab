use crate::helpers::grpc_error::grpc_message;
use crate::common::EntityId;
use crate::helpers::mappings::{map_file_proto_to_view, map_folder_proto_to_view};
use crate::helpers::with_auth::with_auth;
use crate::nas::folder_service_client::FolderServiceClient;
use crate::nas::{CleanUpDeletedFolderRequest, CleanUpTrashRequest, CreateFolderRequest, DeleteFolderRequest, GetAllSubfoldersRequest, GetDeletedFoldersRequest, GetFilesForFolderRequest, GetFolderRequest, GetRootFolderRequest, GetTrashFilesForFolderRequest, GetTrashSubfoldersForFolderRequest, RenameFolderRequest, RestoreDeletedFolderRequest};
use crate::types::model::{FileView, FolderView};
use crate::AppState;
use tonic::Request;
use crate::helpers::auth_token::auth_token;

#[tauri::command]
pub async fn get_root_folder(
    state: tauri::State<'_, AppState>,
) -> Result<FolderView, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = Request::new(GetRootFolderRequest { user_id: None });

    let response = client.get_root_folder(request).await.map_err(|e| grpc_message(&e));

    let root_folder = response?.into_inner();

    Ok(map_folder_proto_to_view(root_folder))
}

#[tauri::command]
pub async fn get_files_for_folder(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(GetFilesForFolderRequest {
        id: Some(EntityId { value: folder_id }),
    });

    let response = client.get_files_for_folder(request).await.map_err(|e| grpc_message(&e));

    let files = response?.into_inner();

    let mapped_files = files
        .files
        .into_iter()
        .map(|f| map_file_proto_to_view(f))
        .collect();

    Ok(mapped_files)
}

#[tauri::command]
pub async fn get_subfolders(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FolderView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = Request::new(GetAllSubfoldersRequest {
        id: Some(EntityId { value: folder_id }),
    });

    let response = client.get_subfolders(request).await.map_err(|e| grpc_message(&e));

    let subfolders = response?.into_inner();

    let mapped_folders = subfolders
        .folders
        .into_iter()
        .map(|f| map_folder_proto_to_view(f))
        .collect();

    Ok(mapped_folders)
}

#[tauri::command]
pub async fn create_folder(
    parent_folder_id: String,
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<FolderView, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = Request::new(CreateFolderRequest {
        parent_folder_id: Some(EntityId {
            value: parent_folder_id,
        }),
        name,
        owner_id: None,
    });

    let response = client.create_folder(request).await.map_err(|e| grpc_message(&e));

    let new_folder = response?.into_inner();

    Ok(map_folder_proto_to_view(new_folder))
}

#[tauri::command]
pub async fn delete_selected_folder(
    selected_folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = tonic::Request::new(DeleteFolderRequest {
        id: Some(EntityId {
            value: selected_folder_id.clone(),
        }),
    });

    client.delete_folder(request).await.map_err(|e| grpc_message(&e))?;

    Ok(())
}

#[tauri::command]
pub async fn cleanup_deleted_folder(
    deleted_folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = tonic::Request::new(CleanUpDeletedFolderRequest {
        folder_id: Some(EntityId {
            value: deleted_folder_id.clone(),
        }),
        user_id: None,
    });

    client.clean_up_deleted_folder(request).await.map_err(|e| grpc_message(&e))?;

    Ok(())
}

#[tauri::command]
pub async fn cleanup_trash(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = tonic::Request::new(CleanUpTrashRequest { user_id: None });

    client.clean_up_trash(request).await.map_err(|e| grpc_message(&e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_deleted_folder(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FolderView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Owner is resolved from the auth token on the backend.
    let request = tonic::Request::new(GetDeletedFoldersRequest { owner_id: None });

    let response = client.get_deleted_folders(request).await.map_err(|e| grpc_message(&e));

    let deleted_folders = response?.into_inner();

    let mapped_folders = deleted_folders
        .folders
        .into_iter()
        .map(|f| map_folder_proto_to_view(f))
        .collect();

    Ok(mapped_folders)
}

#[tauri::command]
pub async fn get_trash_files_by_folder(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = tonic::Request::new(GetTrashFilesForFolderRequest {
        id: Some(EntityId {
            value: folder_id.clone(),
        }),
    });

    let response = client
        .get_trash_files_for_folder(request)
        .await
        .map_err(|e| grpc_message(&e));

    let deleted_files = response?.into_inner();

    let mapped_files = deleted_files
        .files
        .into_iter()
        .map(|f| map_file_proto_to_view(f))
        .collect();

    Ok(mapped_files)
}

#[tauri::command]
pub async fn get_trash_subfolders_by_folder(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FolderView>, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = tonic::Request::new(GetTrashSubfoldersForFolderRequest {
        id: Some(EntityId {
            value: folder_id.clone(),
        }),
    });

    let response = client
        .get_trash_subfolder_for_folder(request)
        .await
        .map_err(|e| grpc_message(&e));

    let deleted_subfolders = response?.into_inner();

    let mapped_subfolders = deleted_subfolders
        .folders
        .into_iter()
        .map(|f| map_folder_proto_to_view(f))
        .collect();

    Ok(mapped_subfolders)
}

#[tauri::command]
pub async fn rename_folder(
    folder_id: String,
    new_name: String,
    state: tauri::State<'_, AppState>,
) -> Result<FolderView, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = tonic::Request::new(RenameFolderRequest {
        id: Some(EntityId { value: folder_id }),
        new_name: new_name.clone(),
    });

    let response = client.rename_folder(request).await.map_err(|e| grpc_message(&e));

    let rename = response?.into_inner();

    Ok(map_folder_proto_to_view(rename))
}

#[tauri::command]
pub async fn restore_folder(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let request = tonic::Request::new(RestoreDeletedFolderRequest {
        folder_id: Some(EntityId {
            value: folder_id.clone(),
        }),
    });

    client.restore_deleted_folder(request).await.map_err(|e| grpc_message(&e))?;

    Ok(())
}


#[tauri::command]
pub async fn get_folder(
    folder_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<FolderView, String> {
    let token = auth_token(&state).await?;
    let mut client = FolderServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );
    
    let req = tonic::Request::new(GetFolderRequest {
        id: Some(EntityId {
            value: folder_id
        })
    });
    
    let res = client.get_folder(req).await.map_err(|e| grpc_message(&e));
    
    Ok(map_folder_proto_to_view(res?.into_inner()))
}