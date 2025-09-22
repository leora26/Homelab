use async_recursion::async_recursion;
use sqlx::PgPool;
use tracing_subscriber::fmt::format;
use uuid::Uuid;
use crate::db::folder_repository;
use crate::domain::folder::Folder;
use crate::exception::data_error::DataError;

pub async fn find_root_folder (pool: &PgPool, user_id: &Uuid) -> Result<Option<Folder>, sqlx::Error> {
    folder_repository::find_root_folder(user_id, pool).await
}

pub async fn find_folder_by_id (pool: &PgPool, folder_id: &Uuid) -> Result<Option<Folder>, sqlx::Error> {
    folder_repository::find_folder_by_id(folder_id, pool).await
}

pub async fn find_all_children_folder (pool: &PgPool, folder_id: &Uuid) -> Result<Vec<Folder>, sqlx::Error> {
    folder_repository::find_all_children_folders(pool, folder_id).await
}

pub async fn delete_folder (pool: &PgPool, folder_id: &Uuid) -> Result<(), sqlx::Error> {
    folder_repository::delete_folder_by_id(pool, folder_id).await
}

pub async fn find_path_to_folder(folder_id: &Uuid, pool: &PgPool) -> Result<String, DataError> {
    let path = get_parent_folder_name(folder_id, pool).await?;
    Ok(path)
}

#[async_recursion]
async fn get_parent_folder_name (f_id: &Uuid, p: &PgPool) -> Result<String, DataError> {
    let f = folder_repository::find_folder_by_id(f_id, p)
        .await
        .map_err(DataError::DatabaseError)?
        .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

    if let Some(parent_id) = f.parent_folder_id{

        let parent_path = get_parent_folder_name(&parent_id, p).await?;

        Ok(format!("{}/{}", parent_path, f.name))

    } else {
        Ok(f.name)
    }
}