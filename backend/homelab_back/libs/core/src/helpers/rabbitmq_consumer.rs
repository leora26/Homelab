use std::sync::Arc;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions, ExchangeDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties, Result, ExchangeKind,
};
use tonic::codegen::tokio_stream::StreamExt;
use crate::helpers::event_handler::EventHandler;

pub struct RabbitMqConsumer;

impl RabbitMqConsumer {
    pub async fn start(
        connection_addr: &str,
        handler: Arc<dyn EventHandler>,
        listen_patterns: Vec<&str>
    ) -> Result<()> {
        let conn = Connection::connect(connection_addr, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        let exchange_name = "homelab_events";
        channel.exchange_declare(
            exchange_name,
            ExchangeKind::Topic,
            ExchangeDeclareOptions {durable: true, ..Default::default()},
            FieldTable::default(),
        ).await?;

        let queue_name = "nas_service_queue";
        let _queue = channel.queue_declare(
            queue_name,
            QueueDeclareOptions { durable: true, ..Default::default() },
            FieldTable::default(),
        ).await?;

        for pattern in listen_patterns {
            channel.queue_bind(
                queue_name,
                exchange_name,
                pattern,
                QueueBindOptions::default(),
                FieldTable::default(),
            ).await?;
            println!("🐰 Bound queue to pattern: {}", pattern);
        }

        let mut consumer = channel.basic_consume(
            queue_name,
            "nas_generic_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await?;

        println!("🚀 Generic Consumer started!");

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let routing_key = delivery.routing_key.as_str();

                match handler.handle(routing_key, &delivery.data).await {
                    Ok(_) => {
                        delivery.ack(BasicAckOptions::default()).await?;
                    }
                    Err(e) => {
                        eprintln!("❌ Error handling {}: {}", routing_key, e);
                        delivery.nack(lapin::options::BasicNackOptions {
                            requeue: false, multiple: false
                        }).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
}