use async_trait::async_trait;
use derive_new::new;
use homelab_proto::nas::volume_service_client::VolumeServiceClient;
use homelab_proto::nas::{ResizeVolumeRequest, ResizeVolumeResponse, VolumeStatusResponse};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

#[async_trait]
pub trait VolumeRemoteClient: Send + Sync {
    async fn get_status(&self) -> Result<VolumeStatusResponse, String>;
    async fn resize(
        &self,
        requested_bytes: i64,
        force_shrink: bool,
    ) -> Result<ResizeVolumeResponse, String>;
}

#[derive(new)]
pub struct VolumeRemoteClientImpl {
    client: VolumeServiceClient<Channel>,
}

impl VolumeRemoteClientImpl {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn Error>> {
        let client = VolumeServiceClient::connect(addr).await?;
        Ok(VolumeRemoteClientImpl { client })
    }
}

#[async_trait]
impl VolumeRemoteClient for VolumeRemoteClientImpl {
    async fn get_status(&self) -> Result<VolumeStatusResponse, String> {
        let mut client = self.client.clone();

        let request = Request::new(());

        match client.get_status(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn resize(
        &self,
        requested_bytes: i64,
        force_shrink: bool,
    ) -> Result<ResizeVolumeResponse, String> {
        let mut client = self.client.clone();

        let request = Request::new(ResizeVolumeRequest {
            requested_bytes,
            force_shrink,
        });

        match client.resize(request).await {
            Ok(response) => Ok(response.into_inner()),
            Err(error) => Err(error.to_string()),
        }
    }
}
