use std::collections::HashMap;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::nas_domain::label::Label;
use sqlx::PgPool;
use uuid::Uuid;

/// A label together with the number of files carrying it. `file_count` is an aggregate
/// over `file_labels`, not a column of `labels`, so it lives here rather than on `Label`.
pub struct LabelWithCount {
    pub label: Label,
    pub file_count: i64,
}

#[async_trait]
pub trait LabelRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Label>, DataError>;
    async fn get_all(&self, owner_id: Uuid) -> Result<Vec<LabelWithCount>, DataError>;
    async fn create(&self, label: Label) -> Result<Label, DataError>;
    async fn delete(&self, id: Uuid) -> Result<(), DataError>;
    async fn update(&self, label: Label) -> Result<Label, DataError>;
    async fn get_labels_by_file(
        &self,
        file_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Vec<Label>, DataError>;
    async fn get_labels_for_files(
        &self,
        file_ids: &[Uuid],
        owner_id: Uuid,
    ) -> Result<HashMap<Uuid, Vec<Label>>, DataError>;
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
            SELECT 
                id, 
                name, 
                color, 
                owner_id, 
                created_at
            FROM labels
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(label)
    }

    async fn get_all(&self, owner_id: Uuid) -> Result<Vec<LabelWithCount>, DataError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                l.id,
                l.name,
                l.color,
                l.owner_id,
                l.created_at,
                COUNT(fl.file_id) AS "file_count!"
            FROM labels l
            LEFT JOIN file_labels fl ON fl.label_id = l.id
            WHERE l.owner_id = $1
            GROUP BY l.id
            ORDER BY l.name
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        let labels = rows
            .into_iter()
            .map(|r| LabelWithCount {
                label: Label {
                    id: r.id,
                    name: r.name,
                    color: r.color,
                    owner_id: r.owner_id,
                    created_at: r.created_at,
                },
                file_count: r.file_count,
            })
            .collect();

        Ok(labels)
    }

    async fn create(&self, label: Label) -> Result<Label, DataError> {
        let label = sqlx::query_as!(
            Label,
            r#"
            INSERT INTO labels (id, name, color, owner_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, color, owner_id, created_at
            "#,
            label.id,
            label.name,
            label.color,
            label.owner_id,
            label.created_at
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
            RETURNING id, name, color, owner_id, created_at
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

    async fn get_labels_by_file(
        &self,
        file_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Vec<Label>, DataError> {
        let labels = sqlx::query_as!(
            Label,
            r#"
            SELECT
                l.id,
                l.name,
                l.color,
                l.owner_id,
                l.created_at
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

    async fn get_labels_for_files(&self, file_ids: &[Uuid], owner_id: Uuid) -> Result<HashMap<Uuid, Vec<Label>>, DataError> {
        if file_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let rows = sqlx::query!(
            r#"
             SELECT
                fl.file_id,
                l.id,
                l.name,
                l.color,
                l.owner_id,
                l.created_at
            FROM labels l
            INNER JOIN file_labels fl ON l.id = fl.label_id
            WHERE fl.file_id = ANY($1) AND l.owner_id = $2
            ORDER BY l.name
            "#,
            file_ids,
            owner_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DataError::DatabaseError(e))?;

        let mut grouped: HashMap<Uuid, Vec<Label>> = HashMap::new();

        for r in rows {
            grouped.entry(r.file_id)
                .or_default()
                .push(Label {
                    id: r.id,
                    name: r.name,
                    color: r.color,
                    owner_id: r.owner_id,
                    created_at: r.created_at,
                })
        }

        Ok(grouped)
    }
}
