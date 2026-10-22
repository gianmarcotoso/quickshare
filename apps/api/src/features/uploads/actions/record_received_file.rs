use crate::{
    features::uploads::{
        batch::Batch, libs::received_file_log_entry::received_file_log_entry,
        types::received_file::ReceivedFile,
    },
    libs::{describe_device::describe_device, format_bytes::format_bytes},
};

pub fn record_received_file(batch: &Batch, file: &ReceivedFile) -> std::io::Result<bool> {
    let size = format_bytes(file.size);
    let device = describe_device(&file.uploader.user_agent);

    if !batch.record_file(&received_file_log_entry(file, &size, &device), file.size)? {
        return Ok(false);
    }

    if file.name != file.original_name {
        tracing::info!(name = %file.name, original = %file.original_name, %size, sha256 = %file.sha256, %device, "file received");
    } else {
        tracing::info!(name = %file.name, %size, sha256 = %file.sha256, %device, "file received");
    }

    Ok(true)
}
