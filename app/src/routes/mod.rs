mod task;

use axum::{response::Json, routing::get, Router};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;
use crate::{repositories, schemas, services, utils};

pub fn init_routers() -> Router {
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

    let task_repository = repositories::task::TaskRepository {};
    let task_service = services::task::TaskService {
        repository: task_repository,
    };
    let state = AppState { task_service };

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .nest("/tasks", task::init_tasks_router())
        .with_state(state)
}
