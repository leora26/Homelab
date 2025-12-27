use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use crate::AppState;
use crate::data::file_folder::change_label_command::ChangeLabelCommand;
use crate::data::file_folder::create_label_command::CreateLabelCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_label_to_proto};
use crate::pb::label_service_server::LabelService;
use crate::pb::{ChangeLabelRequest, CreateLabelRequest, DeleteLabelRequest, LabelListResponse, LabelResponse};

#[derive(new)]
pub struct GrpcLabelService {
    pub app_state: Arc<AppState>
}


#[tonic::async_trait]
impl LabelService for GrpcLabelService {
    async fn get_labels(&self, _ : Request<()>) -> Result<Response<LabelListResponse>, Status> {
        let labels = self.app_state.label_service.get_all().await?;

        let proto_labels = labels.into_iter().map(|l| map_label_to_proto(l))
            .collect();

        Ok(Response::new(LabelListResponse {labels: proto_labels}))
    }

    async fn create_label(&self, request: Request<CreateLabelRequest>) -> Result<Response<LabelResponse>, Status> {
        let req = request.into_inner();

        let owner_id = map_entity_id(req.owner_id)?;

        let command = CreateLabelCommand::new(req.name, req.color, owner_id);

        let label = self.app_state.label_service.create_label(command).await?;

        Ok(Response::new(map_label_to_proto(label)))
    }

    async fn delete_label(&self, request: Request<DeleteLabelRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let label_id = map_entity_id(req.id)?;

        self.app_state.label_service.delete_label(label_id).await?;

        Ok(Response::new(()))
    }

    async fn change_label(&self, request: Request<ChangeLabelRequest>) -> Result<Response<LabelResponse>, Status> {
        let req = request.into_inner();

        let label_id = map_entity_id(req.id)?;

        let command = ChangeLabelCommand::new(label_id, req.name, req.color);

        let label = self.app_state.label_service.change_label(command).await?;

        Ok(Response::new(map_label_to_proto(label)))
    }
}