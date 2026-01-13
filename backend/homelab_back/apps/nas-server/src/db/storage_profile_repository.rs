use crate::helpers::data_error::DataError;
use async_trait::async_trait;
use homelab_core::storage_profile::StorageProfile;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait StorageProfileRepository: Send + Sync {
    async fn create(&self, storage_profile: StorageProfile) -> Result<StorageProfile, DataError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<StorageProfile>, DataError>;
    async fn save(&self, storage_profile: StorageProfile) -> Result<(), DataError>;
}

pub struct StorageProfileRepositoryImpl {
    pool: PgPool,
}

impl StorageProfileRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StorageProfileRepository for StorageProfileRepositoryImpl {
    async fn create(&self, storage_profile: StorageProfile) -> Result<StorageProfile, DataError> {
        let sp = sqlx::query_as!(
            StorageProfile,
            r#"
        INSERT INTO storage_profiles (user_id, allowed_storage, taken_storage)
        VALUES ($1, $2, $3)
        RETURNING user_id, allowed_storage, taken_storage
        "#,
            storage_profile.user_id,
            storage_profile.allowed_storage,
            storage_profile.taken_storage
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(sp)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<StorageProfile>, DataError> {
        let sp = sqlx::query_as!(
            StorageProfile,
            r#"
        SELECT user_id, allowed_storage, taken_storage
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
            SET allowed_storage = $1, taken_storage = $2
            WHERE user_id = $3
            "#,
            storage_profile.allowed_storage,
            storage_profile.taken_storage,
            storage_profile.user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DataError::DatabaseError(e))?;

        Ok(())
    }
}
