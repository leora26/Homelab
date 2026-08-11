use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_core::admin_domain::console_file::ConsoleFile;
use homelab_proto::admin::console_file_service_server::ConsoleFileService;
use homelab_proto::admin::{
    ConsoleFileListResponse, FindFilesRequest, GetVersionsRequest, ListFilesRequest,
};
use crate::AppState;
use crate::helpers::proto_mappers::{map_console_file, map_file_type_filter};

#[derive(new)]
pub struct GrpcFileService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl ConsoleFileService for GrpcFileService {
    async fn get_log(&self, request: Request<ListFilesRequest>) -> Result<Response<ConsoleFileListResponse>, Status> {
        let req = request.into_inner();

        let files = self
            .app_state
            .file_service
            .get_log(i64::from(req.limit), map_file_type_filter(req.file_type))
            .await?;

        Ok(Response::new(to_list(files)))
    }

    async fn get_latest(&self, request: Request<ListFilesRequest>) -> Result<Response<ConsoleFileListResponse>, Status> {
        let req = request.into_inner();

        let files = self
            .app_state
            .file_service
            .get_latest_files(i64::from(req.limit), map_file_type_filter(req.file_type))
            .await?;

        Ok(Response::new(to_list(files)))
    }

    async fn find_files(&self, request: Request<FindFilesRequest>) -> Result<Response<ConsoleFileListResponse>, Status> {
        let req = request.into_inner();

        let files = self
            .app_state
            .file_service
            .find_files(&req.id_prefix)
            .await?;

        Ok(Response::new(to_list(files)))
    }

    async fn get_versions(&self, request: Request<GetVersionsRequest>) -> Result<Response<ConsoleFileListResponse>, Status> {
        let req = request.into_inner();

        let files = self
            .app_state
            .file_service
            .get_file_versions(&req.id_prefix, i64::from(req.limit))
            .await?;

        Ok(Response::new(to_list(files)))
    }
}

fn to_list(files: Vec<ConsoleFile>) -> ConsoleFileListResponse {
    ConsoleFileListResponse {
        files: files.into_iter().map(map_console_file).collect(),
    }
}
