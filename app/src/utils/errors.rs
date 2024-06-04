use axum::extract::rejection::JsonRejection;
use axum::response::Response;
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Debug, ToSchema)]
pub struct APIError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "status_code": status_code.as_u16(), "message": self.message })),
        )
            .into_response()
    }
}

impl APIError {
    pub fn new(status_code: StatusCode, message: String) -> Self {
        Self {
            message,
            status_code,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{entity} with id {id} not found")]
    EntityNotFound { entity: &'static str, id: i32 },
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = self.to_string();

        let (status_code, message) = match self {
            Self::EntityNotFound { .. } => (StatusCode::NOT_FOUND, message),
            Self::ValidationError(_) => (
                StatusCode::BAD_REQUEST,
                format!("Validation errors: [{}", self).replace('\n', ","),
            ),
            _ => (StatusCode::BAD_REQUEST, message),
        };

        APIError::new(status_code, message).into_response()
    }
}
