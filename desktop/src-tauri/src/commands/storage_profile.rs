use serde::Serialize;
use tonic::Request;
use crate::AppState;
use crate::common::EntityId;
use crate::nas::GetStorageProfileByIdRequest;
use crate::nas::storage_profile_service_client::StorageProfileServiceClient;

#[derive(Serialize)]
pub struct StorageProfileView {
    pub user_id: String,
    pub allowed_storage: i64,
    pub taken_storage: i64,
    pub is_blocked: bool,
}

#[tauri::command]
pub async fn get_storage_profile(
    user_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<StorageProfileView, String> {
    let mut client = StorageProfileServiceClient::new(state.nas_grpc_channel.clone());

    let request = Request::new(GetStorageProfileByIdRequest {
        id: Some(EntityId {value: user_id}),
    });

    let response = client
        .get_by_id(request)
        .await
        .map_err(|e| {
            eprintln!("🛑 gRPC Error Code: {:?}", e.code());
            format!("gRPC error details: [{:?}] {}", e.code(), e.message())
        })?;

    let sp_data = response.into_inner();

    Ok(StorageProfileView {
        user_id: sp_data.user_id.unwrap().value,
        allowed_storage: sp_data.allowed_storage,
        taken_storage: sp_data.taken_storage,
        is_blocked: sp_data.is_blocked,
    })
}