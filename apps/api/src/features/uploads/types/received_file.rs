use chrono::{DateTime, Utc};

use crate::features::uploads::types::uploader::Uploader;

pub struct ReceivedFile {
    pub name: String,
    pub original_name: String,
    pub size: u64,
    pub sha256: String,
    pub received_at: DateTime<Utc>,
    pub uploader: Uploader,
}
