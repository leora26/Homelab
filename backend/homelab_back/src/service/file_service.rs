use sqlx::PgPool;
use uuid::Uuid;
use crate::db::file_repository;
use crate::domain::file::File;

pub async fn get_file_by_id (pool: &PgPool, file_id: &Uuid)
                             -> Result<Option<File>, sqlx::Error>{
    file_repository::get_file_by_id(pool, file_id).await
}

pub async fn get_files_by_folder(pool: &PgPool, folder_id: &Uuid)
-> Result<Vec<File>, sqlx::Error> {
    file_repository::get_files_by_folder_id(pool, folder_id).await
}