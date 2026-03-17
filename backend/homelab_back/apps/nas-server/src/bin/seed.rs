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
    if (!storage_dir.exists()) {
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

    // Level 1: Documents & Media (inside Root)
    let docs_folder_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO folders (id, name, owner_id, parent_folder_id) VALUES ($1, $2, $3, $4)",
        docs_folder_id, "Documents", user_id, root_folder_id
    ).execute(&pool).await?;

    let media_folder_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO folders (id, name, owner_id, parent_folder_id) VALUES ($1, $2, $3, $4)",
        media_folder_id, "Media", user_id, root_folder_id
    ).execute(&pool).await?;

    // Level 2: Work (inside Documents) & Photos (inside Media)
    let work_folder_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO folders (id, name, owner_id, parent_folder_id) VALUES ($1, $2, $3, $4)",
        work_folder_id, "Work", user_id, docs_folder_id
    ).execute(&pool).await?;

    let photos_folder_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO folders (id, name, owner_id, parent_folder_id) VALUES ($1, $2, $3, $4)",
        photos_folder_id, "Photos", user_id, media_folder_id
    ).execute(&pool).await?;

    println!("✅ Inserted Folder Tree (Root -> Docs -> Work | Root -> Media -> Photos)");

    // --- 2. CREATE FILES & TRACK STORAGE ---
    let mut total_taken_storage = 0i64;

    // File 1: In Root
    let f1_id = Uuid::new_v4();
    let f1_content = b"Hello, this is a test file for the Homelab NAS!";
    let f1_size = f1_content.len() as i64;
    total_taken_storage += f1_size;
    sqlx::query!(
        "INSERT INTO files (id, name, owner_id, file_type, parent_folder_id, size, upload_status) VALUES ($1, $2, $3, 'text', $4, $5, 'completed')",
        f1_id, "test_document.txt", user_id, root_folder_id, f1_size
    ).execute(&pool).await?;
    fs::write(storage_dir.join(f1_id.to_string()), f1_content)?;

    // File 2: In Documents
    let f2_id = Uuid::new_v4();
    let f2_content = b"month,amount\njan,1000\nfeb,1250\nmar,900";
    let f2_size = f2_content.len() as i64;
    total_taken_storage += f2_size;
    sqlx::query!(
        "INSERT INTO files (id, name, owner_id, file_type, parent_folder_id, size, upload_status) VALUES ($1, $2, $3, 'text', $4, $5, 'completed')",
        f2_id, "budget_2026.csv", user_id, docs_folder_id, f2_size
    ).execute(&pool).await?;
    fs::write(storage_dir.join(f2_id.to_string()), f2_content)?;

    // File 3: In Work
    let f3_id = Uuid::new_v4();
    let f3_content = b"# Project Apollo\nDeadline is this Friday. Do not forget the TPS reports.";
    let f3_size = f3_content.len() as i64;
    total_taken_storage += f3_size;
    sqlx::query!(
        "INSERT INTO files (id, name, owner_id, file_type, parent_folder_id, size, upload_status) VALUES ($1, $2, $3, 'text', $4, $5, 'completed')",
        f3_id, "project_notes.md", user_id, work_folder_id, f3_size
    ).execute(&pool).await?;
    fs::write(storage_dir.join(f3_id.to_string()), f3_content)?;

    // File 4: In Photos (Simulating an Image)
    let f4_id = Uuid::new_v4();
    let f4_content = b"fake_binary_image_data_here_892347923847";
    let f4_size = f4_content.len() as i64;
    total_taken_storage += f4_size;
    sqlx::query!(
        "INSERT INTO files (id, name, owner_id, file_type, parent_folder_id, size, upload_status) VALUES ($1, $2, $3, 'image', $4, $5, 'completed')",
        f4_id, "vacation.jpg", user_id, photos_folder_id, f4_size
    ).execute(&pool).await?;
    fs::write(storage_dir.join(f4_id.to_string()), f4_content)?;

    println!("✅ Inserted Files into DB and wrote to storage drive");

    // --- 3. CREATE STORAGE PROFILE ---

    // Let's give the user 100 MB of total allowed space
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