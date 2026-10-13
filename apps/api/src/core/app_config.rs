use std::path::{Path, PathBuf};

use config::Config;

pub const DEFAULT_LOG_LEVEL: &str = "info";
pub const API_PATH: &str = "/api";

#[derive(Clone)]
pub struct AppConfig {
    pub bind: String,
    pub port: u16,

    pub data_path: PathBuf,

    pub max_upload_bytes: Option<u64>,

    pub log_level: String,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let settings = Config::builder()
            .add_source(
                config::File::from(path)
                    .format(config::FileFormat::Ini)
                    .required(false),
            )
            .build()?;

        let bind = settings
            .get_string("Server.Bind")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = settings.get_int("Server.Port").unwrap_or(3030) as u16;

        let data_path = settings
            .get_string("Storage.DataPath")
            .unwrap_or_else(|_| "./data".to_string());

        let max_upload_bytes = settings
            .get_int("Upload.MaxBytes")
            .ok()
            .filter(|bytes| *bytes > 0)
            .map(|bytes| bytes as u64);

        let log_level = settings
            .get_string("Log.Level")
            .unwrap_or_else(|_| DEFAULT_LOG_LEVEL.to_string());

        Ok(Self {
            bind,
            port,

            data_path: PathBuf::from(data_path),

            max_upload_bytes,

            log_level,
        })
    }

    pub fn template() -> String {
        include_str!("../../config.ini.template").to_string()
    }
}
