use crate::commands::{FindFileCommand, GetVersionsCommand, ListFileCommand};
use anyhow::{Result, anyhow};
use homelab_proto::admin::console_file_service_client::ConsoleFileServiceClient;
use homelab_proto::admin::{
    ConsoleFileListResponse, FindFilesRequest, GetVersionsRequest, ListFilesRequest,
};
use tonic::transport::Channel;
use crate::helpers::friendly_grpc_error;

pub struct FileClient {
    file: ConsoleFileServiceClient<Channel>,
}

impl FileClient {
    pub fn new(channel: Channel) -> Self {
        Self {
            file: ConsoleFileServiceClient::new(channel),
        }
    }

    pub async fn get_log(&self, command: ListFileCommand) -> Result<ConsoleFileListResponse> {
        let resp = self
            .file
            .clone()
            .get_log(ListFilesRequest {
                limit: command.limit as i32,
                file_type: command.file_type,
            })
            .await
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }

    pub async fn get_latest(&self, command: ListFileCommand) -> Result<ConsoleFileListResponse> {
        let resp = self
            .file
            .clone()
            .get_latest(ListFilesRequest {
                limit: command.limit as i32,
                file_type: command.file_type,
            })
            .await
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }

    pub async fn find_files(&self, command: FindFileCommand) -> Result<ConsoleFileListResponse> {
        let resp = self
            .file
            .clone()
            .find_files(FindFilesRequest {
                id_prefix: command.prefix,
            })
            .await
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }

    pub async fn get_versions(
        &self,
        command: GetVersionsCommand,
    ) -> Result<ConsoleFileListResponse> {
        let resp = self
            .file
            .clone()
            .get_versions(GetVersionsRequest {
                id_prefix: command.prefix,
                limit: command.limit as i32,
            })
            .await
            .map_err(|s| anyhow!(friendly_grpc_error(&s)))?;

        Ok(resp.into_inner())
    }
}
