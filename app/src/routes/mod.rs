mod task;

use axum::{
    response::Json,
    routing::{delete, get, patch, post},
    Router,
};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{schemas, utils};

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
            schemas::task::CreateTaskSchema,
            schemas::task::UpdateTaskSchema,
            utils::errors::APIError
        ))
    )]
    struct ApiDoc;

    let root_router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .nest("/tasks", task::init_tasks_router());

    return root_router;
}
