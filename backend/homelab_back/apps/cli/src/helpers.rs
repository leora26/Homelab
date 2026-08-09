use anyhow::{bail, Context, Result};
use tonic::{Code, Status};

pub fn friendly_grpc_error(status: &Status) -> String {
    let message = unwrap_nested_message(status.message());

    match status.code() {
        Code::FailedPrecondition | Code::InvalidArgument | Code::OutOfRange => message,
        Code::Unavailable => format!("cannot reach the storage service: {message}"),
        Code::DeadlineExceeded => format!("the request timed out: {message}"),
        Code::PermissionDenied => format!("permission denied: {message}"),
        Code::NotFound => format!("not found: {message}"),
        _ => message,
    }
}

fn unwrap_nested_message(msg: &str) -> String {
    let mut current = msg;
    while let Some(start) = current.find("message: \"") {
        let after = &current[start + "message: \"".len()..];
        match after.find("\", details:") {
            Some(end) => current = &after[..end],
            None => break,
        }
    }
    current.to_string()
}

pub fn parse_size(size: &String) -> Result<i64> {
    let size = size.trim();
    if size.is_empty() {
        bail!("size cannot be empty");
    }

    let split = size.find(|c: char| !c.is_ascii_digit()).unwrap_or(size.len());
    let (num, unit) = size.split_at(split);

    let value: i64 = num
        .parse()
        .with_context(|| format!("'{num}' is not a whole number in size '{size}'"))?;

    if value <= 0 {
        bail!("size must be greater than zero (got '{size}')");
    }

    let multiplier: i64 = match unit.trim().to_ascii_uppercase().as_str() {
        "" | "B" => 1,
        "K" | "KB" | "KIB" => 1024,
        "M" | "MB" | "MIB" => 1024 * 1024,
        "G" | "GB" | "GIB" => 1024 * 1024 * 1024,
        "T" | "TB" | "TIB" => 1024 * 1024 * 1024 * 1024,
        other => bail!("unknown unit '{other}' in size '{size}' — use B, K, M, G, or T"),
    };

    value
        .checked_mul(multiplier)
        .with_context(|| format!("size '{size}' is too large"))
}