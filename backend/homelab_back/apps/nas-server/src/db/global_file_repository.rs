use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use homelab_core::nas_domain::file::{File, FileType, UploadStatus};
use homelab_core::nas_domain::global_file::GlobalFile;
use sqlx::types::time::OffsetDateTime;
use sqlx::PgPool;
use uuid::Uuid;

/// A published global file joined with the full metadata of its underlying original
/// file, so the list endpoint can be rendered without a second lookup per entry.
///
/// `owner_name` is `users.full_name` of the file's owner and `shared_at` is
/// `global_files.created_at`; both come from the same join as the file itself.
pub struct GlobalFileWithMeta {
    pub id: Uuid,
    pub file: File,
    pub owner_name: String,
    pub shared_at: OffsetDateTime,
}

#[async_trait]
pub trait GlobalFileRepository: Send + Sync {
    async fn save(&self, global_file: GlobalFile) -> Result<GlobalFile, DataError>;
    async fn get_all(&self) -> Result<Vec<GlobalFileWithMeta>, DataError>;
    async fn remove_by_original_id(&self, original_id: Uuid) -> Result<(), DataError>;
    async fn is_global(&self, original_id: Uuid) -> Result<bool, DataError>;
}

pub struct GlobalFileRepositoryImpl {
    pool: PgPool,
}

impl GlobalFileRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GlobalFileRepository for GlobalFileRepositoryImpl {
    async fn save(&self, global_file: GlobalFile) -> Result<GlobalFile, DataError> {
        let gf = sqlx::query_as!(
            GlobalFile,
            r#"
            INSERT INTO global_files (id, original_id, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, original_id, created_at
            "#,
            global_file.id,
            global_file.original_id,
            global_file.created_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(gf)
    }

    async fn get_all(&self) -> Result<Vec<GlobalFileWithMeta>, DataError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                g.id            AS "global_id",
                g.created_at    AS "shared_at",
                u.full_name     AS "owner_name",
                f.id            AS "id",
                f.name          AS "name",
                f.owner_id      AS "owner_id",
                f.parent_folder_id AS "parent_folder_id",
                f.file_type     AS "file_type: FileType",
                f.is_deleted    AS "is_deleted",
                f.ttl           AS "ttl",
                f.size          AS "size",
                f.upload_status AS "upload_status: UploadStatus",
                f.created_at    AS "created_at",
                f.updated_at    AS "updated_at",
                f.hash          AS "hash",
                f.deleted_at    AS "deleted_at"
            FROM global_files g
            JOIN files f ON f.id = g.original_id
            JOIN users u ON u.id = f.owner_id
            WHERE f.is_deleted = FALSE
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        let result = rows
            .into_iter()
            .map(|r| GlobalFileWithMeta {
                id: r.global_id,
                owner_name: r.owner_name,
                shared_at: r.shared_at,
                file: File {
                    id: r.id,
                    name: r.name,
                    owner_id: r.owner_id,
                    parent_folder_id: r.parent_folder_id,
                    file_type: r.file_type,
                    is_deleted: r.is_deleted,
                    ttl: r.ttl,
                    size: r.size,
                    upload_status: r.upload_status,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    hash: r.hash,
                    deleted_at: r.deleted_at,
                },
            })
            .collect();

        Ok(result)
    }

    async fn remove_by_original_id(&self, original_id: Uuid) -> Result<(), DataError> {
        sqlx::query!("DELETE FROM global_files WHERE original_id = $1", original_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn is_global(&self, original_id: Uuid) -> Result<bool, DataError> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM global_files WHERE original_id = $1)"#,
            original_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(exists.unwrap_or(false))
    }
}
