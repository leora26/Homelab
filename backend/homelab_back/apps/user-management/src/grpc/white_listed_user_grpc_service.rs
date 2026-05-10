use crate::AppState;
use derive_new::new;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::data::create_white_listed_user_command::CreateWhiteListedUserCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_user_to_proto, map_wlu_to_proto};
use homelab_proto::user::{
    white_listed_user_service_server::WhiteListedUserService, ConfirmWhiteListedUsersRequest,
    CreateWhiteListedUserRequest, UserResponse, WhiteListedUserList, WhiteListedUserResponse,
};

#[derive(new)]
pub struct GrpcWhiteListedUserService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl WhiteListedUserService for GrpcWhiteListedUserService {
    async fn get_all(
        &self,
        _request: Request<()>,
    ) -> Result<Response<WhiteListedUserList>, Status> {
        let users = self.app_state.white_listed_user_service.get_all().await?;

        let proto_user = users.into_iter().map(|u| map_wlu_to_proto(u)).collect();

        Ok(Response::new(WhiteListedUserList { users: proto_user }))
    }

    async fn create(
        &self,
        request: Request<CreateWhiteListedUserRequest>,
    ) -> Result<Response<WhiteListedUserResponse>, Status> {
        let zitadel_id = request.extensions().get::<String>().cloned().ok_or_else(|| {
            Status::unauthenticated("Missing Zitadel ID")
        })?;

        let req = request.into_inner();

        let command = CreateWhiteListedUserCommand::new(req.email, req.full_name, zitadel_id.to_string());

        let user = self
            .app_state
            .white_listed_user_service
            .create(command)
            .await?;

        Ok(Response::new(map_wlu_to_proto(user)))
    }

    async fn confirm(
        &self,
        request: Request<ConfirmWhiteListedUsersRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.id)?;

        let user = self
            .app_state
            .white_listed_user_service
            .confirm(user_id)
            .await?;

        Ok(Response::new(map_user_to_proto(user)))
    }
}
