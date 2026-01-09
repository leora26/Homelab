use async_trait::async_trait;
use sqlx::{PgPool};
use uuid::Uuid;
use homelab_core::shared_file::{SharedFile};
use crate::exception::data_error::DataError;

#[async_trait]
pub trait SharedFileRepository: Send + Sync {
    async fn create_shared_file(&self, shared_file: &SharedFile) -> Result<SharedFile, DataError>;
    async fn get_all_for_user(&self, user_id: Uuid) -> Result<Vec<SharedFile>, DataError>;
}

pub struct SharedFileRepositoryImpl {
    pool: PgPool,
}

impl SharedFileRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait]
impl SharedFileRepository for SharedFileRepositoryImpl {
    async fn create_shared_file(&self, shared_file: &SharedFile) -> Result<SharedFile, DataError> {
        let shared_file = sqlx::query_as!(
            SharedFile,
            r#"
            INSERT INTO shared_file (id, user_id, owner_id, file_id, access_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, owner_id, file_id, access_type as "access_type: _"
            "#,
            shared_file.id,
            shared_file.user_id,
            shared_file.owner_id,
            shared_file.file_id,
            shared_file.access_type as _
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(shared_file)
    }

    async fn get_all_for_user(&self, user_id: Uuid) -> Result<Vec<SharedFile>, DataError> {
        let shared_files = sqlx::query_as!(
            SharedFile,
            r#"
            SELECT id, user_id, owner_id, file_id, access_type as "access_type: _"
            FROM shared_file
            WHERE user_id = $1
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(shared_files)
    }
}
