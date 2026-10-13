use std::fmt::Write;

use crate::{
    features::uploads::types::{batch::Batch, received_file::ReceivedFile},
    libs::{format_bytes::format_bytes, log_timestamp::log_timestamp},
};

pub fn record_received_file(batch: &Batch, file: &ReceivedFile) -> std::io::Result<bool> {
    let size = format_bytes(file.size);
    let device = file.uploader.device();

    if !batch.record_file(&log_entry(file, &size, &device), file.size)? {
        return Ok(false);
    }

    if file.is_renamed() {
        tracing::info!(name = %file.name, original = %file.original_name, %size, sha256 = %file.sha256, %device, "file received");
    } else {
        tracing::info!(name = %file.name, %size, sha256 = %file.sha256, %device, "file received");
    }

    Ok(true)
}

fn log_entry(file: &ReceivedFile, size: &str, device: &str) -> String {
    let mut entry = format!("[{}] {}\n", log_timestamp(file.received_at), file.name);

    if file.is_renamed() {
        let _ = writeln!(entry, "  original:   {}", file.original_name);
    }

    let _ = writeln!(entry, "  size:       {size} ({} bytes)", file.size);
    let _ = writeln!(entry, "  sha256:     {}", file.sha256);
    let _ = writeln!(entry, "  device:     {device}");
    let _ = writeln!(entry, "  ip:         {}", file.uploader.ip);
    let _ = writeln!(entry, "  user agent: {}", file.uploader.user_agent);
    entry.push('\n');

    entry
}
