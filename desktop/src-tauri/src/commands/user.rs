use serde::Serialize;
use crate::common::EntityId;
use crate::user::user_service_client::UserServiceClient;
use crate::user::GetUserByIdRequest;
use crate::AppState;
use tauri::State;
use tonic::Request;
use crate::utils::format_timestamp;

#[derive(Serialize)]
pub struct UserProfileView {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn get_user_profile(
    user_id: String,
    state: State<'_, AppState>,
) -> Result<UserProfileView, String> {
    println!("🦀 [RUST] Received request to fetch user profile for ID: {}", user_id);
    let mut client = UserServiceClient::new(state.user_grpc_channel.clone());

    let request = Request::new(GetUserByIdRequest {
        id: Some(EntityId { value: user_id }),
    });

    let response = client
        .get_by_id(request)
        .await
        .map_err(|e| format!("gRPC error while fetching user details {}", e.message()))?;

    let user_data = response.into_inner();


    Ok(UserProfileView {
        id: user_data.id.unwrap().value,
        email: user_data.email,
        name: user_data.full_name,
        created_at: format_timestamp(user_data.created_at),
    })
}
