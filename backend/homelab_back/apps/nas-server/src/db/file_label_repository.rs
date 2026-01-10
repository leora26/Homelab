use async_trait::async_trait;
use derive_new::new;
use sqlx::PgPool;
use homelab_core::file_label::FileLabel;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FileLabelRepository: Send + Sync {
    async fn create(&self, fl: FileLabel) -> Result<FileLabel, DataError>;
}

#[derive(new)]
pub struct FileLabelRepositoryImpl {
    pool: PgPool
}

#[async_trait]
impl FileLabelRepository for FileLabelRepositoryImpl {
    async fn create(&self, fl: FileLabel) -> Result<FileLabel, DataError> {
        let fl = sqlx::query_as!(
            FileLabel,
            r#"
            INSERT INTO file_labels (file_id, label_id)
            VALUES ($1, $2)
            RETURNING file_id, label_id
            "#,
            fl.file_id,
            fl.label_id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(fl)
    }
}