use crate::types::model::UserProfileView;
use crate::user::user_service_client::UserServiceClient;
use crate::user::GetUserByIdRequest;
use crate::utils::format_timestamp;
use crate::AppState;
use tauri::State;
use tonic::Request;
use crate::helpers::with_auth::with_auth;

#[tauri::command]
pub async fn get_user_profile(
    state: State<'_, AppState>,
) -> Result<UserProfileView, String> {

    let token = {
        let lock = state.access_token.read().await;
        lock.clone().ok_or("User is not authenticated")?
    };

    let mut client = UserServiceClient::with_interceptor(
        state.user_grpc_channel.clone(),
        with_auth(token)
    );

    // Identity is derived from the auth token on the backend; no id is sent.
    let request = Request::new(GetUserByIdRequest { id: None });

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
