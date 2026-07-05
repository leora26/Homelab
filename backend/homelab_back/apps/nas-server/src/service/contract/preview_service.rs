use std::path::PathBuf;
use homelab_core::nas_domain::file::File;

pub trait PreviewService {
    fn spawn_generation(file: File, storage_path: PathBuf);
}
