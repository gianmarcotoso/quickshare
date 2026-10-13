use serde::Serialize;
use utoipa::ToSchema;

macro_rules! error_codes {
    ($($variant:ident => $code:tt),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
        pub enum ErrorCode {
            $(#[serde(rename = $code)] $variant),*
        }

        impl ErrorCode {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(ErrorCode::$variant => $code),*
                }
            }
        }
    };
}

error_codes! {
    Internal => "ERR_INTERNAL",
    InvalidToken => "ERR_INVALID_TOKEN",
    BatchNotFound => "ERR_BATCH_NOT_FOUND",
    FileMissing => "ERR_FILE_MISSING",
    FileTooLarge => "ERR_FILE_TOO_LARGE",
    UploadInterrupted => "ERR_UPLOAD_INTERRUPTED",
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ErrorDto {
    pub code: ErrorCode,
}

impl ErrorDto {
    pub fn new(code: ErrorCode) -> Self {
        Self { code }
    }
}
