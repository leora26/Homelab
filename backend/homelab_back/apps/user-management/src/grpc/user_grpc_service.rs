use crate::data::create_user_command::CreateUserCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_user_to_proto};
use crate::AppState;
use derive_new::new;
use homelab_proto::user::user_service_server::UserService;
use homelab_proto::user::{FinalizeUserRequest, GetUserByEmailRequest, GetUserByIdRequest, SetStorageQuotaRequest, ToggleBlockStatusRequest, UpdateProfileRequest, UserCountResponse, UserList, UserResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use homelab_core::auth::extractor::RequestIdentityExt;

#[derive(new)]
pub struct GrpcUserService {
    pub app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl UserService for GrpcUserService {
    async fn get_by_id(&self, request: Request<GetUserByIdRequest>) -> Result<Response<UserResponse>, Status> {
        // Identity comes from the validated token (sub -> internal id), not from a
        // client-supplied id, so a caller can only ever read their own profile.
        let user_id = request.get_internal_id(&self.app_state.cached_identity_resolver).await?;

        let user = self
            .app_state
            .user_service
            .get_by_id(user_id)
            .await?
            .ok_or_else(|| Status::not_found(format!("No user found for the given id: {}", user_id)))?;

        Ok(Response::new(map_user_to_proto(user)))
    }

    async fn get_by_email(
        &self,
        request: Request<GetUserByEmailRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let user = self
            .app_state
            .user_service
            .get_by_email(&req.email)
            .await?
            .ok_or_else(|| Status::not_found(format!("No user found with email: {}", req.email)))?;

        Ok(Response::new(map_user_to_proto(user)))
    }

    async fn get_all(&self, _request: Request<()>) -> Result<Response<UserList>, Status> {
        let users = self.app_state.user_service.get_all().await?;

        let proto_users = users.into_iter().map(|u| map_user_to_proto(u)).collect();

        Ok(Response::new(UserList { users: proto_users }))
    }

    async fn finalize(
        &self,
        request: Request<FinalizeUserRequest>,
    ) -> Result<Response<()>, Status> {
        let internal_user_id = request.get_internal_id(&self.app_state.cached_identity_resolver).await?;

        let req = request.into_inner();

        let command = CreateUserCommand::new(internal_user_id, req.email, req.full_name);

        self.app_state.user_service.finalize(command).await?;

        Ok(Response::new(()))
    }


    async fn toggle_block_state(
        &self,
        request: Request<ToggleBlockStatusRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.id)?;

        self.app_state
            .user_service
            .toggle_blocked(user_id, req.is_blocked)
            .await?;

        Ok(Response::new(()))
    }

    async fn update_profile(
        &self,
        request: Request<UpdateProfileRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // Identity comes from the token, so a caller can only rename themselves.
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let user = self
            .app_state
            .user_service
            .update_profile(user_id, req.full_name)
            .await?;

        Ok(Response::new(map_user_to_proto(user)))
    }

    async fn set_storage_quota(
        &self,
        request: Request<SetStorageQuotaRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.id)?;

        self.app_state
            .user_service
            .set_storage_quota(user_id, req.allowed_storage)
            .await?;

        Ok(Response::new(()))
    }

    async fn get_user_count(&self, _request: Request<()>) -> Result<Response<UserCountResponse>, Status> {
        let count = self
            .app_state
            .user_service
            .get_user_count()
            .await?;

        Ok(Response::new(UserCountResponse { count }))
    }
}
