use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_proto::admin::console_file_service_server::ConsoleFileService;
use homelab_proto::admin::{ConsoleFileListResponse, ConsoleFileResponse, GetAllFileVersionsRequest, GetLatestFileVersionRequest};
use crate::AppState;
use crate::helpers::proto_mappers::{map_console_file, map_entity_id};

#[derive(new)]
pub struct GrpcFileService {
    pub app_state: Arc<AppState>
}


#[tonic::async_trait]
impl ConsoleFileService for GrpcFileService {
    async fn get_all(&self, _: Request<()>) -> Result<Response<ConsoleFileListResponse>, Status> {
        let files = self.app_state.file_service.get_all_files().await?;
        
        let proto_files = files.into_iter().map(|f| map_console_file(f)).collect();
        
        Ok(Response::new(ConsoleFileListResponse { files: proto_files }))
    }

    async fn get_latest_version(&self, request: Request<GetLatestFileVersionRequest>) -> Result<Response<ConsoleFileResponse>, Status> {
        let req = request.into_inner();
        
        let file_id = map_entity_id(req.file_id)?;
        
        let file = self.app_state.file_service.get_latest(file_id).await?;
        
        Ok(Response::new(map_console_file(file)))
    }

    async fn get_all_file_versions(&self, request: Request<GetAllFileVersionsRequest>) -> Result<Response<ConsoleFileListResponse>, Status> {
        let req = request.into_inner();
        
        let file_id = map_entity_id(req.file_id)?;
        
        let files = self.app_state.file_service.get_versions(file_id).await?;
        
        let proto_files = files.into_iter().map(|f| map_console_file(f)).collect();
        
        Ok(Response::new(ConsoleFileListResponse { files: proto_files }))
    }
}