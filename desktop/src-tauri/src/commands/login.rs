use rand::RngCore;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use tauri::{AppHandle, Emitter, State};
use webbrowser;
use crate::AppState;

#[tauri::command]
pub async fn trigger_login(state: State<'_, AppState>) -> Result<(), String> {
    let mut verifier_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut verifier_bytes);

    let code_verifier = URL_SAFE_NO_PAD.encode(&verifier_bytes);

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash_result = hasher.finalize();

    let code_challenge = URL_SAFE_NO_PAD.encode(&hash_result);

    {
        let mut verifier_lock = state.pkce_verifier.write().await;
        *verifier_lock = Some(code_verifier);
    }

    let client_id = "376372459773100036";
    let redirect_uri = "pavuk%3A%2F%2Fcallback";

    // Zitadel reserved scope: adds the Pavuk NAS project id to the token audience
    // so the backend (which validates `aud` against this project id) accepts the token.
    let project_id = "376372459739348996";
    let scope = format!(
        "openid+profile+email+urn:zitadel:iam:org:project:id:{}:aud",
        project_id
    );

    let auth_url = format!(
        "http://localhost:8085/oauth/v2/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&prompt=login&code_challenge={}&code_challenge_method=S256",
        client_id,
        redirect_uri,
        scope,
        code_challenge
    );

    webbrowser::open(&auth_url).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_auth_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let token_guard = state.access_token.read().await;
    Ok(token_guard.is_some())
}

/// Clears the cached access token (and any in-flight PKCE verifier) and notifies the UI
/// so it returns to the login screen. Combined with `prompt=login` on the auth URL, this
/// lets a user sign out and log in as a different account.
#[tauri::command]
pub async fn logout(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    {
        let mut token_lock = state.access_token.write().await;
        *token_lock = None;
    }
    {
        let mut verifier_lock = state.pkce_verifier.write().await;
        *verifier_lock = None;
    }

    app.emit("auth_state_changed", false)
        .map_err(|e| format!("Failed to emit auth state: {}", e))?;

    Ok(())
}