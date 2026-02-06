use std::error::Error;
use async_trait::async_trait;
use derive_new::new;
use tonic::Request;
use tonic::transport::Channel;
use uuid::Uuid;
use homelab_proto::user::ConfirmWhiteListedUsersRequest;
use homelab_proto::user::white_listed_user_service_client::WhiteListedUserServiceClient;
use crate::helpers::proto_mappers::{map_id_to_proto};

#[async_trait]
pub trait WluRemoteClient: Send + Sync {
    async fn confirm(&self, user_id: Uuid) -> Result<(), String>;
}

#[derive(new, Clone)]
pub struct WluRemoteClientImpl {
    client: WhiteListedUserServiceClient<Channel>
}

impl WluRemoteClientImpl {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn Error>> {
        let client = WhiteListedUserServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl WluRemoteClient for WluRemoteClientImpl {
    async fn confirm(&self, user_id: Uuid) -> Result<(), String> {
        let mut client = self.client.clone();

        let request = Request::new(ConfirmWhiteListedUsersRequest {
            id: Option::from(map_id_to_proto(user_id))
        });

        match client.confirm(request).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("gRPC call failed{}", e))
        }
    }
}