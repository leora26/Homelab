use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use derive_new::new;
use tokio::process::Command;
use tokio::sync::Mutex;
use homelab_core::nas_domain::volume::{ResizeOutcome, VolumeStatus};
use crate::helpers::data_error::DataError;
use crate::service::contract::volume_service::VolumeService;

#[derive(new)]
pub struct VolumeServiceImpl {
    pool: String,
    dataset: String,
    headroom: i64,
    #[new(default)]
    lock: Mutex<()>
}

#[async_trait]
impl VolumeService for VolumeServiceImpl {
    async fn status(&self) -> Result<VolumeStatus, DataError> {
        self.read_status().await
    }

    async fn resize(&self, requested_bytes: i64, force_shrink: bool) -> Result<ResizeOutcome, DataError> {
        let _guard = self.lock.lock().await;
        let status = self.read_status().await?;

        if status.quota == Some(requested_bytes) && status.reservation == Some(requested_bytes){
            return Ok(ResizeOutcome::NoChange(status))
        }

        validate_resize(requested_bytes, &status, self.headroom, force_shrink)?;

        run("zfs", &["set",
            &format!("quota={requested_bytes}"),
            &format!("reservation={requested_bytes}"),
            &self.dataset]).await?;

        Ok(ResizeOutcome::Resized { from: status.quota, to: requested_bytes })
    }

    async fn ensure_mounted(&self) -> Result<PathBuf, DataError> {
        let out = run("zfs", &["get", "-Hp", "-o", "property,value", "mounted,mountpoint", &self.dataset]).await?;
        let m = parse_pairs(&out);
        let mountpoint = PathBuf::from(field(&m, "mountpoint")?);
        if field(&m, "mounted")? != "yes" {
            return Err(DataError::VolumeNotMounted(format!(
                "ZFS dataset '{}' is not mounted at {} — refusing to start so files are never written outside the volume",
                self.dataset, mountpoint.display()
            )));
        }
        Ok(mountpoint)
    }
}

impl VolumeServiceImpl {
    async fn read_status(&self) -> Result<VolumeStatus, DataError> {
        let props = "used,available,quota,reservation,referenced,usedbysnapshots,mountpoint";
        let out = run("zfs", &["get", "-Hp", "-o", "property,value", props, &self.dataset]).await?;
        let m = parse_pairs(&out);
        let pool_free = parse_i64(run("zpool", &["list","-Hp","-o","free",&self.pool]).await?.trim())?;

        Ok(VolumeStatus::new(
            self.dataset.clone(),
            PathBuf::from(field(&m, "mountpoint")?),
            parse_i64(field(&m, "used")?)?,
            parse_i64(field(&m, "available")?)?,
            parse_opt_i64(field(&m, "quota")?)?,
            parse_opt_i64(field(&m, "reservation")?)?,
            parse_i64(field(&m, "referenced")?)?,
            parse_i64(field(&m, "usedbysnapshots")?)?,
            pool_free,
        ))
    }
}

fn validate_resize(requested: i64, s: &VolumeStatus, headroom: i64, force_shrink: bool) -> Result<(), DataError> {
    if requested <= 0 {
        return Err(DataError::ValidationError(format!("size must be > 0 (got {requested})")));
    }
    if requested < s.used + headroom && !force_shrink {
        return Err(DataError::VolumeResizeRejected(format!(
            "cannot shrink to {requested} bytes: dataset uses {} bytes (need ≥ used + {headroom} headroom)", s.used)));
    }
    let current_res = s.reservation.unwrap_or(0);
    if requested > current_res && (requested - current_res) > s.pool_free {
        return Err(DataError::VolumeResizeRejected(format!(
            "cannot grow by {} bytes: pool has only {} bytes free", requested - current_res, s.pool_free)));
    }
    Ok(())
}

async fn run(program: &str, args: &[&str]) -> Result<String, DataError> {
    let output = Command::new(program).args(args).output().await
        .map_err(|e| DataError::IOError(format!("failed to spawn {program}: {e}")))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(DataError::ZfsCommandError(format!("`{program} {}` failed: {stderr}", args.join(" "))))
    }
}
fn parse_pairs(out: &str) -> HashMap<String, String> {
    out.lines().filter_map(|l| { let mut c = l.split('\t');
        match (c.next(), c.next()) { (Some(k), Some(v)) => Some((k.into(), v.into())), _ => None } }).collect()
}
fn field<'a>(m: &'a HashMap<String, String>, k: &str) -> Result<&'a str, DataError> {
    m.get(k).map(String::as_str).ok_or_else(|| DataError::ZfsCommandError(format!("missing property '{k}'")))
}
fn parse_i64(v: &str) -> Result<i64, DataError> {
    v.trim().parse().map_err(|_| DataError::ZfsCommandError(format!("expected integer bytes, got '{v}'")))
}
fn parse_opt_i64(v: &str) -> Result<Option<i64>, DataError> {
    match v.trim() { "none" | "-" | "0" => Ok(None), o => Ok(Some(parse_i64(o)?)) }
}