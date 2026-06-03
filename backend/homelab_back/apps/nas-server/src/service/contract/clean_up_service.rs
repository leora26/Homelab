use async_trait::async_trait;
use homelab_core::events::TrashCleanUpTriggeredEvent;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait CleanUpService: Send + Sync {
    async fn handle_trash_delete(&self, event: TrashCleanUpTriggeredEvent)
                                 -> Result<(), DataError>;
    async fn hard_delete_all_trash(&self) -> Result<(), DataError>;
}
