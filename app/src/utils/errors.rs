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

        match self {
            Self::EntityNotFound { .. } => {
                APIError::new(StatusCode::NOT_FOUND, message).into_response()
            }
            Self::ValidationError(_) => APIError::new(
                StatusCode::BAD_REQUEST,
                format!("Validation errors: [{}", self).replace('\n', ","),
            )
            .into_response(),
            _ => (StatusCode::BAD_REQUEST, self.into_response()).into_response(),
        }
    }
}
