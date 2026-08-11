use async_trait::async_trait;
use derive_new::new;
use sqlx::PgPool;
use uuid::Uuid;
use homelab_core::admin_domain::console_file::ConsoleFile;
use homelab_core::nas_domain::file::FileType;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileRepo: Send + Sync {
    async fn log_file(&self, file: ConsoleFile) -> Result<(), DataError>;
    async fn get_log(&self, limit: i64, file_type: Option<FileType>) -> Result<Vec<ConsoleFile>, DataError>;
    async fn get_latest(&self, limit: i64, file_type: Option<FileType>) -> Result<Vec<ConsoleFile>, DataError>;
    async fn find_by_prefix(&self, prefix: &str) -> Result<Vec<ConsoleFile>, DataError>;
    async fn get_versions(&self, prefix: &str, limit: i64) -> Result<Vec<ConsoleFile>, DataError>;
    async fn get_latest_file(&self, file_id: Uuid) -> Result<ConsoleFile, DataError>;
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

    async fn get_log(&self, limit: i64, file_type: Option<FileType>) -> Result<Vec<ConsoleFile>, DataError> {
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
                WHERE ($1::file_type IS NULL OR file_type = $1)
                ORDER BY updated_at DESC, version DESC
                LIMIT $2
                "#,
                file_type as _,
                limit,
            )
                .fetch_all(&self.pool)
                .await
                .map_err(|e| DataError::DatabaseError(e))?;

            Ok(files)
    }

    async fn get_latest(&self, limit: i64, file_type: Option<FileType>) -> Result<Vec<ConsoleFile>, DataError> {
        let files = sqlx::query_as!(
                ConsoleFile,
                r#"
                SELECT latest.id, latest.file_id,
                        latest.file_type as "file_type: _", latest.is_deleted, latest.ttl, latest.size,
                       latest.upload_status as "upload_status: _", latest.created_at, latest.updated_at, latest.version
                FROM (
                  SELECT DISTINCT ON (file_id) id, file_id, file_type, is_deleted, ttl, size,
                         upload_status, created_at, updated_at, version
                  FROM console_file
                  WHERE ($1::file_type IS NULL OR file_type = $1)
                  ORDER BY file_id, version DESC
                ) latest
                ORDER BY latest.updated_at DESC
                LIMIT $2
                "#,
                file_type as _,
                limit,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(files)
    }

    async fn find_by_prefix(&self, prefix: &str) -> Result<Vec<ConsoleFile>, DataError> {
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
                WHERE lower(file_id::text) like lower($1) || '%'
                ORDER BY version DESC
                "#,
                prefix
            )
                .fetch_all(&self.pool)
                .await
                .map_err(|e| DataError::DatabaseError(e))?;

            Ok(files)
    }

    async fn get_versions(&self, prefix: &str, limit: i64) -> Result<Vec<ConsoleFile>, DataError> {
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
                WHERE lower(file_id::text) like lower($1) || '%'
                ORDER BY version DESC
                LIMIT $2
                "#,
                prefix,
                limit
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

}