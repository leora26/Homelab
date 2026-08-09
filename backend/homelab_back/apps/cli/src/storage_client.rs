use anyhow::{Context, Result};
use homelab_proto::admin::VolumeStatusResponse;
use homelab_proto::admin::storage_admin_service_client::StorageAdminServiceClient;
use tonic::transport::Channel;

pub struct Client {
    volume: StorageAdminServiceClient<Channel>,
}

impl Client {
    pub async fn connect(addr: String) -> Result<Self> {
        let channel = Channel::from_shared(addr.clone())
            .with_context(|| format!("invalid server address: {addr}"))?
            .connect()
            .await
            .with_context(|| format!("could not reach admin-console at {addr}"))?;

        Ok(Self {
            volume: StorageAdminServiceClient::new(channel),
        })
    }

    pub async fn get_status(&self) -> Result<VolumeStatusResponse> {
        let resp = self
            .volume
            .clone()
            .get_volume_status(())
            .await
            .context("get_volume_status failed")?;

        Ok(resp.into_inner())
    }
}
