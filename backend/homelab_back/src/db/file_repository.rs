use async_trait::async_trait;
use derive_new::new;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::file::File;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait FileRepository: Send + Sync {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError>;
    async fn get_all_deleted(&self) -> Result<Vec<File>, DataError>;
    async fn search_by_name(&self, search_query: String) -> Result<Vec<File>, DataError>;
    async fn get_by_folder_and_file_name(&self, folder_id: Uuid, file_name: String) -> Result<Option<File>, DataError>;
    async fn save(&self, file: File) -> Result<File, DataError>;
    async fn update(&self, file: File) -> Result<File, DataError>;
    async fn delete_all(&self, file_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete_by_id(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn get_all_files_by_label (&self, label_id: Uuid, owner_id: Uuid) -> Result<Vec<File>, DataError>;
}

#[derive(new)]
pub struct FileRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl FileRepository for FileRepositoryImpl {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError> {
        let file = sqlx::query_as!(
        File,
            r#"
            SELECT id, name, owner_id, parent_folder_id, file_type as "file_type: _", is_deleted, ttl, size, upload_status as "upload_status: _"
            FROM files
            WHERE id = $1 AND is_deleted = FALSE
            "#,
        file_id
    )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(file)
    }

    async fn get_all_deleted(&self) -> Result<Vec<File>, DataError> {
        let f: Vec<File> = sqlx::query_as!(
            File,
            r#"
            SELECT id, name, owner_id, file_type as "file_type: _", parent_folder_id, is_deleted, ttl, size, upload_status as "upload_status: _"
            FROM files
            WHERE is_deleted = TRUE
            "#,
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(f)
    }

    async fn search_by_name(&self, search_query: String) -> Result<Vec<File>, DataError> {
        let f: Vec<File> = sqlx::query_as!(
            File,
            r#"
            SELECT id, name, owner_id, file_type as "file_type: _", parent_folder_id, is_deleted, ttl, size, upload_status as "upload_status: _"
            FROM files
            WHERE LOWER(name) LIKE LOWER($1) AND is_deleted = FALSE
            "#,
            search_query
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(f)
    }

    async fn get_by_folder_and_file_name(&self, folder_id: Uuid, file_name: String) -> Result<Option<File>, DataError> {
        let file = sqlx::query_as!(
            File,
            r#"
            SELECT id, name, owner_id, parent_folder_id, file_type as "file_type: _", is_deleted, ttl, size, upload_status as "upload_status: _"
            FROM files
            WHERE parent_folder_id = $1 AND name = $2 AND is_deleted = FALSE
            "#,
            folder_id,
            file_name
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(file)
    }

    async fn save(&self, file: File) -> Result<File, DataError> {
        let f = sqlx::query_as!(
        File,
        r#"
        INSERT INTO files (id, name, owner_id, parent_folder_id, file_type, is_deleted, size, upload_status)
        VALUES ($1,$2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, owner_id,parent_folder_id, file_type as "file_type: _", is_deleted, ttl, size, upload_status as "upload_status: _"
        "#,
            file.id,
            file.name,
            file.owner_id,
            file.parent_folder_id,
            file.file_type as _,
            file.is_deleted,
            file.size,
            file.upload_status as _
    )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(f)
    }

    async fn update(&self, file: File) -> Result<File, DataError> {
        let f = sqlx::query_as!(
            File,
            r#"
            UPDATE files
            SET name = $1, owner_id = $2, file_type = $3, parent_folder_id = $4, is_deleted = $5, ttl = $6, size = $7, upload_status = $8
            WHERE id = $9 and is_deleted = FALSE
            RETURNING id, name, owner_id, file_type as "file_type: _", parent_folder_id, is_deleted, ttl, size, upload_status as "upload_status: _"
            "#,
            file.name,
            file.owner_id,
            file.file_type as _,
            file.parent_folder_id,
            file.is_deleted,
            file.ttl,
            file.size,
            file.upload_status as _,
            file.id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(f)
    }

    async fn delete_all(&self, file_ids: &[Uuid]) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            DELETE FROM files WHERE id = ANY($1) AND is_deleted = TRUE
            "#,
            file_ids
        )
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn delete_by_id(&self, file_id: Uuid) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            DELETE FROM files WHERE id = $1 and is_deleted = TRUE
            "#,
            file_id
        )
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn get_all_files_by_label(&self, label_id: Uuid, owner_id: Uuid) -> Result<Vec<File>, DataError> {
        let files = sqlx::query_as!(
            File,
            r#"
            SELECT
                f.id,
                f.name,
                f.owner_id,
                f.parent_folder_id,
                f.file_type as "file_type: _",
                f.is_deleted,
                f.ttl,
                f.size,
                f.upload_status as "upload_status: _"
            FROM files f
            INNER JOIN file_labels fl ON f.id = fl.file_id
            WHERE fl.label_id = $1 AND f.owner_id = $2
            "#,
            label_id,
            owner_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(files)
    }
}