use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::{
    core::error_code::{ErrorCode, ErrorDto},
    features::uploads::controllers::{
        batches as batches_controller, session as session_controller,
    },
};

pub const OPENAPI_PATH: &str = "/openapi.json";

#[derive(OpenApi)]
#[openapi(
    info(title = "QuickShare API"),
    paths(
        session_controller::check,
        batches_controller::create,
        batches_controller::upload,
        batches_controller::close,
    ),
    components(schemas(ErrorDto, ErrorCode)),
    modifiers(&SessionToken),
)]
pub struct ApiDoc;

struct SessionToken;

impl Modify for SessionToken {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_default()
            .add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            );
    }
}
