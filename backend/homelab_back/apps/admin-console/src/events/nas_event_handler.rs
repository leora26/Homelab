use async_trait::async_trait;
use derive_new::new;
use homelab_core::events::UserCreatedEvent;
use homelab_core::helpers::event_handler::EventHandler;

#[derive(new)]
pub struct NasEventHandler {
}

#[async_trait]
impl EventHandler for NasEventHandler {
    async fn handle(&self, routing_key: &str, data: &[u8]) -> Result<(), String> {
        match routing_key {
            "user.created" => {
                let event: UserCreatedEvent = serde_json::from_slice(data)
                    .map_err(|e| format!("Json Error: {}", e))?;

                println!("👤 Handling User Creation: {}", event.user_id);

                // TODO: Implement service call here

                Ok(())
            },

            _ => {
                println!("⚠️ Ignoring unknown event: {}", routing_key);
                Ok(())
            }
        }
    }
}
