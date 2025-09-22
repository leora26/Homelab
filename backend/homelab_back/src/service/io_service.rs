use tokio::fs;
use sqlx::PgPool;
use crate::domain::file::File;
use crate::service::folder_service;
use std::io::Result;

pub async fn upload_file_to_disk(file_content: &Vec<u8>, f: &File, pool: &PgPool) -> Result<()> {
    let path = match folder_service::find_path_to_folder(&f.parent_folder_id, pool).await {
        Ok(p) => p,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Path not found: {}", e)));
        }
    };

    let full_path = format!("{}/{}", path, f.name);

    fs::write(&full_path, file_content).await?;

    Ok(())
}