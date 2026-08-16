use crate::helpers::grpc_error::grpc_message;
use crate::helpers::auth_token::auth_token;
use crate::helpers::with_auth::with_auth;
use crate::nas::storage_profile_service_client::StorageProfileServiceClient;
use crate::nas::GetStorageProfileByIdRequest;
use crate::types::model::{StorageProfileView, StorageStatsView};
use crate::AppState;
use tonic::Request;

#[tauri::command]
pub async fn get_storage_profile(
    state: tauri::State<'_, AppState>,
) -> Result<StorageProfileView, String> {
    let token = auth_token(&state).await?;

    let mut client = StorageProfileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    // Identity is derived from the auth token on the backend; no id is sent.
    let request = Request::new(GetStorageProfileByIdRequest { id: None });

    let response = client.get_by_id(request).await.map_err(|e| grpc_message(&e))?;

    let sp_data = response.into_inner();

    Ok(StorageProfileView {
        user_id: sp_data.user_id.unwrap().value,
        allowed_storage: sp_data.allowed_storage,
        taken_storage: sp_data.taken_storage,
        is_blocked: sp_data.is_blocked,
    })
}

#[tauri::command]
pub async fn get_storage_stats(
    state: tauri::State<'_, AppState>,
) -> Result<StorageStatsView, String> {
    let token = auth_token(&state).await?;
    let mut client = StorageProfileServiceClient::with_interceptor(
        state.nas_grpc_channel.clone(),
        with_auth(token),
    );

    let res = client
        .get_storage_stats(Request::new(()))
        .await
        .map_err(|e| grpc_message(&e))?;

    let data = res.into_inner();

    Ok(StorageStatsView {
        file_count: data.file_count,
        folder_count: data.folder_count,
        trashed_item_count: data.trashed_item_count,
        trashed_bytes: data.trashed_bytes,
        labelled_file_count: data.labelled_file_count,
        unlabelled_file_count: data.unlabelled_file_count,
        shared_file_count: data.shared_file_count,
        shared_bytes: data.shared_bytes,
    })
}
