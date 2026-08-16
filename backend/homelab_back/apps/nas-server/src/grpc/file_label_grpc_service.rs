use crate::data::create_file_label_command::CreateFileLabelCommand;
use crate::helpers::proto_mappers::{
    map_entity_id, map_file_label_to_proto, map_file_to_proto, map_label_to_proto,
};
use crate::AppState;
use derive_new::new;
use homelab_core::auth::extractor::RequestIdentityExt;
use homelab_proto::nas::file_label_service_server::FileLabelService;
use homelab_proto::nas::{
    CreateFileLabelRequest, DeleteFileLabelRequest, FileLabelResponse, FileListResponse,
    GetFilesForLabelRequest, GetLabelsForFileRequest, LabelListResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::grpc::ownership::{file_owned_by, label_owned_by};

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
        let internal_user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;
        let label_id = map_entity_id(req.label_id)?;

        file_owned_by(&self.app_state, file_id, internal_user_id).await?;
        label_owned_by(&self.app_state, label_id, internal_user_id).await?;

        let command = CreateFileLabelCommand::new(file_id, label_id);

        let fl = self
            .app_state
            .file_label_service
            .create_file_label(command)
            .await?;

        Ok(Response::new(map_file_label_to_proto(fl)))
    }

    async fn delete_file_label(
        &self,
        request: Request<DeleteFileLabelRequest>,
    ) -> Result<Response<()>, Status> {
        let internal_user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;
        let label_id = map_entity_id(req.label_id)?;

        file_owned_by(&self.app_state, file_id, internal_user_id).await?;
        label_owned_by(&self.app_state, label_id, internal_user_id).await?;

        self.app_state
            .file_label_service
            .delete_file_label(file_id, label_id)
            .await?;

        Ok(Response::new(()))
    }

    async fn get_labels_for_file(
        &self,
        request: Request<GetLabelsForFileRequest>,
    ) -> Result<Response<LabelListResponse>, Status> {
        let internal_user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();
        let file_id = map_entity_id(req.file_id)?;

        let labels = self
            .app_state
            .file_label_service
            .get_labels_by_file(file_id, internal_user_id)
            .await?;

        let proto_labels = labels.into_iter().map(map_label_to_proto).collect();

        Ok(Response::new(LabelListResponse {
            labels: proto_labels,
        }))
    }

    async fn get_files_for_label(
        &self,
        request: Request<GetFilesForLabelRequest>,
    ) -> Result<Response<FileListResponse>, Status> {
        let internal_user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();
        let label_id = map_entity_id(req.label_id)?;

        let files = self
            .app_state
            .file_label_service
            .get_files_by_label(label_id, internal_user_id)
            .await?;

        let proto_files = files.into_iter().map(|f| map_file_to_proto(f, Vec::new())).collect();

        Ok(Response::new(FileListResponse { files: proto_files }))
    }
}
