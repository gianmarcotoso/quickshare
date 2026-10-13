use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

use chrono::{DateTime, Utc};

use crate::libs::sha256_file::sha256_file;

pub const BATCH_FOLDER_FORMAT: &str = "%Y_%m_%d_%H_%M_%S";
pub const BATCH_LOG: &str = "quickshare.log";

pub struct Batch {
    id: String,
    directory: PathBuf,
    log: Mutex<BatchLog>,
}

struct BatchLog {
    file: File,
    files: u64,
    bytes: u64,
    closed: bool,
}

pub struct BatchSummary {
    pub files: u64,
    pub bytes: u64,
    pub log_sha256: String,
}

impl Batch {
    pub fn create(data_path: &Path, started_at: DateTime<Utc>) -> std::io::Result<Self> {
        std::fs::create_dir_all(data_path)?;

        let base = started_at.format(BATCH_FOLDER_FORMAT).to_string();
        let mut attempt = 1;

        let (id, directory) = loop {
            let id = if attempt == 1 {
                base.clone()
            } else {
                format!("{base}_{attempt}")
            };
            let directory = data_path.join(&id);

            match std::fs::create_dir(&directory) {
                Ok(()) => break (id, directory),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => attempt += 1,
                Err(error) => return Err(error),
            }
        };

        let file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(directory.join(BATCH_LOG))?;

        Ok(Self {
            id,
            directory,
            log: Mutex::new(BatchLog {
                file,
                files: 0,
                bytes: 0,
                closed: false,
            }),
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    pub fn log_path(&self) -> PathBuf {
        self.directory.join(BATCH_LOG)
    }

    pub fn append(&self, entry: &str) -> std::io::Result<bool> {
        let mut log = self.lock();

        if log.closed {
            return Ok(false);
        }

        log.write(entry)?;

        Ok(true)
    }

    pub fn record_file(&self, entry: &str, size: u64) -> std::io::Result<bool> {
        let mut log = self.lock();

        if log.closed {
            return Ok(false);
        }

        log.write(entry)?;
        log.files += 1;
        log.bytes += size;

        Ok(true)
    }

    pub fn close(
        &self,
        footer: impl FnOnce(u64, u64) -> String,
    ) -> std::io::Result<Option<BatchSummary>> {
        let mut log = self.lock();

        if log.closed {
            return Ok(None);
        }

        log.closed = true;
        let footer = footer(log.files, log.bytes);
        log.write(&footer)?;
        log.file.sync_all()?;

        let log_sha256 = sha256_file(&self.log_path())?;
        log.write(&format!("log sha256 (all previous lines): {log_sha256}\n"))?;
        log.file.sync_all()?;

        Ok(Some(BatchSummary {
            files: log.files,
            bytes: log.bytes,
            log_sha256,
        }))
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, BatchLog> {
        self.log
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl BatchLog {
    fn write(&mut self, entry: &str) -> std::io::Result<()> {
        self.file.write_all(entry.as_bytes())?;
        self.file.flush()
    }
}
