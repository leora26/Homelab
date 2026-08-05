use std::sync::Arc;
use crate::helpers::proto_mappers::map_volume_to_proto;
use crate::AppState;
use derive_new::new;
use homelab_core::nas_domain::volume::ResizeOutcome;
use homelab_proto::nas::volume_service_server::VolumeService;
use homelab_proto::nas::{ResizeVolumeRequest, ResizeVolumeResponse, VolumeStatusResponse};
use tonic::{Request, Response, Status};

#[derive(new)]
pub struct GrpcVolumeService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl VolumeService for GrpcVolumeService {
    async fn get_status(
        &self,
        _request: Request<()>,
    ) -> Result<Response<VolumeStatusResponse>, Status> {
        let s = self.app_state.volume_service.status().await?;
        Ok(Response::new(map_volume_to_proto(s)))
    }

    async fn resize(
        &self,
        request: Request<ResizeVolumeRequest>,
    ) -> Result<Response<ResizeVolumeResponse>, Status> {
        let request = request.into_inner();
        let outcome = self
            .app_state
            .volume_service
            .resize(request.requested_bytes, request.force_shrink)
            .await?;
        let status = self.app_state.volume_service.status().await?;

        let (changed, previous) = match outcome {
            ResizeOutcome::NoChange(_) => (false, status.quota),
            ResizeOutcome::Resized { from, .. } => (true, from),
        };

        Ok(Response::new(ResizeVolumeResponse {
            changed,
            previous_bytes: previous,
            current_bytes: status.quota.unwrap_or(0),
            status: Some(map_volume_to_proto(status)),
        }))
    }
}
