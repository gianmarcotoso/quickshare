use serde::Serialize;
use utoipa::ToSchema;

use crate::features::uploads::types::batch::Batch;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BatchDto {
    pub id: String,
}

impl From<&Batch> for BatchDto {
    fn from(batch: &Batch) -> Self {
        Self {
            id: batch.id().to_string(),
        }
    }
}
