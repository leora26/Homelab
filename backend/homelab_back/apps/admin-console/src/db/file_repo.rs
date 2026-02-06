use async_trait::async_trait;
use derive_new::new;
use sqlx::PgPool;
use uuid::Uuid;
use homelab_core::admin_domain::console_file::ConsoleFile;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileRepo: Send + Sync {
    async fn log_file(&self, file: ConsoleFile) -> Result<(), DataError>;
    async fn get_files(&self) -> Result<Vec<ConsoleFile>, DataError>;
    async fn get_latest_file (&self, file_id: Uuid) -> Result<ConsoleFile, DataError>;
    async fn get_all_file_versions (&self, file_id: Uuid) -> Result<Vec<ConsoleFile>, DataError>;
}

#[derive(new)]
pub struct FileRepoImpl {
    pool: PgPool
}

#[async_trait]
impl FileRepo for FileRepoImpl {
    async fn log_file(&self, file: ConsoleFile) -> Result<(), DataError> {
        sqlx::query_as!(
            ConsoleFile,
            r#"
            INSERT INTO console_file (
                                      id,
                                      file_id,
                                      file_type,
                                      is_deleted,
                                      ttl,
                                      size,
                                      upload_status,
                                      created_at,
                                      updated_at,
                                      version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            file.id,
            file.file_id,
            file.file_type as _,
            file.is_deleted,
            file.ttl,
            file.size,
            file.upload_status as _,
            file.created_at,
            file.updated_at,
            file.version
        )
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn get_files(&self) -> Result<Vec<ConsoleFile>, DataError> {
        let files = sqlx::query_as!(
            ConsoleFile,
            r#"
            SELECT
                id,
                file_id,
                file_type as "file_type: _",
                is_deleted,
                ttl,
                size,
                upload_status as "upload_status: _",
                created_at,
                updated_at,
                version
            FROM console_file
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(files)
    }

    async fn get_latest_file(&self, file_id: Uuid) -> Result<ConsoleFile, DataError> {
        let file = sqlx::query_as!(
            ConsoleFile,
            r#"
            SELECT
                id,
                file_id,
                file_type as "file_type: _",
                is_deleted,
                ttl,
                size,
                upload_status as "upload_status: _",
                created_at,
                updated_at,
                version
            FROM console_file
            WHERE file_id = $1
            ORDER BY version DESC
            LIMIT 1
            "#,
            file_id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(file)
    }

    async fn get_all_file_versions(&self, file_id: Uuid) -> Result<Vec<ConsoleFile>, DataError> {
        let file = sqlx::query_as!(
            ConsoleFile,
            r#"
            SELECT
                id,
                file_id,
                file_type as "file_type: _",
                is_deleted,
                ttl,
                size,
                upload_status as "upload_status: _",
                created_at,
                updated_at,
                version
            FROM console_file
            WHERE file_id = $1
            ORDER BY version DESC
            "#,
            file_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(file)
    }
}