use utoipa::OpenApi;

use crate::openapi::ApiDoc;

pub fn openapi() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", ApiDoc::openapi().to_pretty_json()?);

    Ok(())
}
