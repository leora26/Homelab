use crate::constants::MB;
use crate::data::file_folder::copy_file_command::CopyFileCommand;
use crate::data::file_folder::init_file_command::InitFileCommand;
use crate::data::file_folder::move_file_command::MoveFileCommand;
use crate::data::file_folder::update_file_name_command::UpdateFileNameCommand;
use crate::db::file_repository::FileRepository;
use crate::db::folder_repository::FolderRepository;
use crate::db::user_repository::UserRepository;
use crate::domain::file::{File, UploadStatus};
use crate::domain::folder::Folder;
use crate::domain::user::User;
use crate::exception::data_error::DataError;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use derive_new::new;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
use crate::db::global_file_repository::GlobalFileRepository;
use crate::domain::global_file::GlobalFile;
use crate::service::preview_service::{PreviewService, PreviewServiceImpl};

#[async_trait]
pub trait FileService: Send + Sync {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError>;
    async fn get_all_deleted_files(&self) -> Result<Vec<File>, DataError>;
    async fn search_file(&self, search_query: String) -> Result<Vec<File>, DataError>;
    async fn upload(&self, command: InitFileCommand) -> Result<File, DataError>;
    async fn upload_stream(
        &self,
        file_id: Uuid,
        rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError>;
    async fn update_file_name(
        &self,
        command: UpdateFileNameCommand,
        id: Uuid,
    ) -> Result<File, DataError>;
    async fn update_deleted_file(&self, id: Uuid) -> Result<File, DataError>;
    async fn delete_chosen_files(&self, file_ids: &[Uuid]) -> Result<(), DataError>;
    async fn delete(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn move_file(&self, command: MoveFileCommand) -> Result<File, DataError>;
    async fn copy_file(&self, command: CopyFileCommand) -> Result<File, DataError>;
    async fn update_stream (
        &self,
        file_id: Uuid,
        rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError>;
    async fn get_file_for_streaming(&self, file_id: Uuid) -> Result<PathBuf, DataError>;
}

#[derive(new)]
pub struct FileServiceImpl {
    file_repo: Arc<dyn FileRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    user_repo: Arc<dyn UserRepository>,
    storage_path: PathBuf,
    global_file_repo: Arc<dyn GlobalFileRepository>,
}

#[async_trait]
impl FileService for FileServiceImpl {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError> {
        self.file_repo.get_by_id(file_id).await
    }

    async fn get_all_deleted_files(&self) -> Result<Vec<File>, DataError> {
        self.file_repo.get_all_deleted().await
    }

    async fn search_file(&self, search_query: String) -> Result<Vec<File>, DataError> {
        self.file_repo
            .search_by_name(format!("%{}%", search_query))
            .await
    }

    async fn upload(&self, command: InitFileCommand) -> Result<File, DataError> {
        let folder: Folder = self
            .folder_repo
            .get_by_id(command.destination)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("Folder".to_string()))?;

        if let Some(_) = self
            .file_repo
            .get_by_folder_and_file_name(folder.id, command.name.clone())
            .await?
        {
            return Err(DataError::FileAlreadyExistsError);
        }

        let user: User = self
            .user_repo
            .get_by_id(command.owner_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        if user.validate_storage_size(command.expected_size) {
            let f = File::new(
                Uuid::new_v4(),
                command.name,
                user.id,
                folder.id,
                false,
                command.expected_size,
            );
            
            if command.is_global {
                let original = f.id.clone();
                let global_file = GlobalFile::new(Uuid::new_v4(), original);
                
                self.global_file_repo.save(global_file).await?;
            }
            
            self.file_repo.save(f).await
        } else {
            Err(DataError::NoFreeStorageError)
        }
    }

    async fn upload_stream(
        &self,
        file_id: Uuid,
        mut rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError> {
        let mut f = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        if f.upload_status != UploadStatus::Pending {
            return Err(DataError::ValidationError(
                "File is not pending".to_string(),
            ));
        }

        let file_path = f.build_file_path(&self.storage_path);

        // Make sure that parent directories exist.
        // Storing a bunch of files without parent directories that come build_file_path would be an unoptimized
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| DataError::IOError(format!("Failed to create buckets: {}", e)))?;
        }

        let file_handle = tokio::fs::File::create(&file_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let mut writer = BufWriter::with_capacity(MB as usize, file_handle);
        let mut total_bytes = 0i64;

        while let Some(chunk_result) = rx.recv().await {
            let data = chunk_result?;

            total_bytes += data.len() as i64;

            if let Err(e) = writer.write_all(&data).await {
                let _ = tokio::fs::remove_file(&file_path).await;
                return Err(DataError::IOError(e.to_string()));
            }
        }

        if let Err(e) = writer.flush().await {
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        // If the file size is not the same as provided during initialization,
        // we need to remove file from the disk since the file was probably corrupted
        if !f.validate_size(total_bytes) {
            f.update_status(UploadStatus::Failed);
            self.file_repo.update(f).await?;
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(DataError::NotMatchingByteSizeError);
        }

        f.update_status(UploadStatus::Completed);
        self.file_repo.update(f.clone()).await?;

        // After the file has been uploaded we need to create a preview of this file
        PreviewServiceImpl::spawn_generation(f, self.storage_path.clone());

        Ok(())
    }

    async fn update_file_name(
        &self,
        command: UpdateFileNameCommand,
        file_id: Uuid,
    ) -> Result<File, DataError> {
        let mut file: File = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.rename(command.new_name);

        self.file_repo.update(file).await
    }

    async fn update_deleted_file(&self, id: Uuid) -> Result<File, DataError> {
        let mut file: File = self
            .file_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_undeleted();

        self.file_repo.update(file).await
    }

    async fn delete_chosen_files(&self, file_ids: &[Uuid]) -> Result<(), DataError> {
        self.file_repo.delete_all(file_ids).await
    }

    async fn delete(&self, file_id: Uuid) -> Result<(), DataError> {
        let mut file = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_deleted();

        self.file_repo.delete_by_id(file_id).await
    }

    async fn move_file(&self, command: MoveFileCommand) -> Result<File, DataError> {
        let mut file = self
            .file_repo
            .get_by_id(command.file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.update_parent_folder(command.folder_id);

        Ok(self.file_repo.update(file).await?)
    }

    async fn copy_file(&self, command: CopyFileCommand) -> Result<File, DataError> {
        let file = self
            .file_repo
            .get_by_id(command.file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let user = self
            .user_repo
            .get_by_id(command.user_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

        if !user.validate_storage_size(file.size) {
            return Err(DataError::NoFreeStorageError);
        }

        let new_file_id = Uuid::new_v4();

        let mut new_file = File::new(
            new_file_id,
            format!("{}_copy", file.name.clone()),
            user.id,
            command.target_folder_id,
            false,
            file.size,
        );

        new_file.upload_status = UploadStatus::Completed;

        let source_path = file.build_file_path(&self.storage_path);
        let dest_path = new_file.build_file_path(&self.storage_path);

        if let Some(parent) = dest_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| DataError::IOError(format!("Failed to create buckets: {}", e)))?;
        }

        if let Err(e) = tokio::fs::copy(&source_path, &dest_path).await {
            return Err(DataError::IOError(e.to_string()));
        }

        match self.file_repo.save(new_file).await {
            Ok(uploaded_file) => Ok(uploaded_file),
            Err(err) => {
                if let Err(del_err) = tokio::fs::remove_file(&dest_path).await {
                    return Err(DataError::IOError(format!(
                        "Failed to delete ghost file. Needs immediate attention: {}",
                        del_err.to_string()
                    )));
                }

                Err(err)
            }
        }
    }

    async fn update_stream(&self, file_id: Uuid, mut rx: Receiver<Result<Vec<u8>, DataError>>) -> Result<(), DataError> {
        let mut f = self.file_repo.get_by_id(file_id).await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let target_path = f.build_file_path(&self.storage_path);

        let parent = target_path.parent()
            .ok_or_else(|| DataError::UnknownError("Invalid file path structure".to_string()))?;

        let temp_path = parent.join(format!("{}.tmp", f.id));

        let file_handle = tokio::fs::File::create(&temp_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let mut writer = BufWriter::with_capacity(MB as usize, file_handle);
        let mut new_total_bytes = 0i64;

        while let Some(chunk_result) = rx.recv().await {
            let data = chunk_result?;
            new_total_bytes += data.len() as i64;

            if let Err(e) = writer.write_all(&data).await {
                let _ = tokio::fs::remove_file(&temp_path).await;
                return Err(DataError::IOError(e.to_string()));
            }
        }

        if let Err(e) = writer.flush().await {
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        if let Err(e) = writer.get_ref().sync_all().await {
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        let old_size = f.size;
        let size_diff = new_total_bytes - old_size;

        if size_diff > 0 {
            let user = self.user_repo.get_by_id(f.owner_id).await?
                .ok_or_else(|| DataError::EntityNotFoundException("User".to_string()))?;

            if !user.validate_storage_size(size_diff) {
                let _ = tokio::fs::remove_file(&temp_path).await;
                return Err(DataError::NoFreeStorageError);
            }
        }

        if let Err(e) = tokio::fs::rename(&temp_path, &target_path).await {
            let _ = tokio::fs::remove_file(&temp_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        f.update_size(new_total_bytes);
        self.file_repo.update(f).await?;

        Ok(())
    }

    async fn get_file_for_streaming(&self, file_id: Uuid) -> Result<PathBuf, DataError> {
        let file = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let file_path = file.build_file_path(&self.storage_path);

        if !file_path.exists() {
            return Err(DataError::IOError("File metadata exists but disk file is missing".to_string()));
        }

        Ok(file_path)
    }
}
