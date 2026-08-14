use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ExternalIdResolver: Send + Sync {
    async fn resolve_external_id(&self, external_id: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// Live (never cached) check of whether the resolved user is currently blocked.
    /// Each service answers from its own source of truth — nas from `storage_profiles`,
    /// user-management from `users`.
    async fn is_blocked(&self, internal_id: Uuid) -> Result<bool, Box<dyn std::error::Error>>;
}