use sqlx::PgPool;
use uuid::Uuid;
use crate::db::folder_repository;
use crate::domain::folder::Folder;

pub async fn find_root_folder (pool: &PgPool, user_id: &Uuid) -> Result<Option<Folder>, sqlx::Error> {
    folder_repository::find_root_folder(user_id, pool).await
}

pub async fn find_folder_by_id (pool: &PgPool, folder_id: &Uuid) -> Result<Option<Folder>, sqlx::Error> {
    folder_repository::find_folder_by_id(folder_id, pool).await
}

pub async fn find_all_children_folder (pool: &PgPool, folder_id: &Uuid) -> Result<Vec<Folder>, sqlx::Error> {
    folder_repository::find_all_children_folders(pool, folder_id).await
}

