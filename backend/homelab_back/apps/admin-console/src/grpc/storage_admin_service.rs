use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_proto::admin::storage_admin_service_server::StorageAdminService;
use homelab_proto::admin::{SetVolumeSizeRequest, SetVolumeSizeResponse, VolumeStatusResponse};
use crate::AppState;

#[derive(new)]
pub struct GrpcStorageAdminService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl StorageAdminService for GrpcStorageAdminService {
    async fn get_volume_status(&self, _request: Request<()>) -> Result<Response<VolumeStatusResponse>, Status> {
        let status = self
            .app_state
            .volume_client
            .get_status()
            .await
            .map_err(Status::internal)?;

        Ok(Response::new(map_status(status)))
    }

    async fn set_volume_size(&self, request: Request<SetVolumeSizeRequest>) -> Result<Response<SetVolumeSizeResponse>, Status> {
        let req = request.into_inner();

        let resize = self
            .app_state
            .volume_client
            .resize(req.requested_bytes, req.force_shrink)
            .await
            .map_err(Status::internal)?;

        Ok(Response::new(map_resize(resize)))
    }
}

// nas.* proto -> admin.* proto (separate packages, identical shapes)
fn map_status(s: homelab_proto::nas::VolumeStatusResponse) -> VolumeStatusResponse {
    VolumeStatusResponse {
        dataset: s.dataset,
        mountpoint: s.mountpoint,
        used: s.used,
        available: s.available,
        quota: s.quota,
        reservation: s.reservation,
        referenced: s.referenced,
        used_by_snapshots: s.used_by_snapshots,
        pool_free: s.pool_free,
    }
}

fn map_resize(r: homelab_proto::nas::ResizeVolumeResponse) -> SetVolumeSizeResponse {
    SetVolumeSizeResponse {
        changed: r.changed,
        previous_bytes: r.previous_bytes,
        current_bytes: r.current_bytes,
        status: r.status.map(map_status),
    }
}
