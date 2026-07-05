use crate::grpc::ownership::file_owned_by;
use crate::helpers::proto_mappers::{map_entity_id, map_global_file_to_proto};
use crate::AppState;
use async_trait::async_trait;
use derive_new::new;
use homelab_core::auth::extractor::RequestIdentityExt;
use homelab_proto::nas::global_file_service_server::GlobalFileService;
use homelab_proto::nas::{GlobalFileCommand, GlobalFileListResponse, IsGlobalResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(new)]
pub struct GrpcGlobalFileService {
    pub app_state: Arc<AppState>,
}

#[async_trait]
impl GlobalFileService for GrpcGlobalFileService {
    async fn get_all(&self, _: Request<()>) -> Result<Response<GlobalFileListResponse>, Status> {
        let global_files = self.app_state.global_file_service.get_all().await?;

        let proto_global_files = global_files
            .into_iter()
            .map(|f| map_global_file_to_proto(f))
            .collect();

        Ok(Response::new(GlobalFileListResponse {
            global_files: proto_global_files,
        }))
    }

    async fn make_global(
        &self,
        request: Request<GlobalFileCommand>,
    ) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let file_id = map_entity_id(request.into_inner().file_id)?;

        // Only the owner may publish their own file.
        file_owned_by(&self.app_state, file_id, user_id).await?;

        self.app_state.global_file_service.make_global(file_id).await?;

        Ok(Response::new(()))
    }

    async fn make_private(
        &self,
        request: Request<GlobalFileCommand>,
    ) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let file_id = map_entity_id(request.into_inner().file_id)?;

        // Only the owner may un-publish their own file.
        file_owned_by(&self.app_state, file_id, user_id).await?;

        self.app_state
            .global_file_service
            .make_private(file_id)
            .await?;

        Ok(Response::new(()))
    }

    async fn is_global(
        &self,
        request: Request<GlobalFileCommand>,
    ) -> Result<Response<IsGlobalResponse>, Status> {
        let file_id = map_entity_id(request.into_inner().file_id)?;

        let is_global = self.app_state.global_file_service.is_global(file_id).await?;

        Ok(Response::new(IsGlobalResponse { is_global }))
    }
}
