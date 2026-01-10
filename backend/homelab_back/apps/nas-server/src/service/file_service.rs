use homelab_core::constants::MB;
use crate::data::copy_file_command::CopyFileCommand;
use crate::data::init_file_command::InitFileCommand;
use crate::data::move_file_command::MoveFileCommand;
use crate::data::update_file_name_command::UpdateFileNameCommand;
use crate::db::file_repository::FileRepository;
use crate::db::folder_repository::FolderRepository;
use crate::db::global_file_repository::GlobalFileRepository;
use crate::db::user_repository::UserRepository;
use homelab_core::file::{File, UploadStatus};
use homelab_core::folder::Folder;
use homelab_core::global_file::GlobalFile;
use homelab_core::user::User;
use crate::helpers::data_error::DataError;
use crate::service::preview_service::{PreviewService, PreviewServiceImpl};
use async_compression::tokio::write::{GzipDecoder, GzipEncoder};
use async_trait::async_trait;
use derive_new::new;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::{fs};
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
use futures::stream::{self, StreamExt};

#[async_trait]
pub trait FileService: Send + Sync {
    async fn get_by_id(&self, file_id: Uuid) -> Result<Option<File>, DataError>;
    async fn get_all_deleted_files(&self, user_id: Uuid) -> Result<Vec<File>, DataError>;
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
    async fn update_stream(
        &self,
        file_id: Uuid,
        rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError>;
    async fn get_file_for_streaming(&self, file_id: Uuid) -> Result<PathBuf, DataError>;
    async fn archive_file(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn unarchive_file(&self, file_id: Uuid) -> Result<(), DataError>;
    async fn cleanup_deleted_files(&self, user_id: Uuid) -> Result<(), DataError>;
    async fn cleanup_expired_files(&self) -> Result<(), DataError>;
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

    async fn get_all_deleted_files(&self, user_id: Uuid) -> Result<Vec<File>, DataError> {
        self.file_repo.get_all_deleted(user_id).await
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
        let files = self.file_repo.get_all_by_ids(file_ids).await?;

        for mut file in files {
            file.set_as_deleted();
            self.file_repo.update(file).await.map_err(|e| e)?;
        }

        Ok(())
    }

    async fn delete(&self, file_id: Uuid) -> Result<(), DataError> {
        let mut file = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        file.set_as_deleted();

        let _ = self.file_repo.update(file).await?;

        Ok(())
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

    async fn update_stream(
        &self,
        file_id: Uuid,
        mut rx: Receiver<Result<Vec<u8>, DataError>>,
    ) -> Result<(), DataError> {
        let mut f = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let target_path = f.build_file_path(&self.storage_path);

        let parent = target_path
            .parent()
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
            let user = self
                .user_repo
                .get_by_id(f.owner_id)
                .await?
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
            return Err(DataError::IOError(
                "File metadata exists but disk file is missing".to_string(),
            ));
        }

        Ok(file_path)
    }

    async fn archive_file(&self, file_id: Uuid) -> Result<(), DataError> {
        let file = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        if file.is_archived(&self.storage_path) {
            return Err(DataError::FileIsNotArchivedError);
        }

        let original_path = file.build_file_path(&self.storage_path);

        let mut compressed_path = original_path.clone();
        compressed_path.set_extension("gz");

        let mut source_file = fs::File::open(&original_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let dest_file = fs::File::create(&compressed_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let buffered_writer = BufWriter::new(dest_file);
        let mut encoder = GzipEncoder::new(buffered_writer);

        if let Err(e) = tokio::io::copy(&mut source_file, &mut encoder).await {
            let _ = fs::remove_file(&compressed_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        if let Err(e) = encoder.shutdown().await {
            let _ = fs::remove_file(&compressed_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        fs::remove_file(&original_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        Ok(())
    }

    async fn unarchive_file(&self, file_id: Uuid) -> Result<(), DataError> {
        let file = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        if !file.is_archived(&self.storage_path) {
            return Err(DataError::FileIsAlreadyArchivedError);
        }

        let compressed_path = file.build_file_path(&self.storage_path);

        let mut output_path = compressed_path.clone();
        output_path.set_extension("");

        let source_file = fs::File::open(&compressed_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let reader = BufReader::new(source_file);
        let mut decoder = GzipDecoder::new(reader);

        let dest_file = fs::File::create(&output_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        let mut buffered_writer = BufWriter::new(dest_file);

        if let Err(e) = tokio::io::copy(&mut decoder, &mut buffered_writer).await {
            let _ = fs::remove_file(&output_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        if let Err(e) = buffered_writer.flush().await {
            let _ = fs::remove_file(&output_path).await;
            return Err(DataError::IOError(e.to_string()));
        }

        fs::remove_file(&compressed_path)
            .await
            .map_err(|e| DataError::IOError(e.to_string()))?;

        Ok(())
    }

    async fn cleanup_deleted_files(&self, user_id: Uuid) -> Result<(), DataError> {
        let deleted_files = self.file_repo.get_all_deleted(user_id).await?;

        if deleted_files.is_empty() {
            return Ok(());
        }
        
        self.remove_deleted_files(deleted_files).await
    }

    async fn cleanup_expired_files(&self) -> Result<(), DataError> {
        let expired_files = self.file_repo.get_expired_files().await?;
        
        if expired_files.is_empty() {
            return Ok(());  
        }
        
        self.remove_deleted_files(expired_files).await
    }
}

impl FileServiceImpl {
    async fn remove_deleted_files(&self, deleted_files: Vec<File>) -> Result<(), DataError> {

        const CONCURRENCY_LIMIT: usize = 10;

        let results = stream::iter(deleted_files)
            .map(|file| async move {
                let path = file.build_file_path(&self.storage_path);

                let remove_result = match fs::remove_file(&path).await {
                    Ok(_) => Ok(file.id),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        let mut gz_path = path.clone();
                        gz_path.set_extension("gz");

                        match fs::remove_file(gz_path).await {
                            Ok(_) => Ok(file.id),
                            Err(e2) => Err((file.id, e2))
                        }
                    }
                    Err(e) => Err((file.id, e)),
                };

                remove_result
            })
            .buffer_unordered(CONCURRENCY_LIMIT)
            .collect::<Vec<_>>()
            .await;

        let mut success_results = Vec::new();
        let mut error_results = Vec::new();

        for res in results {
            match res {
                Ok(id) => success_results.push(id),
                Err(e) => error_results.push(e)
            }
        }

        if !success_results.is_empty() {
            self.file_repo.delete_by_ids(&success_results).await?;
        }

        if !error_results.is_empty() {
            return Err(DataError::IOError(format!(
                "Failed to delete {} files from disk. Check logs.",
                error_results.len()
            )));
        }

        Ok(())
    }
}
