use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_proto::admin::console_user_service_server::ConsoleUserService;
use homelab_proto::admin::{
    ConsoleUserListResponse, FindUsersRequest, GetUserVersionsRequest, ListUsersRequest,
    SetQuotaRequest, ToggleBlockedRequest,
};
use homelab_core::admin_domain::console_user::ConsoleUser;
use crate::AppState;
use crate::helpers::proto_mappers::{map_console_user, map_entity_id};

#[derive(new)]
pub struct GrpcUserService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl ConsoleUserService for GrpcUserService {
    async fn get_log(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ConsoleUserListResponse>, Status> {
        let req = request.into_inner();
        let users = self
            .app_state
            .user_service
            .get_log(i64::from(req.limit), req.is_blocked)
            .await?;
        Ok(Response::new(to_list(users)))
    }

    async fn get_latest(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ConsoleUserListResponse>, Status> {
        let req = request.into_inner();
        let users = self
            .app_state
            .user_service
            .get_latest(i64::from(req.limit), req.is_blocked)
            .await?;
        Ok(Response::new(to_list(users)))
    }

    async fn find_users(
        &self,
        request: Request<FindUsersRequest>,
    ) -> Result<Response<ConsoleUserListResponse>, Status> {
        let req = request.into_inner();
        let users = self.app_state.user_service.find_users(req.query).await?;
        Ok(Response::new(to_list(users)))
    }

    async fn get_versions(
        &self,
        request: Request<GetUserVersionsRequest>,
    ) -> Result<Response<ConsoleUserListResponse>, Status> {
        let req = request.into_inner();
        let users = self
            .app_state
            .user_service
            .get_versions(req.query, i64::from(req.limit))
            .await?;
        Ok(Response::new(to_list(users)))
    }

    async fn toggle_blocked(
        &self,
        request: Request<ToggleBlockedRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let user_id = map_entity_id(req.user_id)?;

        self.app_state
            .user_client
            .toggle_blocked(user_id, req.is_blocked)
            .await
            .map_err(Status::internal)?;

        Ok(Response::new(()))
    }

    async fn set_quota(
        &self,
        request: Request<SetQuotaRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let user_id = map_entity_id(req.user_id)?;

        self.app_state
            .user_client
            .set_quota(user_id, req.allowed_storage)
            .await
            .map_err(Status::internal)?;

        Ok(Response::new(()))
    }
}

fn to_list(users: Vec<ConsoleUser>) -> ConsoleUserListResponse {
    ConsoleUserListResponse {
        users: users.into_iter().map(map_console_user).collect(),
    }
}
