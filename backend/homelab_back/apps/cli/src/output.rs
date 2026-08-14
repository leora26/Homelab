use homelab_proto::admin::{
    ConsoleFileListResponse, ConsoleUserListResponse, SetVolumeSizeResponse, VolumeStatusResponse,
};
use homelab_proto::common::EntityId;
use tabled::settings::Style;
use tabled::{Table, Tabled};

fn human_bytes(n: i64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = n as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{n} B")
    } else {
        format!("{size:.2} {}", UNITS[unit])
    }
}

const BAR_WIDTH: usize = 30;

fn usage_bar(used: i64, quota: Option<i64>) -> String {
    match quota {
        Some(q) if q > 0 => {
            let frac = (used as f64 / q as f64).clamp(0.0, 1.0);
            let filled = (frac * BAR_WIDTH as f64).round() as usize;
            let bar = "█".repeat(filled) + &"░".repeat(BAR_WIDTH - filled);
            format!("[{bar}]  {:.1}%", frac * 100.0)
        }
        _ => format!("[{}]  (no quota)", "░".repeat(BAR_WIDTH)),
    }
}

pub fn print_volume_status_human(s: &VolumeStatusResponse) {
    let quota = s.quota.map(human_bytes).unwrap_or_else(|| "∞".to_string());

    println!("{}  →  {}", s.dataset, s.mountpoint);
    println!();
    println!("  {}", usage_bar(s.used, s.quota));
    println!("  {} used of {} quota", human_bytes(s.used), quota);
    println!();
    println!("  Available     {}", human_bytes(s.available));
    println!(
        "  Reservation   {}   (guaranteed floor)",
        s.reservation
            .map(human_bytes)
            .unwrap_or_else(|| "none".to_string())
    );
    println!("  Snapshots     {}", human_bytes(s.used_by_snapshots));
    println!("  Pool free     {}", human_bytes(s.pool_free));
}

pub fn print_volume_json(s: &VolumeStatusResponse) -> anyhow::Result<()> {
    let used_percent = match s.quota {
        Some(q) if q > 0 => Some((s.used as f64 / q as f64) * 100.0),
        _ => None,
    };

    let value = serde_json::json!({
        "dataset": s.dataset,
        "mountpoint": s.mountpoint,
        "used_bytes": s.used,
        "available_bytes": s.available,
        "quota_bytes": s.quota,
        "reservation_bytes": s.reservation,
        "referenced_bytes": s.referenced,
        "used_by_snapshots_bytes": s.used_by_snapshots,
        "pool_free_bytes": s.pool_free,
        "used_percent": used_percent,
    });

    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

pub fn print_resize(r: &SetVolumeSizeResponse) {
    if r.changed {
        let previous = r
            .previous_bytes
            .map(human_bytes)
            .unwrap_or_else(|| "unset".to_string());
        println!("Resized: {} → {}", previous, human_bytes(r.current_bytes));
    } else {
        println!("No change: already {}", human_bytes(r.current_bytes));
    }

    if let Some(status) = &r.status {
        println!();
        print_volume_status_human(status);
    }
}

// ---- file table -------------------------------------------------------------

#[derive(Tabled)]
struct FileRow {
    #[tabled(rename = "FILE ID")]
    file_id: String,
    #[tabled(rename = "TYPE")]
    file_type: String,
    #[tabled(rename = "STATUS")]
    status: String,
    #[tabled(rename = "SIZE")]
    size: String,
    #[tabled(rename = "DEL")]
    deleted: String,
    #[tabled(rename = "ARCH")]
    archived: String,
    #[tabled(rename = "VER")]
    version: String,
    #[tabled(rename = "UPDATED")]
    updated: String,
}

/// Render any file list (log / latest / matches / versions) as a table.
pub fn print_file_table(list: &ConsoleFileListResponse) {
    if list.files.is_empty() {
        println!("No files.");
        return;
    }

    let rows: Vec<FileRow> = list
        .files
        .iter()
        .map(|f| FileRow {
            file_id: short_id(&f.file_id),
            file_type: file_type_name(f.file_type),
            status: upload_status_name(f.upload_status),
            size: human_bytes(f.size),
            deleted: flag(f.is_deleted),
            archived: flag(f.is_archived),
            version: f.version.to_string(),
            updated: relative_time(f.updated_at.as_ref().map(|t| t.seconds)),
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::psql());
    println!("{table}");
}

// ---- user table -------------------------------------------------------------

#[derive(Tabled)]
struct UserRow {
    #[tabled(rename = "USER ID")]
    user_id: String,
    #[tabled(rename = "EMAIL")]
    email: String,
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "QUOTA")]
    quota: String,
    #[tabled(rename = "USED")]
    used: String,
    #[tabled(rename = "USE%")]
    use_pct: String,
    #[tabled(rename = "BLK")]
    blocked: String,
    #[tabled(rename = "VER")]
    version: String,
    #[tabled(rename = "UPDATED")]
    updated: String,
}

