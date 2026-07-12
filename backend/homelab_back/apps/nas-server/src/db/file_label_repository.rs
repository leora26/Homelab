use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::nas_domain::file_label::FileLabel;
use sqlx::PgPool;

#[async_trait]
pub trait FileLabelRepository: Send + Sync {
    async fn create(&self, fl: FileLabel) -> Result<FileLabel, DataError>;
    async fn delete(&self, fl: FileLabel) -> Result<(), DataError>;
}

#[derive(new)]
pub struct FileLabelRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl FileLabelRepository for FileLabelRepositoryImpl {
    async fn create(&self, fl: FileLabel) -> Result<FileLabel, DataError> {
        let fl = sqlx::query_as!(
            FileLabel,
            r#"
            INSERT INTO file_labels (file_id, label_id)
            VALUES ($1, $2)
            ON CONFLICT (file_id, label_id) DO UPDATE SET label_id = EXCLUDED.label_id
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

    async fn delete(&self, fl: FileLabel) -> Result<(), DataError> {
        sqlx::query!(
            "DELETE FROM file_labels WHERE file_id = $1 AND label_id = $2",
            fl.file_id,
            fl.label_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }
}
