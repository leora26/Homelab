use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use std::{env, fs};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let storage_dir_str =
        env::var("ROOT_FOLDER_PATH").expect("ROOT_FOLDER_PATH must be set in .env file");
    let storage_dir = Path::new(&storage_dir_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    let user_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, full_name, password_hash, role)
        VALUES ($1, $2, $3, $4, 'admin')
        "#,
        user_id,
        "admin@homelab.local",
        "Admin User",
        "hashed_password"
    )
    .execute(&pool)
    .await?;
    println!("✅ Inserted User: Admin");

    let root_folder_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO folders (id, name, owner_id, parent_folder_id)
        VALUES ($1, $2, $3, NULL)
        "#,
        root_folder_id,
        "Root",
        user_id
    )
    .execute(&pool)
    .await?;
    println!("✅ Inserted Root Folder");

    let file_id = Uuid::new_v4();
    let file_content = b"Hello, this is a test file for the Homelab NAS!";
    let file_size = file_content.len() as i64;

    sqlx::query!(
        r#"
        INSERT INTO files (id, name, owner_id, file_type, parent_folder_id, size, upload_status)
        VALUES ($1, $2, $3, 'Text', $4, $5, 'Completed')
        "#,
        file_id,
        "test_document.txt",
        user_id,
        root_folder_id,
        file_size
    )
    .execute(&pool)
    .await?;

    let physical_file_path = storage_dir.join(file_id.to_string());
    fs::write(physical_file_path, file_content)?;

    println!("✅ Inserted File into DB and wrote to D:/Homelab/storage");

    let allowed_storage = file_size * 20;
    let taken_storage = file_size;

    sqlx::query!(
        r#"
    INSERT INTO storage_profiles (user_id, allowed_storage, taken_storage, is_blocked)
    VALUES ($1, $2, $3, $4)
    "#,
        user_id,
        allowed_storage,
        taken_storage,
        false
    )
    .execute(&pool)
    .await?;

    println!(
        "✅ Inserted Storage Profile: {} bytes allowed (File uses 5%)",
        allowed_storage
    );
    println!("🎉 Seeding Complete!");
    Ok(())
}
