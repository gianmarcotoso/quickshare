use chrono::Utc;

use crate::{
    features::uploads::batch::Batch,
    libs::{format_bytes::format_bytes, log_timestamp::log_timestamp},
};

pub fn finish_batch(batch: &Batch) -> std::io::Result<()> {
    let Some(summary) = batch.close(|files, bytes| {
        format!(
            "[{}] batch closed: {files} files, {} ({bytes} bytes)\n",
            log_timestamp(Utc::now()),
            format_bytes(bytes)
        )
    })?
    else {
        return Ok(());
    };

    tracing::info!(
        folder = %batch.directory().display(),
        files = summary.files,
        size = %format_bytes(summary.bytes),
        log_sha256 = %summary.log_sha256,
        "batch closed"
    );

    Ok(())
}
