use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use tonic::{Request, Response, Status};
use crate::AppState;
use crate::helpers::proto_mappers::{map_global_file_to_proto};
use homelab_proto::nas::global_file_service_server::GlobalFileService;
use homelab_proto::nas::GlobalFileListResponse;

#[derive(new)]
pub struct GrpcGlobalFileService {
    pub app_state: Arc<AppState>,
}

#[async_trait]
impl GlobalFileService for GrpcGlobalFileService {
    async fn get_all(&self, _: Request<()>) -> Result<Response<GlobalFileListResponse>, Status> {
        let global_files = self.app_state.global_file_service.get_all()
            .await?;
        
        let proto_global_files = global_files.into_iter()
            .map(|f| map_global_file_to_proto(f)).collect();
        
        Ok(Response::new(GlobalFileListResponse {global_files: proto_global_files}))
    }
}