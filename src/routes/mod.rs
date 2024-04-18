mod task;

use axum::{
    Router,
    routing::{
        get, post, delete, patch
    },
    response::{IntoResponse, Json}
};
use serde_json::json;


pub fn init_routers() -> Router {
    let root_router = Router::new()
        .route("/", get(|| async { Json(json!({"message": "Hello world"})) }))
        .route("/task", post(task::create_task))
        .route("/tasks", get(task::get_all_tasks))
        .route("/task/:id", get(task::get_task))
        .route("/task/:id", delete(task::delete_task));
        // .route("/task/:id", patch(task::update_task));


    return root_router
}