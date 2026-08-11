use anyhow::{anyhow, Result};
use homelab_proto::admin::{SetVolumeSizeRequest, SetVolumeSizeResponse, VolumeStatusResponse};
use homelab_proto::admin::storage_admin_service_client::StorageAdminServiceClient;
use tonic::transport::Channel;
use crate::commands::ResizeCommand;
use crate::helpers::friendly_grpc_error;

pub struct StorageClient {
    storage: StorageAdminServiceClient<Channel>,
}

impl StorageClient {
    pub fn new(channel: Channel) -> Self {
        Self { storage: StorageAdminServiceClient::new(channel) }
    }

    pub async fn get_status(&self) -> Result<VolumeStatusResponse> {
        let resp = self
            .storage
            .clone()
            .get_volume_status(())
            .await
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }

    pub async fn resize (&self, command: ResizeCommand) -> Result<SetVolumeSizeResponse> {
        let resp = self
            .storage
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
