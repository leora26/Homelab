use crate::data::file_folder::create_file_label_command::CreateFileLabelCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_file_label_to_proto, map_file_to_proto, map_label_to_proto};
use homelab_proto::nas::file_label_service_server::FileLabelService;
use homelab_proto::nas::{
    CreateFileLabelRequest, FileLabelResponse, FileListResponse, GetFilesForLabelRequest,
    GetLabelsForFileRequest, LabelListResponse,
};
use crate::AppState;
use derive_new::new;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(new)]
pub struct GrpcFileLabelService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl FileLabelService for GrpcFileLabelService {
    async fn create_file_label(
        &self,
        request: Request<CreateFileLabelRequest>,
    ) -> Result<Response<FileLabelResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;
        let label_id = map_entity_id(req.label_id)?;

        let command = CreateFileLabelCommand::new(file_id, label_id);

        let fl = self
            .app_state
            .file_label_service
            .create_file_label(command)
            .await?;

        Ok(Response::new(map_file_label_to_proto(fl)))
    }

    async fn get_labels_for_file(
        &self,
        request: Request<GetLabelsForFileRequest>,
    ) -> Result<Response<LabelListResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;
        let owner_id = map_entity_id(req.owner_id)?;

        let labels = self
            .app_state
            .file_label_service
            .get_labels_by_file(file_id, owner_id)
            .await?;

        let proto_labels = labels.into_iter().map(map_label_to_proto).collect();

        Ok(Response::new(LabelListResponse { labels: proto_labels }))
    }

    async fn get_files_for_label(
        &self,
        request: Request<GetFilesForLabelRequest>,
    ) -> Result<Response<FileListResponse>, Status> {
        let req = request.into_inner();

        let label_id = map_entity_id(req.label_id)?;
        let owner_id = map_entity_id(req.owner_id)?;

        let files = self
            .app_state
            .file_label_service
            .get_files_by_label(label_id, owner_id)
            .await?;

        let proto_files = files.into_iter().map(map_file_to_proto).collect();

        Ok(Response::new(FileListResponse {files: proto_files}))
    }
}
