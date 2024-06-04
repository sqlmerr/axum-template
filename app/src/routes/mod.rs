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
use crate::{repositories, schemas, services, utils, Config};

pub async fn init_routers(settings: &Config) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            task::get_all_tasks,
            task::get_task,
            task::create_task,
            task::delete_task,
            task::update_task,
        ),
        components(schemas(
            schemas::task::TaskSchema,
            schemas::task::CreateTaskSchema,
            schemas::task::UpdateTaskSchema,
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
