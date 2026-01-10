use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use homelab_core::file::{File, FileType};
use homelab_core::folder::Folder;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FolderRepository: Send + Sync {
    async fn get_root(&self, user_id: Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_by_id(&self, folder_id: Uuid) -> Result<Option<Folder>, DataError>;
    async fn get_children_by_id(&self, folder_id: Uuid) -> Result<Vec<Folder>, DataError>;
    async fn search_by_name(&self, search_query: String) -> Result<Vec<Folder>, DataError>;
    async fn filter_files_in_folder(&self, file_types: &[FileType], folder_id: Uuid) -> Result<Vec<File>, DataError>;
    async fn get_by_folder_id(&self, folder_id: Uuid) -> Result<Vec<File>, DataError>;
    async fn create(&self, folder: Folder) -> Result<Folder, DataError>;
    async fn update_folder(&self, folder: Folder) -> Result<Folder, DataError>;
    async fn delete_all(&self, folder_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete_by_id(&self, folder_id: Uuid) -> Result<(), DataError>;
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
    async fn get_root(&self, user_id: Uuid) -> Result<Option<Folder>, DataError> {
        let folder = sqlx::query_as!(
        Folder,
        r#"
        SELECT id, parent_folder_id, name, owner_id, created_at
        FROM folders
        WHERE parent_folder_id IS NULL AND owner_id = $1"#,
        user_id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folder)
    }

    async fn get_by_id(&self, folder_id: Uuid) -> Result<Option<Folder>, DataError> {
        let folder = sqlx::query_as!(
        Folder,
        r#"
        SELECT id, parent_folder_id, name, owner_id, created_at
        FROM folders
        WHERE id = $1"#,
        folder_id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folder)
    }

    async fn get_children_by_id(&self, folder_id: Uuid) -> Result<Vec<Folder>, DataError> {
        let folders = sqlx::query_as!(
        Folder,
        r#"
        SELECT id, parent_folder_id, name, owner_id, created_at
        FROM folders
        WHERE parent_folder_id = $1"#,
        folder_id
    )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folders)
    }

    async fn search_by_name(&self, search_query: String) -> Result<Vec<Folder>, DataError> {
        let f: Vec<Folder> = sqlx::query_as!(
            Folder,
            r#"
            SELECT id, name, owner_id, created_at, parent_folder_id
            FROM folders
            WHERE LOWER(name) LIKE LOWER($1)
            "#,
            search_query
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(f)
    }

    async fn filter_files_in_folder(&self, file_types: &[FileType], folder_id: Uuid) -> Result<Vec<File>, DataError> {
        let files = sqlx::query_as!(
            File,
            r#"
            SELECT id, name, owner_id, parent_folder_id, file_type as "file_type: _", is_deleted, ttl, size, upload_status as "upload_status: _"
            FROM files
            WHERE parent_folder_id = $1 AND file_type = ANY($2::file_type[]) AND is_deleted = FALSE
            "#,
            folder_id,
            file_types as &[FileType]
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(files)
    }

    async fn get_by_folder_id(&self, folder_id: Uuid) -> Result<Vec<File>, DataError> {
        let files = sqlx::query_as!(
        File,
        r#"
        SELECT id, name, owner_id, parent_folder_id, file_type as "file_type: _", is_deleted, ttl, size, upload_status as "upload_status: _"
        FROM files
        WHERE parent_folder_id = $1 AND is_deleted = FALSE
        "#,
        folder_id
    )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(files)
    }

    async fn create(&self, folder: Folder) -> Result<Folder, DataError> {
        let folder = sqlx::query_as!(
            Folder,
            r#"
            INSERT INTO folders (id, name, owner_id, created_at, parent_folder_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, owner_id, created_at, parent_folder_id
            "#,
            folder.id,
            folder.name,
            folder.owner_id,
            folder.created_at,
            folder.parent_folder_id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(folder)
    }

    async fn update_folder(&self, folder: Folder) -> Result<Folder, DataError> {
        let f = sqlx::query_as!(
            Folder,
            r#"
            UPDATE folders
            SET name = $1, owner_id = $2, parent_folder_id = $3
            WHERE id = $4
            RETURNING id, name, owner_id, created_at, parent_folder_id
            "#,
            folder.name,
            folder.owner_id,
            folder.parent_folder_id,
            folder.id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(f)
    }

    async fn delete_all(&self, folder_ids: &[Uuid]) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            DELETE FROM folders
            WHERE id = ANY($1)
            "#,
            folder_ids
        )
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn delete_by_id(&self, folder_id: Uuid) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            DELETE FROM folders
            WHERE id = $1
            "#,
            folder_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }
}
