use std::{fs::OpenOptions, io::Write, path::Path};

use crate::core::{
    app_config::{AppConfig, DEFAULT_LOG_LEVEL},
    logging::init_logging,
};

pub fn init(config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    init_logging(DEFAULT_LOG_LEVEL);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_path)
        .map_err(|e| format!("Cannot create {}: {e}", config_path.display()))?;

    file.write_all(AppConfig::template().as_bytes())?;

    tracing::info!(path = %config_path.display(), "config written");

    Ok(())
}
