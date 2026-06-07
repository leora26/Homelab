use async_trait::async_trait;
use uuid::Uuid;
use homelab_core::folder::Folder;
use crate::data::create_folder_command::CreateFolderCommand;
use crate::data::move_folder_command::MoveFolderCommand;
use crate::data::update_folder_name_command::UpdateFolderNameCommand;
use crate::helpers::data_error::DataError;

#[async_trait]
pub trait FolderWriteService: Send + Sync {
    async fn update_folder_name(
        &self,
        command: UpdateFolderNameCommand,
        folder_id: Uuid,
    ) -> Result<Folder, DataError>;
    async fn trash_chosen_folders(&self, folder_ids: &[Uuid]) -> Result<(), DataError>;
    async fn trash(&self, folder_id: Uuid) -> Result<(), DataError>;
    async fn create(&self, command: CreateFolderCommand) -> Result<Folder, DataError>;
    async fn move_folder(&self, command: MoveFolderCommand) -> Result<Folder, DataError>;
    async fn clean_up_trash(&self, user_id: Uuid) -> Result<(), DataError>;
    async fn permanently_delete_folder(
        &self,
        folder_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DataError>;
    async fn restore_deleted_folder(&self, folder_id: Uuid) -> Result<(), DataError>;
}