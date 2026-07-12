use crate::AppState;

// Reads the current access token from state; every NAS gRPC call must carry it,
// otherwise the backend auth interceptor rejects the request. Identity (the owner)
// is derived from this token on the backend, not from any client-supplied id.
pub async fn auth_token(state: &tauri::State<'_, AppState>) -> Result<String, String> {
    let lock = state.access_token.read().await;
    lock.clone().ok_or_else(|| "User is not authenticated".to_string())
}