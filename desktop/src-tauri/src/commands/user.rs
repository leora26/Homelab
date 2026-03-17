use crate::common::EntityId;
use crate::types::model::UserProfileView;
use crate::user::user_service_client::UserServiceClient;
use crate::user::GetUserByIdRequest;
use crate::utils::format_timestamp;
use crate::AppState;
use tauri::State;
use tonic::Request;

#[tauri::command]
pub async fn get_user_profile(
    user_id: String,
    state: State<'_, AppState>,
) -> Result<UserProfileView, String> {
    println!(
        "🦀 [RUST] Received request to fetch user profile for ID: {}",
        user_id
    );
    let mut client = UserServiceClient::new(state.user_grpc_channel.clone());

    let request = Request::new(GetUserByIdRequest {
        id: Some(EntityId { value: user_id }),
    });

    let response = client.get_by_id(request).await.map_err(|e| {
        eprintln!(
            "🛑 gRPC Error Code when fetching user details: {:?}",
            e.code()
        );
        format!(
            "gRPC error details when fetching user details: [{:?}] {}",
            e.code(),
            e.message()
        )
    });

    let user_data = response?.into_inner();

    Ok(UserProfileView {
        id: user_data.id.unwrap().value,
        email: user_data.email,
        name: user_data.full_name,
        created_at: format_timestamp(user_data.created_at),
    })
}
