use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TaskSchema {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateTaskSchema {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub description: Option<String>,
}
