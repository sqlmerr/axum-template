use axum::routing::{delete, get, patch, post};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde_json::json;

use crate::{
    db::db_connection,
    models::task::{self, Entity as Task},
    schemas::task::{CreateTaskSchema, UpdateTaskSchema},
    utils::errors::APIError,
};

#[utoipa::path(
    get,
    path = "/tasks",
    responses(
        (status = 200, description = "Tasks")
    )
)]
pub async fn get_all_tasks() -> impl IntoResponse {
    let db = db_connection().await.unwrap();
    let tasks = Task::find().all(&db).await.unwrap();
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
pub async fn get_task(Path(id): Path<i32>) -> Result<impl IntoResponse, APIError> {
    let db = db_connection().await.unwrap();
    let task = Task::find_by_id(id).one(&db).await.unwrap();
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
pub async fn create_task(Json(task): Json<CreateTaskSchema>) -> impl IntoResponse {
    let db = db_connection().await.unwrap();
    let task = task::ActiveModel {
        title: Set(task.title),
        description: Set(task.description),
        ..Default::default()
    };
    let task: task::Model = task.insert(&db).await.unwrap();
    tracing::info!("Successfully created a task: {:?}", task);
    (
        StatusCode::CREATED,
        Json(json!({"id": task.id, "title": task.title})),
    )
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
pub async fn delete_task(Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = db_connection().await.unwrap();
    Task::delete_by_id(id).exec(&db).await.unwrap();
    Ok(Json(json!({"message": "Task deleted"})))
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
    Path(id): Path<i32>,
    Json(body): Json<UpdateTaskSchema>,
) -> Result<impl IntoResponse, APIError> {
    let db = db_connection().await.unwrap();
    let task: Option<task::Model> = Task::find_by_id(id).one(&db).await.unwrap();
    if task.is_none() {
        return Err(APIError {
            message: format!("Task with id {} not found", id),
            status_code: StatusCode::NOT_FOUND,
        });
    }

    let mut task: task::ActiveModel = task.unwrap().into();
    if let Some(title) = body.title {
        task.title = Set(title)
    }
    if let Some(description) = body.description {
        task.description = Set(description);
    }

    task.update(&db).await.unwrap();
    Ok(Json(json!({ "message": "Task updated!" })))
}

pub fn init_tasks_router() -> Router {
    let router = Router::new()
        .route("/", post(create_task).get(get_all_tasks))
        .route("/:id", get(get_task).delete(delete_task).patch(update_task));

    return router;
}
