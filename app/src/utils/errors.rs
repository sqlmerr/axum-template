use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct APIError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({ "status_code": self.status_code.as_u16(), "message": self.message })),
        )
            .into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("`{entity}` with id `{id}` not found")]
    EntityNotFound { entity: &'static str, id: i32 },
}
