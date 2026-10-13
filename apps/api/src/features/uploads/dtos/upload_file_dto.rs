use utoipa::ToSchema;

#[derive(ToSchema)]
#[allow(dead_code)]
pub struct UploadFileDto {
    #[schema(value_type = String, format = Binary)]
    pub file: Vec<u8>,
}
