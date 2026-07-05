use crate::AppState;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use futures_util::StreamExt;
use tauri::{AppHandle, Manager, State};
use tokio::io::AsyncWriteExt;

// The nas-server actix REST server that streams file download/preview content.
const NAS_REST_BASE: &str = "http://127.0.0.1:8080";

async fn access_token(state: &State<'_, AppState>) -> Result<String, String> {
    let lock = state.access_token.read().await;
    lock.clone()
        .ok_or_else(|| "User is not authenticated".to_string())
}

/// Downloads a file from the authenticated REST endpoint and saves it to the OS
/// downloads directory. The bearer token lives in Rust state, so the request must
/// be made here rather than via a browser `<a>` link (which cannot send the token).
///
/// The response body is streamed chunk-by-chunk directly to disk, so the whole file
/// is never held in memory. On any stream/write error the partial file is removed.
///
/// Returns the absolute path the file was saved to.
#[tauri::command]
pub async fn download_file(
    app: AppHandle,
    state: State<'_, AppState>,
    file_id: String,
    file_name: String,
) -> Result<String, String> {
    let token = access_token(&state).await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/files/{}/download", NAS_REST_BASE, file_id))
        .query(&[("name", &file_name)])
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Download request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()));
    }

    let download_dir = app
        .path()
        .download_dir()
        .map_err(|e| format!("Could not resolve downloads directory: {}", e))?;

    let destination = download_dir.join(&file_name);

    let mut file = tokio::fs::File::create(&destination)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;

    // Stream the body straight to disk instead of buffering it all in memory.
    let write_result = async {
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Download stream error: {}", e))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| format!("Failed to write to file: {}", e))?;
        }
        file.flush()
            .await
            .map_err(|e| format!("Failed to flush file: {}", e))?;
        Ok::<(), String>(())
    }
    .await;

    if let Err(e) = write_result {
        // Don't leave a truncated/corrupt file behind.
        let _ = tokio::fs::remove_file(&destination).await;
        return Err(e);
    }

    Ok(destination.to_string_lossy().to_string())
}

/// Fetches a file's preview from the authenticated REST endpoint and returns it as a
/// `data:` URL, so the webview can render it in an `<img>` element (which cannot send
/// an Authorization header on its own). Previews are small, so buffering here is fine.
#[tauri::command]
pub async fn get_file_preview(
    state: State<'_, AppState>,
    file_id: String,
) -> Result<String, String> {
    let token = access_token(&state).await?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/files/{}/preview", NAS_REST_BASE, file_id))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Preview request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Preview failed: HTTP {}", response.status()));
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read preview body: {}", e))?;

    let encoded = STANDARD.encode(&bytes);

    Ok(format!("data:{};base64,{}", content_type, encoded))
}
