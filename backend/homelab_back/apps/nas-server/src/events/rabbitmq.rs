use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Result};
use lapin::options::BasicPublishOptions;
use serde::Serialize;
use homelab_core::events::DomainEvent;

#[derive(Clone)]
pub struct RabbitMqPublisher {
    channel: Channel,
}

impl RabbitMqPublisher {
    pub async fn new(connection_str: &str) -> Result<Self> {
        let connection =
            Connection::connect(connection_str, ConnectionProperties::default()).await?;

        let channel = connection.create_channel().await?;

        Ok(Self { channel })
    }

    pub async fn publish<T> (&self, event: &T) -> Result<()>
    where
        T: DomainEvent + Serialize,
    {
        let payload = serde_json::to_vec(event).expect("Failed to serialize event");

        self.channel
            .basic_publish(
                "homelab_events",
                event.routing_key(),
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
            )
            .await?
            .await?;

        Ok(())
    }
}
