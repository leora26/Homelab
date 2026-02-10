use std::error::Error;
use async_trait::async_trait;
use derive_new::new;
use tonic::Request;
use tonic::transport::Channel;
use uuid::Uuid;
use homelab_proto::user::ToggleBlockStatusRequest;
use homelab_proto::user::user_service_client::UserServiceClient;
use crate::helpers::proto_mappers::map_id_to_proto;

#[async_trait]
pub trait UserRemoteClient: Send + Sync {
    async fn toggle_blocked (&self, user_id: Uuid, is_blocked: bool) -> Result<(), String>;
}


#[derive(new)]
pub struct UserRemoteClientImpl {
    client: UserServiceClient<Channel>
}

impl UserRemoteClientImpl {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn Error>> {
        let client = UserServiceClient::connect(addr).await?;
        Ok(UserRemoteClientImpl { client })
    }
}


#[async_trait]
impl UserRemoteClient for UserRemoteClientImpl {
    async fn toggle_blocked(&self, user_id: Uuid, is_blocked: bool) -> Result<(), String> {
        let mut client = self.client.clone();

        let request = Request::new(ToggleBlockStatusRequest {
            id: Option::from(map_id_to_proto(user_id)),
            is_blocked
        });
        
        match client.toggle_block_state(request).await { 
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}