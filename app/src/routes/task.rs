use axum::routing::{delete, get, patch, post};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use serde_json::json;
use std::fmt::format;

use crate::{
    db::db_connection,
    models::task::{self, Entity as Task},
    repositories::task::TaskRepository,
    schemas::task::{CreateTaskSchema, UpdateTaskSchema},
    state::AppState,
    utils::errors::{APIError, NotFound},
};

#[utoipa::path(
    get,
    path = "/tasks",
    responses(
        (status = 200, description = "Tasks")
    )
)]
pub async fn get_all_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let tasks = state.task_service.find_all_tasks().await;
    Json(json!(tasks))
}

#[utoipa::path(
    get,
    path = "/tasks/{id}",
    responses(
        (status = 200, description = "task found successfully"),
        (status = 404, description = "task not found")
    ),
    params(
        ("id" = i32, Path, description = "Task id from database")
    )
)]
pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, APIError> {
    let task = state.task_service.find_one_task(&id).await;
    if task.is_none() {
        return Err(APIError {
            message: format!("Task with id {} not found", id),
            status_code: StatusCode::NOT_FOUND,
        });
    }
    Ok(Json(json!(task)))
}

#[utoipa::path(
    post,
    path = "/tasks",
    responses(
        (status = 201, description = "Task created successfully")
    ),
    request_body = CreateTaskSchema
)]
pub async fn create_task(
    State(state): State<AppState>,
    Json(task): Json<CreateTaskSchema>,
) -> impl IntoResponse {
    let task = state.task_service.create_task(task).await;
    tracing::info!("Successfully created a task: {:?}", task);
    (StatusCode::CREATED, Json(task))
}

#[utoipa::path(
    delete,
    path = "/tasks/{id}",
    responses(
        (status = 200, description = "Task deleted successfully")
    ),
    params(
        ("id" = i32, Path, description = "Task id from database")
    )
)]
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, APIError> {
    let response = state
        .task_service
        .delete_task(&id)
        .await;
    match response {
        Ok(_) => Ok(Json(json!({"message": "Task deleted"}))),
        Err(e) => Err(APIError { message: e.message, status_code: StatusCode::NOT_FOUND })
    }
}

#[utoipa::path(
    patch,
    path = "/tasks/{id}",
    responses(
        (status = 200, description = "Task edited successfully"),
        (status = 404, description = "Task not found")
    ),
    params(
        ("id" = i32, Path, description = "Task id from database")
    )
)]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateTaskSchema>,
) -> Result<impl IntoResponse, APIError> {
    let response = state.task_service.update_task(&id, body).await;
    return match response {
        Ok(_) => Ok(Json(json!({ "message": "Task updated!" }))),
        Err(e) => Err(APIError {
            message: e.message,
            status_code: StatusCode::NOT_FOUND,
        }),
    };
}

pub fn init_tasks_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", post(create_task).get(get_all_tasks))
        .route("/:id", get(get_task).delete(delete_task).patch(update_task));

    router
}
