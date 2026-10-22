use std::fmt::Write;

use crate::{
    features::uploads::types::received_file::ReceivedFile, libs::log_timestamp::log_timestamp,
};

pub fn received_file_log_entry(file: &ReceivedFile, size: &str, device: &str) -> String {
    let mut entry = format!("[{}] {}\n", log_timestamp(file.received_at), file.name);

    if file.name != file.original_name {
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
