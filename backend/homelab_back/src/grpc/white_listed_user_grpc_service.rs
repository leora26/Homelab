use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::AppState;

use crate::data::user::confirm_user_command::ConfirmUserCommand;
use crate::data::user::create_white_listed_user_command::CreateWhiteListedUserCommand;
use crate::helpers::proto_mappers::{map_user_to_proto, map_wlu_to_proto};
use crate::pb::{
    white_listed_user_service_server::WhiteListedUserService,
    WhiteListedUserList,
    WhiteListedUserResponse,
    CreateWhiteListedUserRequest,
    ConfirmWhiteListedUsersRequest,
    UserResponse,
};

#[derive(new)]
pub struct GrpcWhiteListedUserService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl WhiteListedUserService for GrpcWhiteListedUserService {
    async fn get_all(&self, _request: Request<()>) -> Result<Response<WhiteListedUserList>, Status> {
        let users = self.app_state.white_listed_user_service
            .get_all()
            .await?;

        let proto_user = users.into_iter().map(|u| map_wlu_to_proto(u)).collect();

        Ok(Response::new(WhiteListedUserList {users: proto_user}))
    }

    async fn create(&self, request: Request<CreateWhiteListedUserRequest>) -> Result<Response<WhiteListedUserResponse>, Status> {

        let req = request.into_inner();

        let command = CreateWhiteListedUserCommand::new(req.email, req.full_name);

        let user = self.app_state.white_listed_user_service.create(command).await?;

        Ok(Response::new(map_wlu_to_proto(user)))
    }

    async fn confirm(&self, request: Request<ConfirmWhiteListedUsersRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let entity_id = req.id.ok_or_else(|| Status::invalid_argument("Missing ID"))?;

        let user_id = Uuid::parse_str(&entity_id.value)
            .map_err(|_| Status::invalid_argument("Invalid UUID format"))?;

        let command= ConfirmUserCommand::new(req.allowed_storage);

        let user = self.app_state.white_listed_user_service.confirm(user_id, command).await?;

        Ok(Response::new(map_user_to_proto(user)))
    }
}

