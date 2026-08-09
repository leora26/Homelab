use async_trait::async_trait;
use derive_new::new;
use homelab_proto::nas::volume_service_client::VolumeServiceClient;
use homelab_proto::nas::{ResizeVolumeRequest, ResizeVolumeResponse, VolumeStatusResponse};
use std::error::Error;
use tonic::transport::Channel;
use tonic::{Request, Status};

#[async_trait]
pub trait VolumeRemoteClient: Send + Sync {
    async fn get_status(&self) -> Result<VolumeStatusResponse, Status>;
    async fn resize(
        &self,
        requested_bytes: i64,
        force_shrink: bool,
    ) -> Result<ResizeVolumeResponse, Status>;
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
    async fn get_status(&self) -> Result<VolumeStatusResponse, Status> {
        let mut client = self.client.clone();

        let response = client.get_status(Request::new(())).await?;
        Ok(response.into_inner())
    }

    async fn resize(
        &self,
        requested_bytes: i64,
        force_shrink: bool,
    ) -> Result<ResizeVolumeResponse, Status> {
        let mut client = self.client.clone();

        let request = Request::new(ResizeVolumeRequest {
            requested_bytes,
            force_shrink,
        });

        let response = client.resize(request).await?;
        Ok(response.into_inner())
    }
}
