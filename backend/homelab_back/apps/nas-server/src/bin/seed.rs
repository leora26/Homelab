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

    // Create physical storage directory if it doesn't exist
    if !storage_dir.exists() {
        fs::create_dir_all(storage_dir)?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // OPTIONAL: Clear the database before seeding to prevent duplicate data errors
    // Uncomment the line below if you want a totally fresh slate every time you run this!
    // sqlx::query!("TRUNCATE storage_profiles, files, folders, users CASCADE").execute(&pool).await?;

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

    // --- 1. CREATE FOLDER STRUCTURE ---

    // Root
    let root_folder_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO folders (id, name, owner_id, parent_folder_id) VALUES ($1, $2, $3, NULL)",
        root_folder_id, "Root", user_id
    ).execute(&pool).await?;

    println!("✅ Inserted Root Folder");

    // --- 2. CREATE STORAGE PROFILE ---

    // Setting starting storage to 0 bytes
    let total_taken_storage = 0i64;

    // Giving the user 100 MB of total allowed space (Note: 100 * 512 is technically ~50KB,
    // if you meant 100 Megabytes, you might want: 100 * 1024 * 1024)
    let allowed_storage: i64 = 100 * 512;

    sqlx::query!(
        r#"
        INSERT INTO storage_profiles (user_id, allowed_storage, taken_storage, is_blocked)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        allowed_storage,
        total_taken_storage,
        false
    )
        .execute(&pool)
        .await?;

    println!(
        "✅ Inserted Storage Profile: {} bytes allowed, {} bytes taken",
        allowed_storage, total_taken_storage
    );
    println!("🎉 Seeding Complete!");

    Ok(())
}