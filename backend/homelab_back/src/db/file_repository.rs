use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::file::File;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait FileRepository: Send + Sync {
    async fn get_by_id(&self, file_id: &Uuid) -> Result<Option<File>, DataError>;
    async fn get_by_folder_id(&self, folder_id: &Uuid) -> Result<Vec<File>, DataError>;
    async fn delete_by_id(&self, file_id: &Uuid) -> Result<(), DataError>;
    async fn upload(&self, file: File) -> Result<File, DataError>;
}

pub struct FileRepositoryImpl {
    pool: PgPool,
}

impl FileRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileRepository for FileRepositoryImpl {
    async fn get_by_id(&self, file_id: &Uuid) -> Result<Option<File>, DataError> {
        let file = sqlx::query_as!(
        File,
        "SELECT id, name, owner_id, parent_folder_id, file_type as \"file_type: _\" FROM files WHERE id = $1",
        file_id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(file)
    }

    async fn get_by_folder_id(&self, folder_id: &Uuid) -> Result<Vec<File>, DataError> {
        let files = sqlx::query_as!(
        File,
        "SELECT id, name, owner_id, parent_folder_id, file_type as \"file_type: _\" FROM files WHERE parent_folder_id = $1",
        folder_id
    )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(files)
    }

    async fn delete_by_id(&self, file_id: &Uuid) -> Result<(), DataError> {
        sqlx::query!("DELETE FROM files WHERE id = $1", file_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn upload(&self, file: File) -> Result<File, DataError> {
        let file = sqlx::query_as!(
        File,
        "INSERT INTO files (id, name, owner_id, parent_folder_id, file_type) \
        VALUES ($1,$2, $3, $4, $5::file_type) RETURNING id, name, owner_id,parent_folder_id, file_type as \"file_type: _\"",
        file.id,
        file.name,
        file.owner_id,
        file.parent_folder_id,
        file.file_type as _
    )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(file)
    }
}