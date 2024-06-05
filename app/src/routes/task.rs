use axum::routing::{get, post};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use serde_json::json;

use crate::{
    schemas::task::{CreateTaskSchema, UpdateTaskSchema, TaskSchema},
    state::AppState,
    utils::{errors::AppError, validator::ValidatedJson},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        get_all_tasks,
        get_task,
        create_task,
        delete_task,
        update_task,
    ),
    components(schemas(
        TaskSchema,
        CreateTaskSchema,
        UpdateTaskSchema
    )),
    tags(
        (name = "tasks", description = "Tasks api")
    )
)]
pub(super) struct TaskDoc;

#[utoipa::path(
    get,
    path = "",
    tag = "tasks",
    responses(
        (status = 200, description = "Tasks", body = Vec<TaskSchema>)
    )
)]
pub async fn get_all_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let tasks = state.task_service.find_all_tasks().await;
    Json(json!(tasks))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "tasks",
    responses(
        (status = 200, description = "task found successfully", body = TaskSchema),
        (status = 404, description = "task not found")
    ),
    params(
        ("id" = i32, Path, description = "Task id from database")
    )
)]
pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let task = state.task_service.find_one_task(&id).await?;
    Ok(Json(json!(task)))
}

#[utoipa::path(
    post,
    path = "",
    tag = "tasks",
    responses(
        (status = 201, description = "Task created successfully", body = TaskSchema)
    ),
    request_body = CreateTaskSchema
)]
pub async fn create_task(
    State(state): State<AppState>,
    ValidatedJson(task): ValidatedJson<CreateTaskSchema>,
) -> impl IntoResponse {
    let task = state.task_service.create_task(task).await;
    tracing::info!("Successfully created a task: {:?}", task);
    (StatusCode::CREATED, Json(task))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "tasks",
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
) -> Result<impl IntoResponse, AppError> {
    state.task_service.delete_task(&id).await?;
    Ok(Json(json!({"message": "Task deleted"})))
}

#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "tasks",
    request_body=UpdateTaskSchema,
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
    ValidatedJson(body): ValidatedJson<UpdateTaskSchema>,
) -> Result<impl IntoResponse, AppError> {
    state.task_service.update_task(&id, body).await?;
    Ok(Json(json!({ "message": "Task updated!" })))
}

pub fn init_tasks_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_task).get(get_all_tasks))
        .route("/:id", get(get_task).delete(delete_task).patch(update_task))
}
