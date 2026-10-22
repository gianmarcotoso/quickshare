use axum::extract::multipart::Field;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::core::action_error::ActionError;

pub async fn write_field(field: &mut Field<'_>, file: File) -> Result<(u64, String), ActionError> {
    let mut writer = BufWriter::new(file);
    let mut hasher = Sha256::new();
    let mut size = 0u64;

    while let Some(chunk) = field.chunk().await? {
        hasher.update(&chunk);
        size += chunk.len() as u64;
        writer
            .write_all(&chunk)
            .await
            .map_err(ActionError::internal)?;
    }

    writer.flush().await.map_err(ActionError::internal)?;
    writer
        .into_inner()
        .sync_all()
        .await
        .map_err(ActionError::internal)?;

    Ok((size, hex::encode(hasher.finalize())))
}
