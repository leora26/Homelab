use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use std::{env, fs};
use uuid::Uuid;
use serde_json::json;
use reqwest::StatusCode;
use homelab_core::auth::diplomat::ZitadelDiplomat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // --- 1. SETUP DB & FOLDERS ---
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let storage_dir_str = env::var("ROOT_FOLDER_PATH").expect("ROOT_FOLDER_PATH must be set in .env file");
    let storage_dir = Path::new(&storage_dir_str);

    if !storage_dir.exists() {
        fs::create_dir_all(storage_dir)?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5) // Protects your laptop's memory from connection spam
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // --- 2. AUTHENTICATE WITH ZITADEL ---
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let key_path = format!("{}/backend-key.json", manifest_dir);

    println!("Looking for key at: {}", key_path);

    let diplomat = ZitadelDiplomat::new(
        "http://localhost:8085".to_string(),
        &key_path
    ).await.expect("Failed to initialize Zitadel Diplomat");

    let access_token = diplomat.get_token().await?;

    // --- 3. STEP ONE: CREATE USER ---
    // We borrow this client multiple times below to utilize connection pooling
    let client = reqwest::Client::new();
    let email = "testUser@homelab.local";
    let password = "PavukTestUser@2026!";

    let zitadel_payload = json!({
        "userName": email,
        "profile": {
            "firstName": "Test",
            "lastName": "User",
            "displayName": "Test User"
        },
        "email": {
            "email": email,
            "isEmailVerified": true
        },
        // Must be a flat string to pass the gRPC type checker
        "initialPassword": password
    });

    let res = client.post("http://localhost:8085/management/v1/users/human")
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&zitadel_payload)
        .send()
        .await?;

    let zitadel_id = if res.status() == StatusCode::CONFLICT {
        println!("⚠️ User already exists in Zitadel. Fetching existing ID...");

        let search_payload = json!({
            "queries": [{"userNameQuery": {"userName": email, "method": "TEXT_QUERY_METHOD_EQUALS"}}]
        });

        let search_res = client.post("http://localhost:8085/management/v1/users/_search")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&search_payload)
            .send()
            .await?;

        let search_data: serde_json::Value = search_res.json().await?;

        search_data["result"][0]["id"]
            .as_str()
            .ok_or("Could not parse existing userId from Zitadel search response")?
            .to_string()
    } else if res.status().is_success() {
        let response_data: serde_json::Value = res.json().await?;
        response_data["userId"]
            .as_str()
            .ok_or("Could not parse userId from Zitadel response")?
            .to_string()
    } else {
        let err = res.text().await?;
        return Err(format!("Failed to create user in Zitadel: {}", err).into());
    };

    println!("✅ Resolved Zitadel User ID: {}. Securing permanent password...", zitadel_id);

    // --- 3.5 STEP TWO: LOCK THE PASSWORD (Bypass the setup screen) ---
    let password_payload = json!({
        "password": password,
        "noChangeRequired": true // This is the magic flag!
    });

    let pw_res = client.post(format!("http://localhost:8085/management/v1/users/{}/password", zitadel_id))
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&password_payload)
        .send()
        .await?;

    if !pw_res.status().is_success() {
        let err = pw_res.text().await?;
        return Err(format!("Failed to lock permanent password: {}", err).into());
    }

    // --- 4. CREATE OR UPDATE USER IN LOCAL DB ---
    let user_record = sqlx::query!(
        r#"
        INSERT INTO users (id, external_id, email, full_name, role)
        VALUES ($1, $2, $3, $4, 'admin')
        ON CONFLICT (email) DO UPDATE
        SET external_id = EXCLUDED.external_id
        RETURNING id
        "#,
        Uuid::new_v4(),
        zitadel_id,
        email,
        "Test User"
    )
        .fetch_one(&pool)
        .await?;

    let local_user_id = user_record.id;
    println!("✅ Synced User into local DB with internal ID: {}", local_user_id);

    // --- 5. CREATE FOLDER STRUCTURE & STORAGE PROFILE ---
    sqlx::query!(
        r#"
        INSERT INTO folders (id, name, owner_id, parent_folder_id)
        VALUES ($1, $2, $3, NULL)
        ON CONFLICT DO NOTHING
        "#,
        Uuid::new_v4(), "Root", local_user_id
    ).execute(&pool).await?;

    let allowed_storage: i64 = 100 * 1024 * 1024;
    sqlx::query!(
        r#"
        INSERT INTO storage_profiles (user_id, allowed_storage, taken_storage, is_blocked, external_id)
        VALUES ($1, $2, $3, false, $4)
        ON CONFLICT (user_id) DO NOTHING
        "#,
        local_user_id,
        allowed_storage,
        0i64,
        zitadel_id
    ).execute(&pool).await?;

    println!("✅ Ensured Storage Profile and Root Folder exist");
    println!("🚀 Seeding Complete! You can now log into Tauri with {} / {}", email, password);

    Ok(())
}