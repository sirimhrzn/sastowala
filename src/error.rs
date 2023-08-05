use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]

pub struct ErrorResponse {
    pub message: String,
}

#[derive(Deserialize)]

pub enum CustomError {
    JsonParseFail,
    FileCreateFailed,
    FileReadFailed,
    CustomMessage(ErrorResponse),
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (message, status_code) = match self {
            Self::JsonParseFail => (
                "Failed Parsing Json".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::FileCreateFailed => (
                "Failed to create file".to_string(),
                StatusCode::EXPECTATION_FAILED,
            ),
            Self::FileReadFailed=>(
                "Failed to read file".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR
            ),
            Self::CustomMessage(e) => (e.message, StatusCode::EXPECTATION_FAILED),
        };
        (status_code, Json(json!({"error": message}))).into_response()
    }
}
