use tauri::State;
use crate::AppState;

pub async fn access_token(state: &State<'_, AppState>) -> Result<String, String> {
    let lock = state.access_token.read().await;
    lock.clone()
        .ok_or_else(|| "User is not authenticated".to_string())
}
