use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tokio::fs::{File, OpenOptions};

use crate::libs::numbered_file_name::numbered_file_name;

const MAX_ATTEMPTS: u32 = 10_000;

pub async fn create_unique_file(
    directory: &Path,
    name: &str,
) -> std::io::Result<(String, PathBuf, File)> {
    for attempt in 0..MAX_ATTEMPTS {
        let candidate = numbered_file_name(name, attempt);
        let path = directory.join(&candidate);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .await
        {
            Ok(file) => return Ok((candidate, path, file)),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }

    Err(std::io::Error::new(
        ErrorKind::AlreadyExists,
        format!("too many files named {name}"),
    ))
}
