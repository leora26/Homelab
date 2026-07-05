use crate::data::copy_file_command::CopyFileCommand;
use crate::data::init_file_command::InitFileCommand;
use crate::data::move_file_command::MoveFileCommand;
use crate::data::update_file_name_command::UpdateFileNameCommand;
use crate::grpc::ownership::{file_owned_by, folder_owned_by};
use crate::helpers::proto_mappers::{map_entity_id, map_file_to_proto};
use crate::AppState;
use async_trait::async_trait;
use derive_new::new;
use homelab_proto::nas::file_chunk::Data as FileChunkData;
use homelab_proto::nas::file_service_server::FileService;
use homelab_proto::nas::{ArchiveFileRequest, CopyFileRequest, DeleteChosenFilesRequest, DeleteFileRequest, FileChunk, FileListResponse, FileResponse, GetDeletedFilesRequest, GetFileRequest, InitFileRequest, MoveFileRequest, RemoveDeletedFileRequest, RenameFileRequest, SearchFilesRequest, UnarchiveFileRequest, UndeleteFileRequest};
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;
use homelab_core::auth::extractor::{resolve_internal_id, RequestIdentityExt};

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
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        let file = self
            .app_state
            .file_read_service
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| Status::not_found("File not found"))?;

        if file.owner_id != user_id
            && !self.app_state.global_file_service.is_global(file.id).await?
        {
            return Err(Status::not_found("File not found"));
        }

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn search_files(
        &self,
        request: Request<SearchFilesRequest>,
    ) -> Result<Response<FileListResponse>, Status> {
        let req = request.into_inner();

        let files = self
            .app_state
            .file_read_service
            .search_file(req.file_name)
            .await?;

        let proto_files = files.into_iter().map(|f| map_file_to_proto(f)).collect();

        Ok(Response::new(FileListResponse { files: proto_files }))
    }

    async fn get_deleted_files(
        &self,
        request: Request<GetDeletedFilesRequest>,
    ) -> Result<Response<FileListResponse>, Status> {
        let internal_user_id = request.get_internal_id(&self.app_state.cached_identity_resolver).await?;

        let files = self
            .app_state
            .file_read_service
            .get_all_deleted_files(internal_user_id)
            .await?;

        let proto_files = files.into_iter().map(|f| map_file_to_proto(f)).collect();

        Ok(Response::new(FileListResponse { files: proto_files }))
    }

    async fn init_file(
        &self,
        request: Request<InitFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let internal_user_id = request.get_internal_id(&self.app_state.cached_identity_resolver).await?;

        let req = request.into_inner();

        let destination = map_entity_id(req.destination)?;

        // The destination folder must belong to the caller.
        folder_owned_by(&self.app_state, destination, internal_user_id).await?;

        let command =
            InitFileCommand::new(destination, internal_user_id, req.name, req.size);

        let file = self.app_state.file_write_service.upload(command).await?;
        println!("{:#?}", file);

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn upload_content(
        &self,
        request: Request<Streaming<FileChunk>>,
    ) -> Result<Response<()>, Status> {
        // `Streaming<T>` is not `Sync`, so the `RequestIdentityExt` trait method is
        // unavailable here; read the injected `sub` from extensions and resolve it.
        let sub = request
            .extensions()
            .get::<String>()
            .cloned()
            .ok_or_else(|| Status::internal("Critical: Bouncer interceptor failed to inject external ID"))?;
        let user_id = resolve_internal_id(&self.app_state.cached_identity_resolver, &sub).await?;

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

        // Only the owner may stream content into a file.
        file_owned_by(&self.app_state, file_id, user_id).await?;

        let (tx, rx) = mpsc::channel(32);

        let app_state_clone = self.app_state.clone();

        let service_handle = tokio::spawn(async move {
            app_state_clone
                .file_write_service
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
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        file_owned_by(&self.app_state, file_id, user_id).await?;

        let command = UpdateFileNameCommand::new(req.new_name);

        let file = self
            .app_state
            .file_write_service
            .update_file_name(command, file_id)
            .await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn undelete_file(
        &self,
        request: Request<UndeleteFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        file_owned_by(&self.app_state, file_id, user_id).await?;

        let file = self
            .app_state
            .file_write_service
            .update_deleted_file(file_id)
            .await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn delete_chosen_files(
        &self,
        request: Request<DeleteChosenFilesRequest>,
    ) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_ids: Vec<Uuid> = req
            .file_ids
            .into_iter()
            .map(|id| map_entity_id(Some(id)))
            .collect::<Result<Vec<_>, _>>()?;

        for file_id in &file_ids {
            file_owned_by(&self.app_state, *file_id, user_id).await?;
        }

        self.app_state
            .file_write_service
            .delete_chosen_files(&file_ids)
            .await?;

        Ok(Response::new(()))
    }

    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.id)?;

        file_owned_by(&self.app_state, file_id, user_id).await?;

        self.app_state.file_write_service.delete(file_id).await?;

        Ok(Response::new(()))
    }

    async fn move_file(
        &self,
        request: Request<MoveFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        let folder_id = map_entity_id(req.folder_id)?;

        // Both the file and the destination folder must belong to the caller.
        file_owned_by(&self.app_state, file_id, user_id).await?;
        folder_owned_by(&self.app_state, folder_id, user_id).await?;

        let command = MoveFileCommand::new(folder_id, file_id);

        let file = self.app_state.file_write_service.move_file(command).await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn copy_file(
        &self,
        request: Request<CopyFileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        let target_folder_id = map_entity_id(req.target_folder_id)?;

        // Both the source file and the destination folder must belong to the caller.
        file_owned_by(&self.app_state, file_id, user_id).await?;
        folder_owned_by(&self.app_state, target_folder_id, user_id).await?;

        let command = CopyFileCommand::new(file_id, target_folder_id);

        let file = self.app_state.file_write_service.copy_file(command).await?;

        Ok(Response::new(map_file_to_proto(file)))
    }

    async fn update_file_content(
        &self,
        request: Request<Streaming<FileChunk>>,
    ) -> Result<Response<()>, Status> {
        // `Streaming<T>` is not `Sync`, so the `RequestIdentityExt` trait method is
        // unavailable here; read the injected `sub` from extensions and resolve it.
        let sub = request
            .extensions()
            .get::<String>()
            .cloned()
            .ok_or_else(|| Status::internal("Critical: Bouncer interceptor failed to inject external ID"))?;
        let user_id = resolve_internal_id(&self.app_state.cached_identity_resolver, &sub).await?;

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

        // Only the owner may overwrite a file's content.
        file_owned_by(&self.app_state, file_id, user_id).await?;

        let (tx, rx) = mpsc::channel(32);

        let app_state_clone = self.app_state.clone();

        let service_handle = tokio::spawn(async move {
            app_state_clone
                .file_write_service
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

    async fn archive_file(
        &self,
        request: Request<ArchiveFileRequest>,
    ) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        file_owned_by(&self.app_state, file_id, user_id).await?;

        self.app_state.file_write_service.archive_file(file_id).await?;

        Ok(Response::new(()))
    }

    async fn unarchive_file(
        &self,
        request: Request<UnarchiveFileRequest>,
    ) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        file_owned_by(&self.app_state, file_id, user_id).await?;

        self.app_state.file_write_service.unarchive_file(file_id).await?;

        Ok(Response::new(()))
    }


    async fn remove_delete_file(&self, request: Request<RemoveDeletedFileRequest>) -> Result<Response<()>, Status> {
        let user_id = request
            .get_internal_id(&self.app_state.cached_identity_resolver)
            .await?;

        let req = request.into_inner();

        let file_id = map_entity_id(req.file_id)?;

        file_owned_by(&self.app_state, file_id, user_id).await?;

        self.app_state.file_write_service.remove_deleted_file(file_id).await?;

        Ok(Response::new(()))
    }
}
