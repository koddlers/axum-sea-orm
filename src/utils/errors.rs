use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: Option<u8>,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        let response = (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({
                "Status Code": self.status_code.as_u16(),
                "Error Code": self.error_code,
                "Message": self.message,
            })),
        );

        response.into_response()
    }
}
