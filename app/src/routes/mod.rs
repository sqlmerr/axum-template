mod task;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response::Json, routing::get, Router};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::db::db_connection;
use crate::state::AppState;
use crate::utils::errors::APIError;
use crate::{repositories, services, utils, Config};

use task::TaskDoc;

pub async fn init_routers(settings: &Config) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        nest(
            (path = "/tasks", api = TaskDoc)
        ),
        components(schemas(
            utils::errors::APIError
        ))
    )]
    struct ApiDoc;

    let db_connection = db_connection(settings).await.unwrap();

    let task_repository = repositories::task::TaskRepository { db_connection };
    let task_service = services::task::TaskService {
        repository: task_repository,
    };
    let state = AppState { task_service };

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .nest("/tasks", task::init_tasks_router())
        .fallback(handler_404)
        .with_state(state)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        APIError::new(StatusCode::NOT_FOUND, "Not found".to_string()),
    )
}
