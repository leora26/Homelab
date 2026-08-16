use crate::types::model::MachineInfoView;
use local_ip_address::{list_afinet_netifas, local_ip};
use std::fs;
use std::net::IpAddr;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_machine_info(app: AppHandle) -> Result<MachineInfoView, String> {
    Ok(MachineInfoView {
        hostname: read_hostname(),
        address: lan_address(),
        uptime_seconds: uptime_seconds(),
        app_version: app.package_info().version.to_string(),
    })
}

fn read_hostname() -> String {
    hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string())
}

fn lan_address() -> String {
    if let Ok(IpAddr::V4(v4)) = local_ip() {
        if is_usable(&v4) {
            return v4.to_string();
        }
    }

    list_afinet_netifas()
        .ok()
        .and_then(|ifaces| {
            ifaces.into_iter().find_map(|(_, ip)| match ip {
                IpAddr::V4(v4) if is_usable(&v4) => Some(v4.to_string()),
                _ => None,
            })
        })
        .unwrap_or_else(|| "unavailable".to_string())
}

fn is_usable(ip: &std::net::Ipv4Addr) -> bool {
    !ip.is_loopback() && !ip.is_unspecified() && !ip.is_link_local()
}

fn uptime_seconds() -> i64 {
    fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok())
        .map(|secs| secs as i64)
        .unwrap_or(0)
}
