use serde::Serialize;
use utoipa::ToSchema;

use crate::features::uploads::types::received_file::ReceivedFile;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedFileDto {
    pub name: String,
    pub size: u64,
    pub sha256: String,
}

impl From<ReceivedFile> for ReceivedFileDto {
    fn from(file: ReceivedFile) -> Self {
        Self {
            name: file.name,
            size: file.size,
            sha256: file.sha256,
        }
    }
}
