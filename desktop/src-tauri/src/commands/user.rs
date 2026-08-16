use crate::helpers::grpc_error::grpc_message;
use crate::helpers::auth_token::auth_token;
use crate::helpers::with_auth::with_auth;
use crate::types::model::UserProfileView;
use crate::user::user_service_client::UserServiceClient;
use crate::user::{GetUserByIdRequest, UpdateProfileRequest};
use crate::utils::to_unix;
use crate::AppState;
use tauri::State;
use tonic::Request;

#[tauri::command]
pub async fn get_user_profile(state: State<'_, AppState>) -> Result<UserProfileView, String> {
    let token = auth_token(&state).await?;

    let mut client =
        UserServiceClient::with_interceptor(state.user_grpc_channel.clone(), with_auth(token));

    // Identity is derived from the auth token on the backend; no id is sent.
    let request = Request::new(GetUserByIdRequest { id: None });

    let response = client.get_by_id(request).await.map_err(|e| grpc_message(&e));

    let user_data = response?.into_inner();

    Ok(UserProfileView {
        id: user_data.id.unwrap().value,
        email: user_data.email,
        name: user_data.full_name,
        role: match user_data.role {
            0 => "User".to_string(),
            1 => "Admin".to_string(),
            _ => "Unknown".to_string(),
        },
        created_at: to_unix(user_data.created_at),
    })
}

#[tauri::command]
pub async fn get_user_count(state: State<'_, AppState>) -> Result<i64, String> {
    let token = auth_token(&state).await?;

    let mut client =
        UserServiceClient::with_interceptor(state.user_grpc_channel.clone(), with_auth(token));

    let res = client.get_user_count(Request::new(())).await.map_err(|e| grpc_message(&e))?;

    Ok(res.into_inner().count)
}

/// Renames the signed-in user. Identity is taken from the token on the backend, so no
/// id is sent and a caller can only ever rename themselves.
#[tauri::command]
pub async fn update_user_name(
    state: State<'_, AppState>,
    full_name: String,
) -> Result<UserProfileView, String> {
    let token = auth_token(&state).await?;

    let mut client =
        UserServiceClient::with_interceptor(state.user_grpc_channel.clone(), with_auth(token));

    let response = client
        .update_profile(Request::new(UpdateProfileRequest { full_name }))
        .await
        .map_err(|e| grpc_message(&e))?;

    let user = response.into_inner();

    Ok(UserProfileView {
        id: user.id.map(|i| i.value).unwrap_or_default(),
        email: user.email,
        name: user.full_name,
        role: match user.role {
            0 => "User".to_string(),
            1 => "Admin".to_string(),
            _ => "Unknown".to_string(),
        },
        created_at: to_unix(user.created_at),
    })
}
