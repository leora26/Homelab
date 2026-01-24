use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_proto::admin::console_user_service_server::ConsoleUserService;
use homelab_proto::admin::{ConsoleUserListResponse, ConsoleUserResponse, GetAllUserVersionsRequest, GetLatestUserVersionRequest};
use crate::AppState;
use crate::helpers::proto_mappers::{map_console_user, map_entity_id};

#[derive(new)]
pub struct GrpcUserService {
    pub app_state: Arc<AppState>
}

#[tonic::async_trait]
impl ConsoleUserService for GrpcUserService {
    async fn get_all(&self, _request: Request<()>) -> Result<Response<ConsoleUserListResponse>, Status> {
        let users = self.app_state.user_service.get_all_users().await?;
        
        let proto_users = users.into_iter().map(|u| map_console_user(u)).collect();
        
        Ok(Response::new(ConsoleUserListResponse { users: proto_users }))
    }

    async fn get_latest_version(&self, request: Request<GetLatestUserVersionRequest>) -> Result<Response<ConsoleUserResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = map_entity_id(req.user_id)?;
        
        let user = self.app_state.user_service.get_latest(user_id).await?;
        
        Ok(Response::new(map_console_user(user)))
    }

    async fn get_all_user_versions(&self, request: Request<GetAllUserVersionsRequest>) -> Result<Response<ConsoleUserListResponse>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.user_id)?;

        let users = self.app_state.user_service.get_versions(user_id).await?;
        
        let proto_users = users.into_iter().map(|u| map_console_user(u)).collect();

        Ok(Response::new(ConsoleUserListResponse { users: proto_users }))
    }
}