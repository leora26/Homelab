use anyhow::{anyhow, Context, Result};
use homelab_proto::admin::{SetVolumeSizeRequest, SetVolumeSizeResponse, VolumeStatusResponse};
use homelab_proto::admin::storage_admin_service_client::StorageAdminServiceClient;
use tonic::transport::Channel;
use crate::commands::ResizeCommand;
use crate::helpers::friendly_grpc_error;

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
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }

    pub async fn resize (&self, command: ResizeCommand) -> Result<SetVolumeSizeResponse> {
        let resp = self
            .volume
            .clone()
            .set_volume_size(SetVolumeSizeRequest {
                requested_bytes: command.requested_bytes,
                force_shrink: command.force_shrink,
            })
            .await
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }
}
