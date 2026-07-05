use async_trait::async_trait;

#[async_trait]
pub trait ExternalIdResolver: Send + Sync {
    async fn resolve_external_id(&self, external_id: &str) -> Result<String, Box<dyn std::error::Error>>;
}