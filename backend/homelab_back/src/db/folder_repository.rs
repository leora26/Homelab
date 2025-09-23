use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::folder::Folder;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait FolderRepository: Send + Sync {
    async fn get_root(&self, user_id: &Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_by_id(&self, folder_id: &Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_children_by_id(&self, folder_id: &Uuid) -> Result<Vec<Folder>, DataError>;
    async fn delete_by_id(&self, folder_id: &Uuid) -> Result<(), DataError>;
}

pub struct FolderRepositoryImpl {
    pool: PgPool,
}

impl FolderRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FolderRepository for FolderRepositoryImpl {
    async fn get_root(&self, user_id: &Uuid) -> Result<Option<Folder>, DataError> {
        let folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent_folder_id, name, owner_id, created_at FROM folders WHERE parent_folder_id IS NULL AND owner_id = $1",
        user_id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folder)
    }

    async fn get_by_id(&self, folder_id: &Uuid) -> Result<Option<Folder>, DataError> {
        let folder = sqlx::query_as!(
        Folder,
        "SELECT id, parent_folder_id, name, owner_id, created_at FROM folders WHERE id = $1",
        folder_id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folder)
    }

    async fn get_children_by_id(&self, folder_id: &Uuid) -> Result<Vec<Folder>, DataError> {
        let folders = sqlx::query_as!(
        Folder,
        "SELECT id, parent_folder_id, name, owner_id, created_at FROM folders WHERE parent_folder_id = $1",
        folder_id
    )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folders)
    }

    async fn delete_by_id(&self, folder_id: &Uuid) -> Result<(), DataError> {
        sqlx::query!("DELETE FROM folders WHERE id = $1", folder_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }
}
