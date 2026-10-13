use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use tokio::fs::{File, OpenOptions};

const MAX_ATTEMPTS: u32 = 10_000;

pub async fn create_unique_file(
    directory: &Path,
    name: &str,
) -> std::io::Result<(String, PathBuf, File)> {
    for attempt in 0..MAX_ATTEMPTS {
        let candidate = numbered_name(name, attempt);
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

fn numbered_name(name: &str, attempt: u32) -> String {
    if attempt == 0 {
        return name.to_string();
    }

    match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => format!("{stem} ({attempt}).{extension}"),
        _ => format!("{name} ({attempt})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates_are_numbered_before_the_extension() {
        assert_eq!(numbered_name("report.pdf", 0), "report.pdf");
        assert_eq!(numbered_name("report.pdf", 2), "report (2).pdf");
        assert_eq!(numbered_name("archive.tar.gz", 1), "archive.tar (1).gz");
        assert_eq!(numbered_name(".env", 1), ".env (1)");
        assert_eq!(numbered_name("README", 1), "README (1)");
    }
}
