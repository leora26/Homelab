use std::sync::Arc;
use async_trait::async_trait;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::AppState;
use crate::data::file_folder::update_folder_name_command::UpdateFolderNameCommand;
use crate::helpers::proto_mappers::{map_entity_id, map_file_to_proto, map_folder_to_proto};
use crate::pb::folder_service_server::FolderService;
use crate::pb::{DeleteAllFolderRequest, DeleteFolderRequest, FileListResponse, FolderResponse, FolderResponseList, GetAllSubfoldersRequest, GetFilesForFolderRequest, GetFolderRequest, RenameFolderRequest, SearchFolderRequest};

pub struct GrpcFolderService {
    app_state: Arc<AppState>
}

#[async_trait]
impl FolderService for GrpcFolderService {
    async fn get_folder(&self, request: Request<GetFolderRequest>) -> Result<Response<FolderResponse>, Status> {
        let req = request.into_inner();

        let folder_id = map_entity_id(req.id)?;

        let folder = self.app_state.folder_service.get_by_id(folder_id)
            .await?
            .ok_or_else(|| Status::not_found(format!("No folder found with id: {}", folder_id)))?;

        Ok(Response::new(map_folder_to_proto(folder)))
    }

    async fn get_subfolders(&self, request: Request<GetAllSubfoldersRequest>) -> Result<Response<FolderResponseList>, Status> {
        let req = request.into_inner();

        let folder_id = map_entity_id(req.id)?;

        let folders = self.app_state.folder_service.get_children_by_id(folder_id).await?;

        let proto_folders = folders.into_iter().map(|f| map_folder_to_proto(f)).collect();

        Ok(Response::new(FolderResponseList {folders: proto_folders}))
    }

    async fn delete_folder(&self, request: Request<DeleteFolderRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let folder_id = map_entity_id(req.id)?;

        self.app_state.folder_service.delete(folder_id).await?;

        Ok(Response::new(()))
    }

    async fn get_files_for_folder(&self, request: Request<GetFilesForFolderRequest>) -> Result<Response<FileListResponse>, Status> {
        let req = request.into_inner();

        let folder_id = map_entity_id(req.id)?;

        let files = self.app_state.folder_service.get_by_folder(folder_id).await?;

        let proto_files = files.into_iter().map(|f| map_file_to_proto(f)).collect();

        Ok(Response::new(FileListResponse {files: proto_files}))
    }

    async fn rename_folder(&self, request: Request<RenameFolderRequest>) -> Result<Response<FolderResponse>, Status> {
        let req = request.into_inner();

        let folder_id = map_entity_id(req.id)?;

        let command = UpdateFolderNameCommand::new(req.new_name);

        let folder = self.app_state.folder_service.update_folder_name(command, folder_id).await?;

        Ok(Response::new(map_folder_to_proto(folder)))
    }

    async fn search_folder(&self, request: Request<SearchFolderRequest>) -> Result<Response<FolderResponseList>, Status> {
        let req = request.into_inner();

        let folders = self.app_state.folder_service.search_folder(req.query).await?;

        let proto_folders = folders.into_iter().map(|f| map_folder_to_proto(f)).collect();

        Ok(Response::new(FolderResponseList {folders: proto_folders}))
    }

    async fn delete_chosen_folders(&self, request: Request<DeleteAllFolderRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let folder_ids: Vec<Uuid> = req.id
            .into_iter()
            .map(|folder_id| map_entity_id(Some(folder_id)))
            .collect::<Result<Vec<_>, _>>()?;

        self.app_state.folder_service.delete_chosen_folders(&folder_ids).await?;

        Ok(Response::new(()))
    }
}