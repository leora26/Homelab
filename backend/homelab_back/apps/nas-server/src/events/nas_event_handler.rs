use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::events::{TrashCleanUpTriggeredEvent, UserCreatedEvent};
use homelab_core::helpers::event_handler::EventHandler;
use crate::service::clean_up_service::CleanUpService;
use crate::service::storage_profile_service::StorageProfileService;

#[derive(new)]
pub struct NasEventHandler {
    storage_profile_service: Arc<dyn StorageProfileService>,
    clean_up_service: Arc<dyn CleanUpService>
}

#[async_trait]
impl EventHandler for NasEventHandler {
    async fn handle(&self, routing_key: &str, data: &[u8]) -> Result<(), String> {
        match routing_key {
            "user.created" => {
                let event: UserCreatedEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("👤 Handling User Creation: {}", event.user_id);

                let profile = self.storage_profile_service.save_storage_profile(event).await
                    .map_err(|e| format!("DB Error: {}", e))?;

                eprintln!("Create storage profile: {}; {}; {}",
                          profile.user_id,
                          profile.allowed_storage,
                          profile.taken_storage
                );

                Ok(())
            },
            "cleanup.triggered" => {
                let event: TrashCleanUpTriggeredEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("Handling Trash CleanUp Triggered: {}", event.user_id);

                self.clean_up_service.handle_trash_delete(event).await
                    .map_err(|e| format!("DB Error: {}", e))?;

                Ok(())
            },

            _ => {
                println!("⚠️ Ignoring unknown event: {}", routing_key);
                Ok(())
            }
        }
    }
}
