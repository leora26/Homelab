use crate::commands::{
    FindUserCommand, GetUserVersionCommand, ListUserCommand, SetQuotaCommand, ToggleBlockCommand,
};
use crate::helpers::friendly_grpc_error;
use anyhow::{Result, anyhow};
use homelab_proto::admin::console_user_service_client::ConsoleUserServiceClient;
use homelab_proto::admin::{
    ConsoleUserListResponse, FindUsersRequest, GetUserVersionsRequest,
    ListUsersRequest, SetQuotaRequest, ToggleBlockedRequest,
};
use tonic::transport::Channel;
use homelab_proto::common::EntityId;

pub struct UserClient {
    user: ConsoleUserServiceClient<Channel>,
}

impl UserClient {
    pub fn new(channel: Channel) -> Self {
        Self {
            user: ConsoleUserServiceClient::new(channel),
        }
    }

    pub async fn get_log(&self, command: ListUserCommand) -> Result<ConsoleUserListResponse> {
        let resp = self
            .user
            .clone()
            .get_log(ListUsersRequest {
                limit: command.limit as i32,
                is_blocked: command.is_blocked,
            })
            .await
            .map_err(|err| anyhow!(friendly_grpc_error(&err)))?;

        Ok(resp.into_inner())
    }

    pub async fn get_latest(&self, command: ListUserCommand) -> Result<ConsoleUserListResponse> {
        let resp = self
            .user
            .clone()
            .get_latest(ListUsersRequest {
                limit: command.limit as i32,
                is_blocked: command.is_blocked,
            })
            .await
            .map_err(|err| anyhow!(friendly_grpc_error(&err)))?;

        Ok(resp.into_inner())
    }

    pub async fn find_users(&self, command: FindUserCommand) -> Result<ConsoleUserListResponse> {
        let resp = self
            .user
            .clone()
            .find_users(FindUsersRequest {
                query: command.query,
            })
            .await
            .map_err(|err| anyhow!(friendly_grpc_error(&err)))?;

        Ok(resp.into_inner())
    }

    pub async fn get_versions(
        &self,
        command: GetUserVersionCommand,
    ) -> Result<ConsoleUserListResponse> {
        let resp = self
            .user
            .clone()
            .get_versions(GetUserVersionsRequest {
                query: command.query,
                limit: command.limit as i32,
            })
            .await
            .map_err(|err| anyhow!(friendly_grpc_error(&err)))?;

        Ok(resp.into_inner())
    }

    pub async fn toggle_blocked(&self, command: ToggleBlockCommand) -> Result<()> {
        self.user
            .clone()
            .toggle_blocked(ToggleBlockedRequest {
                user_id: Some(EntityId {
                    value: command.user_id
                }),
                is_blocked: command.is_blocked,
            })
            .await
            .map_err(|err| anyhow!(friendly_grpc_error(&err)))?;

        Ok(())
    }

    pub async fn set_quota(&self, command: SetQuotaCommand) -> Result<()> {
        self.user
            .clone()
            .set_quota(SetQuotaRequest {
                allowed_storage: command.allowed_storage,
                user_id: Some(EntityId {
                    value: command.user_id
                }),
            })
            .await
            .map_err(|err| anyhow!(friendly_grpc_error(&err)))?;

        Ok(())
    }
}
