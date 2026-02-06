use std::sync::Arc;
use derive_new::new;
use tonic::{Request, Response, Status};
use homelab_proto::admin::console_wlu_service_server::ConsoleWluService;
use homelab_proto::admin::{ConfirmWluRequest, ConsoleWluListResponse, ConsoleWluResponse, GetAllWluVersionsRequest, GetLatestWluRequest};
use crate::AppState;
use crate::helpers::proto_mappers::{map_console_wlu, map_entity_id};

#[derive(new)]
pub struct GrpcWluService {
    app_state: Arc<AppState>,
}

#[tonic::async_trait]
impl ConsoleWluService for GrpcWluService {
    async fn get_all_wlu(&self, _request: Request<()>) -> Result<Response<ConsoleWluListResponse>, Status> {
        let users = self.app_state.wlu_service.get_all().await?;

        let proto_users = users.into_iter().map(map_console_wlu).collect();

        Ok(Response::new(ConsoleWluListResponse {users: proto_users}))
    }

    async fn get_all_confirmed(&self, _request: Request<()>) -> Result<Response<ConsoleWluListResponse>, Status> {
        let users = self.app_state.wlu_service.get_all_confirmed().await?;

        let proto_users = users.into_iter().map(map_console_wlu).collect();

        Ok(Response::new(ConsoleWluListResponse {users: proto_users}))
    }

    async fn get_all_unconfirmed(&self, _request: Request<()>) -> Result<Response<ConsoleWluListResponse>, Status> {
        let users = self.app_state.wlu_service.get_all_unconfirmed().await?;

        let proto_users = users.into_iter().map(map_console_wlu).collect();

        Ok(Response::new(ConsoleWluListResponse {users: proto_users}))
    }

    async fn get_latest_version(&self, request: Request<GetLatestWluRequest>) -> Result<Response<ConsoleWluResponse>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.user_id)?;

        let user = self.app_state.wlu_service.get_latest(user_id).await?;
        Ok(Response::new(map_console_wlu(user)))
    }

    async fn get_all_wlu_version(&self, request: Request<GetAllWluVersionsRequest>) -> Result<Response<ConsoleWluListResponse>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.user_id)?;

        let users = self.app_state.wlu_service.get_versions(user_id).await?;

        let proto_users = users.into_iter().map(map_console_wlu).collect();

        Ok(Response::new(ConsoleWluListResponse {users: proto_users}))
    }

    async fn confirm_wlu(&self, request: Request<ConfirmWluRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let user_id = map_entity_id(req.id)?;

        let _ = self.app_state.wlu_client.confirm(user_id).await;

        Ok(Response::new(()))
    }
}