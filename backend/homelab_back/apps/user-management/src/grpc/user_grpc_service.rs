use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use crate::AppState;
use crate::data::create_user_command::CreateUserCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_user_to_proto};
use homelab_proto::user::{CreateUserRequest, GetUserByEmailRequest, UpdatePasswordRequest, UserList, UserResponse};
use homelab_proto::user::user_service_server::UserService;

#[derive(new)]
pub struct GrpcUserService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl UserService for GrpcUserService {
    async fn get_by_email(&self, request: Request<GetUserByEmailRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let user = self.app_state.user_service.get_by_email(&req.email)
            .await?
            .ok_or_else(|| Status::not_found(format!("No user found with email: {}", req.email)))?;

        Ok(Response::new(map_user_to_proto(user)))
    }

    async fn get_all(&self, _request: Request<()>) -> Result<Response<UserList>, Status> {
        let users = self.app_state.user_service.get_all().await?;

        let proto_users = users.into_iter().map(|u| map_user_to_proto(u)).collect();

        Ok(Response::new(UserList{ users: proto_users }))
    }

    async fn create(&self, request: Request<CreateUserRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let command = CreateUserCommand::new(req.email, req.password, req.full_name, req.allowed_storage);

        let user = self.app_state.user_service.create(command).await?;

        Ok(Response::new(map_user_to_proto(user)))
    }

    async fn update_password(&self, request: Request<UpdatePasswordRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.id)?;

        self.app_state.user_service.update_password(user_id, &req.password).await?;

        Ok(Response::new(()))
    }
}