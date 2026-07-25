use std::path::PathBuf;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, new, Deserialize, Clone)]
pub struct VolumeStatus {
    pub dataset: String,
    pub mountpoint: PathBuf,
    pub used: i64,
    pub available: i64,
    pub quota: Option<i64>,
    pub reservation: Option<i64>,
    pub referenced: i64,
    pub used_by_snapshots: i64,
    pub pool_free: i64,
}


#[derive(Debug, Clone)]
pub enum ResizeOutcome {
    NoChange(VolumeStatus),
    Resized {from: Option<i64>, to: i64},
}

