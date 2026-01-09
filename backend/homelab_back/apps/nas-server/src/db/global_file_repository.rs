use homelab_core::global_file::GlobalFile;
use crate::exception::data_error::DataError;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait GlobalFileRepository: Send + Sync {
    async fn save(&self, global_file: GlobalFile) -> Result<GlobalFile, DataError>;
    async fn get_all(&self) -> Result<Vec<GlobalFile>, DataError>;
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
            INSERT INTO global_files (id, original_id)
            VALUES ($1, $2)
            RETURNING id, original_id
            "#,
            global_file.id,
            global_file.original_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(gf)
    }

    async fn get_all(&self) -> Result<Vec<GlobalFile>, DataError> {
        let global_files: Vec<GlobalFile> = sqlx::query_as!(
            GlobalFile,
            r#"
            SELECT id, original_id FROM global_files
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;
        
        Ok(global_files)
    }
}