/// Render any user list (log / latest / matches / versions) as a table.
pub fn print_user_table(list: &ConsoleUserListResponse) {
    if list.users.is_empty() {
        println!("No users.");
        return;
    }

    let rows: Vec<UserRow> = list
        .users
        .iter()
        .map(|u| UserRow {
            user_id: short_id(&u.user_id),
            email: u.email.clone(),
            name: u.full_name.clone(),
            quota: human_bytes(u.allowed_storage),
            used: human_bytes(u.taken_storage),
            use_pct: usage_percent(u.taken_storage, u.allowed_storage),
            blocked: flag(u.is_blocked),
            version: u.version.to_string(),
            updated: relative_time(u.updated_at.as_ref().map(|t| t.seconds)),
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::psql());
    println!("{table}");
}

/// "42%" from used/quota, guarding against a zero or negative quota.
fn usage_percent(used: i64, quota: i64) -> String {
    if quota <= 0 {
        return "-".to_string();
    }
    format!("{:.0}%", (used as f64 / quota as f64) * 100.0)
}

fn short_id(id: &Option<EntityId>) -> String {
    match id {
        Some(e) => e.value.chars().take(12).collect(),
        None => "-".to_string(),
    }
}

fn flag(on: bool) -> String {
    if on { "✓".to_string() } else { String::new() }
}

fn file_type_name(v: i32) -> String {
    use homelab_proto::nas::FileType;
    match FileType::try_from(v) {
        Ok(FileType::Image) => "image",
        Ok(FileType::Text) => "text",
        Ok(FileType::Video) => "video",
        Ok(FileType::Audio) => "audio",
        Ok(FileType::Pdf) => "pdf",
        Ok(FileType::Zip) => "zip",
        Ok(FileType::Unknown) => "unknown",
        Err(_) => "?",
    }
    .to_string()
}

fn upload_status_name(v: i32) -> String {
    use homelab_proto::nas::UploadStatus;
    match UploadStatus::try_from(v) {
        Ok(UploadStatus::Pending) => "pending",
        Ok(UploadStatus::Completed) => "completed",
        Ok(UploadStatus::Failed) => "failed",
        Err(_) => "?",
    }
    .to_string()
}

/// Human-friendly "3m ago" from a unix-seconds timestamp.
fn relative_time(seconds: Option<i64>) -> String {
    let Some(ts) = seconds else {
        return "-".to_string();
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let diff = now - ts;
    if diff < 0 {
        return "just now".to_string();
    }
    if diff < 60 {
        return format!("{diff}s ago");
    }
    let mins = diff / 60;
    if mins < 60 {
        return format!("{mins}m ago");
    }
    let hours = mins / 60;
    if hours < 24 {
        return format!("{hours}h ago");
    }
    format!("{}d ago", hours / 24)
}
