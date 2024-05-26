mod task;

use axum::{response::Json, routing::get, Router};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::db::db_connection;
use crate::state::AppState;
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
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .nest("/tasks", task::init_tasks_router())
        .with_state(state)
}
