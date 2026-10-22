use std::{path::PathBuf, sync::Arc};

use chrono::Utc;

use crate::{features::uploads::batch::Batch, libs::log_timestamp::log_timestamp};

pub struct PartialFile {
    batch: Arc<Batch>,
    path: PathBuf,
    original_name: String,
    kept: bool,
}

impl PartialFile {
    pub fn new(batch: Arc<Batch>, path: PathBuf, original_name: String) -> Self {
        Self {
            batch,
            path,
            original_name,
            kept: false,
        }
    }

    pub fn keep(mut self) {
        self.kept = true;
    }
}

impl Drop for PartialFile {
    fn drop(&mut self) {
        if self.kept {
            return;
        }

        let _ = std::fs::remove_file(&self.path);

        tracing::warn!(name = %self.original_name, batch = %self.batch.id(), "upload not completed, partial file deleted");

        let _ = self.batch.append(&format!(
            "[{}] upload not completed, partial file deleted: {}\n\n",
            log_timestamp(Utc::now()),
            self.original_name
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::uploads::constants::BATCH_LOG;

    #[test]
    fn a_partial_file_is_deleted_and_noted_unless_kept() {
        let data = tempfile::TempDir::new().unwrap();
        let batch = Arc::new(Batch::create(data.path(), Utc::now()).unwrap());

        let dropped = batch.directory().join("dropped.bin");
        std::fs::write(&dropped, b"partial").unwrap();
        drop(PartialFile::new(
            batch.clone(),
            dropped.clone(),
            "dropped.bin".to_string(),
        ));

        let kept = batch.directory().join("kept.bin");
        std::fs::write(&kept, b"complete").unwrap();
        PartialFile::new(batch.clone(), kept.clone(), "kept.bin".to_string()).keep();

        let log = std::fs::read_to_string(batch.directory().join(BATCH_LOG)).unwrap();

        assert!(!dropped.exists());
        assert!(kept.exists());
        assert!(log.contains("upload not completed, partial file deleted: dropped.bin"));
        assert!(!log.contains("kept.bin"));
    }
}
