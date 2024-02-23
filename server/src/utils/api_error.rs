use axum::{http::{StatusCode, header}, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub message: String,
    pub error_code: Option<i8>
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{{status_code: {}, message: {}}}", 
            self.status_code, self.message
        )
    }
}

impl IntoResponse for ApiError{

    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE,"application/json")], Json(json!({ "StatusCode": self.status_code.as_u16(),"ErrorCode": self.error_code,"Message": self.message })) ).into_response()
    }

}

impl From<StatusCode> for ApiError {
    fn from (status_code: StatusCode) -> Self {
        ApiError {
            status_code,
            message: status_code.canonical_reason().unwrap_or("").to_string(),
            error_code: None
        }
    }
}