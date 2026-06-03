use std::path::PathBuf;
use homelab_core::file::File;

pub trait PreviewService {
    fn spawn_generation(file: File, storage_path: PathBuf);
}
