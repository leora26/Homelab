use std::path::PathBuf;
use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use uuid::Uuid;
use homelab_core::nas_domain::file::File;
use crate::db::file_repository::FileRepository;
use crate::helpers::data_error::DataError;
use crate::service::contract::file_read_service::FileReadService;

#[derive(new)]
pub struct FileReadServiceImpl {
    file_repo: Arc<dyn FileRepository>,
    storage_path: PathBuf,
}

#[async_trait]
impl FileReadService for FileReadServiceImpl {
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

    async fn get_file_preview_for_streaming(&self, file_id: Uuid) -> Result<PathBuf, DataError> {
        let file = self
            .file_repo
            .get_by_id(file_id)
            .await?
            .ok_or_else(|| DataError::EntityNotFoundException("File".to_string()))?;

        let original_path = file.build_file_path(&self.storage_path);

        if !original_path.exists() {
            return Err(DataError::IOError(
                "File metadata exists but disk file is missing".to_string(),
            ));
        }
        
        let png_preview = original_path.with_extension("preview.png");
        let jpg_preview = original_path.with_extension("preview.jpg");
        
        if png_preview.exists() {
            return Ok(png_preview);
        }
        
        if jpg_preview.exists() {
            return Ok(jpg_preview);
        }

        Err(DataError::IOError(
            "Preview for a file does not exist".to_string(),
        ))
    }
}
