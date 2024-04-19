mod task;

use axum::{
    response::{IntoResponse, Json},
    routing::{delete, get, patch, post},
    Router,
};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::schemas;

pub fn init_routers() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            task::get_all_tasks,
            task::get_task,
            task::create_task,
            task::delete_task,
        ),
        components(schemas(schemas::task::CreateTaskSchema, schemas::task::UpdateTaskSchema))
    )]
    struct ApiDoc;

    let root_router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .route("/task", post(task::create_task))
        .route("/tasks", get(task::get_all_tasks))
        .route("/task/:id", get(task::get_task))
        .route("/task/:id", delete(task::delete_task));
    // .route("/task/:id", patch(task::update_task));

    return root_router;
}
