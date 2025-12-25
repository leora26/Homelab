use crate::data::file_folder::copy_file_command::CopyFileCommand;
use crate::data::file_folder::init_file_command::InitFileCommand;
use crate::data::file_folder::move_file_command::MoveFileCommand;
use crate::data::file_folder::update_file_name_command::UpdateFileNameCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_file_to_proto};
use crate::pb::file_chunk::Data as FileChunkData;
use crate::pb::file_service_server::FileService;
use crate::pb::{
    CopyFileRequest, DeleteChosenFilesRequest, DeleteFileRequest, FileChunk, FileListResponse,
    FileResponse, GetFileRequest, InitFileRequest, MoveFileRequest, RenameFileRequest,
    SearchFilesRequest, UndeleteFileRequest,
};
use crate::AppState;
use async_trait::async_trait;
use derive_new::new;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

#[derive(new)]
pub struct GrpcFileService {
    pub app_state: Arc<AppState>,
}

#[async_trait]
impl FileService for GrpcFileService {
    async fn get_file(
        &self,
        request: Request<GetFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        let file = self
            .app_state
            .file_service
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| Status::not_found(format!("No user found with email: {}", file_id)))?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn search_files(
        &self,
        request: Request<SearchFilesRequest>,
    ) -> Result<Response<FileListResponse>, Status> {
        let req = request.into_inner();

        let files = self
            .app_state
            .file_service
            .search_file(req.file_name)
            .await?;

        let proto_files = files.into_iter().map(|f| map_file_to_proto(f)).collect();

        Ok(Response::new(FileListResponse { files: proto_files }))
    }

    async fn get_deleted_files(
        &self,
        _request: Request<()>,
    ) -> Result<Response<FileListResponse>, Status> {
        let files = self.app_state.file_service.get_all_deleted_files().await?;

        let proto_files = files.into_iter().map(|f| map_file_to_proto(f)).collect();

        Ok(Response::new(FileListResponse { files: proto_files }))
    }

    async fn init_file(
        &self,
        request: Request<InitFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let req = request.into_inner();

        let destination = map_entity_id(req.destination)?;

        let owner_id = map_entity_id(req.owner_id)?;

        let command =
            InitFileCommand::new(destination, owner_id, req.name, req.size, req.is_global);

        let file = self.app_state.file_service.upload(command).await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn upload_content(
        &self,
        request: Request<Streaming<FileChunk>>,
    ) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();

        let first_msg = match stream.message().await? {
            Some(msg) => msg,
            None => return Err(Status::invalid_argument("Stream cannot be empty")),
        };

        let file_id = match first_msg.data {
            Some(FileChunkData::FileId(id_msg)) => map_entity_id(Some(id_msg))?,
            Some(FileChunkData::Content(_)) => {
                return Err(Status::invalid_argument(
                    "First message must be File ID, not content",
                ));
            }
            None => return Err(Status::invalid_argument("First message empty")),
        };
        let (tx, rx) = mpsc::channel(32);

        let app_state_clone = self.app_state.clone();

        let service_handle = tokio::spawn(async move {
            app_state_clone
                .file_service
                .upload_stream(file_id, rx)
                .await
        });

        while let Ok(Some(msg)) = stream.message().await {
            match msg.data {
                Some(FileChunkData::Content(bytes)) => {
                    if tx.send(Ok(bytes)).await.is_err() {
                        break;
                    }
                }

                Some(FileChunkData::FileId(_)) => {
                    return Err(Status::invalid_argument(
                        "Received File ID inside content stream",
                    ));
                }

                None => {
                    continue;
                }
            }
        }

        drop(tx);

        match service_handle.await {
            Ok(service_result) => {
                service_result.map_err(|e| Status::from(e))?;
                Ok(Response::new(()))
            }
            Err(_) => Err(Status::internal("Upload task panicked")),
        }
    }

    async fn rename_file(
        &self,
        request: Request<RenameFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        let command = UpdateFileNameCommand::new(req.new_name);

        let file = self
            .app_state
            .file_service
            .update_file_name(command, file_id)
            .await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn undelete_file(
        &self,
        request: Request<UndeleteFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        let file = self
            .app_state
            .file_service
            .update_deleted_file(file_id)
            .await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn delete_chosen_files(
        &self,
        request: Request<DeleteChosenFilesRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let file_ids: Vec<Uuid> = req
            .file_ids
            .into_iter()
            .map(|id| map_entity_id(Some(id)))
            .collect::<Result<Vec<_>, _>>()?;

        self.app_state
            .file_service
            .delete_chosen_files(&file_ids)
            .await?;

        Ok(Response::new(()))
    }

    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        self.app_state.file_service.delete(file_id).await?;

        Ok(Response::new(()))
    }

    async fn move_file(
        &self,
        request: Request<MoveFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        let folder_id = map_entity_id(req.folder_id)?;

        let command = MoveFileCommand::new(file_id, folder_id);

        let file = self.app_state.file_service.move_file(command).await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn copy_file(
        &self,
        request: Request<CopyFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        let target_folder_id = map_entity_id(req.target_folder_id)?;

        let user_id = map_entity_id(req.user_id)?;

        let command = CopyFileCommand::new(file_id, target_folder_id, user_id);

        let file = self.app_state.file_service.copy_file(command).await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn update_file_content(
        &self,
        request: Request<Streaming<FileChunk>>,
    ) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();

        let first_msg = match stream.message().await? {
            Some(msg) => msg,
            None => return Err(Status::invalid_argument("Stream cannot be empty")),
        };

        let file_id = match first_msg.data {
            Some(FileChunkData::FileId(id_msg)) => map_entity_id(Some(id_msg))?,
            Some(FileChunkData::Content(_)) => {
                return Err(Status::invalid_argument(
                    "First message must be File ID, not content",
                ));
            }
            None => return Err(Status::invalid_argument("First message empty")),
        };
        let (tx, rx) = mpsc::channel(32);

        let app_state_clone = self.app_state.clone();

        let service_handle = tokio::spawn(async move {
            app_state_clone
                .file_service
                .update_stream(file_id, rx)
                .await
        });

        while let Ok(Some(msg)) = stream.message().await {
            match msg.data {
                Some(FileChunkData::Content(bytes)) => {
                    if tx.send(Ok(bytes)).await.is_err() {
                        break;
                    }
                }

                Some(FileChunkData::FileId(_)) => {
                    return Err(Status::invalid_argument(
                        "Received File ID inside content stream",
                    ));
                }

                None => {
                    continue;
                }
            }
        }

        drop(tx);

        match service_handle.await {
            Ok(service_result) => {
                service_result.map_err(|e| Status::from(e))?;
                Ok(Response::new(()))
            }
            Err(_) => Err(Status::internal("Upload task panicked")),
        }
    }
}
