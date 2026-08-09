use homelab_proto::admin::{SetVolumeSizeResponse, VolumeStatusResponse};

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
