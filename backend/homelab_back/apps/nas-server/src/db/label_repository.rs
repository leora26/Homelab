use async_trait::async_trait;
use derive_new::new;
use sqlx::PgPool;
use uuid::Uuid;
use homelab_core::label::Label;
use crate::exception::data_error::DataError;

#[async_trait]
pub trait LabelRepository: Send + Sync {
    async fn get_by_id (&self, id: Uuid) -> Result<Option<Label>, DataError>;
    async fn get_all (&self) -> Result<Vec<Label>, DataError>;
    async fn create(&self, label: Label) -> Result<Label, DataError>;
    async fn delete(&self, id: Uuid) -> Result<(), DataError>;
    async fn update(&self, label: Label) -> Result<Label, DataError>;
    async fn get_labels_by_file (&self, file_id: Uuid, owner_id: Uuid) -> Result<Vec<Label>, DataError>;
}

#[derive(new)]
pub struct LabelRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl LabelRepository for LabelRepositoryImpl {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Label>, DataError> {
        let label = sqlx::query_as!(
            Label,
            r#"
            SELECT id, name, color, owner_id FROM labels
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(label)
    }

    async fn get_all(&self) -> Result<Vec<Label>, DataError> {
        let labels = sqlx::query_as!(
            Label,
            r#"
            SELECT id, name, color, owner_id FROM labels
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(labels)
    }

    async fn create(&self, label: Label) -> Result<Label, DataError> {
        let label = sqlx::query_as!(
            Label,
            r#"
            INSERT INTO labels (id, name, color, owner_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, color, owner_id
            "#,
            label.id,
            label.name,
            label.color,
            label.owner_id,
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(label)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            DELETE FROM labels
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn update(&self, label: Label) -> Result<Label, DataError> {
        let label = sqlx::query_as!(
            Label,
            r#"
            UPDATE labels
            SET name = $1, color = $2
            WHERE id = $3
            RETURNING id, name, color, owner_id
            "#,
            label.name,
            label.color,
            label.id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(label)
    }

    async fn get_labels_by_file(&self, file_id: Uuid, owner_id: Uuid) -> Result<Vec<Label>, DataError> {
        let labels = sqlx::query_as!(
            Label,
            r#"
            SELECT
                l.id,
                l.name,
                l.color,
                l.owner_id
            FROM labels l
            INNER JOIN file_labels fl ON l.id = fl.label_id
            WHERE fl.file_id = $1 AND l.owner_id = $2
            "#,
            file_id,
            owner_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        Ok(labels)
    }
}