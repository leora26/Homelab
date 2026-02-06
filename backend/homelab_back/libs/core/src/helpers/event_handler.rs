use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle (&self, routing_key: &str, data: &[u8]) -> Result<(), String>;
}