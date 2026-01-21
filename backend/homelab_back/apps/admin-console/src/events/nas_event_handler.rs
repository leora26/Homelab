use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::events::{UserCreatedEvent, UserUpdatedEvent, WhiteListedUserCreatedEvent, WhiteListedUserUpdatedEvent};
use homelab_core::helpers::event_handler::EventHandler;
use crate::service::user_service::UserService;
use crate::service::wlu_service::WluService;

#[derive(new)]
pub struct NasEventHandler {
    user_service: Arc<dyn UserService>,
    wlu_service: Arc<dyn WluService>
}

#[async_trait]
impl EventHandler for NasEventHandler {
    async fn handle(&self, routing_key: &str, data: &[u8]) -> Result<(), String> {
        match routing_key {
            "user.created" => {
                let event: UserCreatedEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("👤 Handling User Creation: {}", event.user_id);

                self.user_service.log_new_user(event).await
                    .map_err(|e| format!("Error handling new user: {}", e))?;

                Ok(())
            },
            "user.updated" => {
                let event: UserUpdatedEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("👤 Handling User Creation: {}", event.user_id);

                self.user_service.log_updated_user(event).await
                    .map_err(|e| format!("Error handling updated user: {}", e))?;

                Ok(())
            },
            "whitelisted.user.created" => {
                let event: WhiteListedUserCreatedEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("👤 Handling User Creation: {}", event.user_id);

                self.wlu_service.log_new_wlu(event).await
                    .map_err(|e| format!("Error handling new WhiteListedUser: {}", e))?;

                Ok(())
            },
            "whitelisted.user.updated" => {
                let event: WhiteListedUserUpdatedEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("👤 Handling User Creation: {}", event.user_id);

                self.wlu_service.log_updated_wlu(event).await
                    .map_err(|e| format!("Error handling updated WhiteListedUser: {}", e))?;

                Ok(())
            }

            _ => {
                println!("⚠️ Ignoring unknown event: {}", routing_key);
                Ok(())
            }
        }
    }
}
