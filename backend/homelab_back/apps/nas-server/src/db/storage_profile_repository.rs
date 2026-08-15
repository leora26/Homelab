use std::error::Error;
use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use homelab_core::nas_domain::storage_profile::StorageProfile;
use sqlx::PgPool;
use uuid::Uuid;
use homelab_core::auth::resolver::ExternalIdResolver;
use homelab_core::nas_domain::storage_stats::StorageStats;

#[async_trait]
pub trait StorageProfileRepository: Send + Sync {
    async fn create(&self, storage_profile: StorageProfile) -> Result<StorageProfile, DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<StorageProfile>, DataError>;
    async fn save(&self, storage_profile: StorageProfile) -> Result<(), DataError>;
    async fn get_stats(&self, id: Uuid) -> Result<StorageStats, DataError>;
}

#[derive(Clone)]
pub struct StorageProfileRepositoryImpl {
    pool: PgPool,
}

impl StorageProfileRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExternalIdResolver for StorageProfileRepositoryImpl {
    async fn resolve_external_id(&self, external_id: &str) -> Result<String, Box<dyn Error>> {
        let record = sqlx::query!(
            "SELECT user_id FROM storage_profiles WHERE external_id = $1",
            external_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(record.user_id.to_string())
    }

    async fn is_blocked(&self, internal_id: Uuid) -> Result<bool, Box<dyn Error>> {
        let record = sqlx::query!(
            "SELECT is_blocked FROM storage_profiles WHERE user_id = $1",
            internal_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(record.is_blocked)
    }
}

#[async_trait]
impl StorageProfileRepository for StorageProfileRepositoryImpl {
    async fn create(&self, storage_profile: StorageProfile) -> Result<StorageProfile, DataError> {
        let sp = sqlx::query_as!(
            StorageProfile,
            r#"
        INSERT INTO storage_profiles (user_id, allowed_storage, taken_storage, is_blocked)
        VALUES ($1, $2, $3, $4)
        RETURNING user_id, allowed_storage, taken_storage, is_blocked
        "#,
            storage_profile.user_id,
            storage_profile.allowed_storage,
            storage_profile.taken_storage,
            storage_profile.is_blocked,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(sp)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<StorageProfile>, DataError> {
        let sp = sqlx::query_as!(
            StorageProfile,
            r#"
        SELECT user_id, allowed_storage, taken_storage, is_blocked
        FROM storage_profiles
        WHERE user_id = $1
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(sp)
    }

    async fn save(&self, storage_profile: StorageProfile) -> Result<(), DataError> {
        sqlx::query!(
            r#"
            UPDATE storage_profiles
            SET allowed_storage = $1, taken_storage = $2, is_blocked = $3
            WHERE user_id = $4
            "#,
            storage_profile.allowed_storage,
            storage_profile.taken_storage,
            storage_profile.is_blocked,
            storage_profile.user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }

    async fn get_stats(&self, id: Uuid) -> Result<StorageStats, DataError> {
        let stats = sqlx::query_as!(
            StorageStats,
            r#"
            SELECT
                (SELECT COUNT(*) FROM files
                  WHERE owner_id = $1 AND is_deleted = FALSE) AS "file_count!",

                (SELECT COUNT(*) FROM folders
                  WHERE owner_id = $1 AND is_deleted = FALSE
                    AND parent_folder_id IS NOT NULL) AS "folder_count!",

                -- Top-level only: an item whose parent is itself trashed is not its own
                -- row in Trash. Same filter the Trash listing queries use, so this count
                -- matches what the screen renders.
                (
                  (SELECT COUNT(*) FROM files f
                     LEFT JOIN folders p ON p.id = f.parent_folder_id
                    WHERE f.owner_id = $1 AND f.is_deleted = TRUE
                      AND (p.id IS NULL OR p.is_deleted = FALSE))
                  +
                  (SELECT COUNT(*) FROM folders f
                     LEFT JOIN folders p ON p.id = f.parent_folder_id
                    WHERE f.owner_id = $1 AND f.is_deleted = TRUE
                      AND (p.id IS NULL OR p.is_deleted = FALSE))
                ) AS "trashed_item_count!",

                (SELECT COALESCE(SUM(size), 0)::BIGINT FROM files
                  WHERE owner_id = $1 AND is_deleted = TRUE) AS "trashed_bytes!",

                (SELECT COUNT(*) FROM files f
                  WHERE f.owner_id = $1 AND f.is_deleted = FALSE
                    AND EXISTS (SELECT 1 FROM file_labels fl
                                 WHERE fl.file_id = f.id)) AS "labelled_file_count!",

                (SELECT COUNT(*) FROM files f
                  WHERE f.owner_id = $1 AND f.is_deleted = FALSE
                    AND NOT EXISTS (SELECT 1 FROM file_labels fl
                                     WHERE fl.file_id = f.id)) AS "unlabelled_file_count!",

                (SELECT COUNT(*) FROM global_files g
                   JOIN files f ON f.id = g.original_id
                  WHERE f.owner_id = $1 AND f.is_deleted = FALSE) AS "shared_file_count!",

                (SELECT COALESCE(SUM(f.size), 0)::BIGINT FROM global_files g
                   JOIN files f ON f.id = g.original_id
                  WHERE f.owner_id = $1 AND f.is_deleted = FALSE) AS "shared_bytes!"
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(stats)
    }
}
