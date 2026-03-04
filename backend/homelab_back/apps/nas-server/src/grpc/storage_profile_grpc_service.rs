use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_proto::nas::{GetStorageProfileByIdRequest, StorageProfileResponse};
use homelab_proto::nas::storage_profile_service_server::StorageProfileService;
use crate::AppState;
use crate::helpers::proto_mappers::{map_entity_id, map_storage_profile_to_proto};

#[derive(new)]
pub struct GrpcStorageProfileService {
    pub app_state: Arc<AppState>
}


#[async_trait]
impl StorageProfileService for GrpcStorageProfileService {
    async fn get_by_id(&self, request: Request<GetStorageProfileByIdRequest>) -> Result<Response<StorageProfileResponse>, Status> {
        let req = request.into_inner();

        let id = map_entity_id(req.id)?;

        let sp = self.app_state
            .storage_profile_service
            .get_by_id(id)
            .await?
            .ok_or_else(|| Status::not_found(format!("No storage was found with given id: {}", id)))?;

        Ok(Response::new(map_storage_profile_to_proto(sp)))
    }
}