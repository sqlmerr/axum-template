use axum::{
    response::{IntoResponse, Json},
    extract::{Query, Path},
    http::{StatusCode}
};
use sea_orm::{ActiveModelTrait, Set, EntityTrait};
use serde_json::json;

use crate::{
    db::db_connection,
    models::task::{self, Entity as Task},
    schemas::task::{CreateTaskSchema, UpdateTaskSchema},
    utils::error::APIError
};

pub async fn get_all_tasks() -> impl IntoResponse {
    let db = db_connection().await.unwrap();
    let tasks = Task::find().all(&db).await.unwrap();
    Json(json!(tasks))
}

pub async fn get_task(Path(id): Path<i32>) -> Result<impl IntoResponse, APIError> {
    let db = db_connection().await.unwrap();
    let task = Task::find_by_id(id).one(&db).await.unwrap();
    if task.is_none() {
        return Err(APIError { message: format!("Task with id {} not found", id), status_code: StatusCode::NOT_FOUND })
    }
    Ok(Json(json!(task)))
}

pub async fn create_task(Json(task): Json<CreateTaskSchema>) -> impl IntoResponse {
    let db = db_connection().await.unwrap();
    let task = task::ActiveModel {
        title: Set(task.title),
        description: Set(task.description),
        ..Default::default()
    };
    let task: task::Model = task.insert(&db).await.unwrap();

    (
        StatusCode::CREATED,
        Json(json!({"id": task.id, "title": task.title}))
    )
}

pub async fn delete_task(Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = db_connection().await.unwrap();
    Task::delete_by_id(id).exec(&db).await.unwrap();
    Ok(Json(json!({"message": "Task deleted"})))
}

// pub async fn update_task(
//     Path(id): Path<i32>,
//     Json(body): Json<UpdateTaskSchema>,
// ) -> Result<impl IntoResponse, APIError> {
//     let db = db_connection().await.unwrap();
//     let task: Option<task::Model> = Task::find_by_id(id).one(&db).await.unwrap();
//     if task.is_none() {
//         return Err(APIError { message: format!("Task with id {} not found", id), status_code: StatusCode::NOT_FOUND })
//     }
//
//     let mut task: task::ActiveModel = task.unwrap().into();
//     if let Some(title) = task.title {}
//     task.title = Set(body.title.unwrap().to_owned());
//     task.description = Set(Some(body.description.unwrap().to_owned()));
//
//     let task = task.update(&db).await.unwrap();
//
//     Ok(Json(json!({ "message": "Task updated!" })))
// } TODO: complete
