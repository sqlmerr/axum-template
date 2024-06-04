use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TaskSchema {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateTaskSchema {
    #[validate(length(min = 4, message = "Title must be at least 4 characters long"))]
    pub title: String,
    #[validate(length(max = 256, message = "Description must be less than 256 characters"))]
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateTaskSchema {
    #[validate(length(min = 4, message = "Title must be at least 4 characters long"))]
    pub title: Option<String>,
    #[validate(length(max = 256, message = "Description must be less than 256 characters"))]
    pub description: Option<String>,
}
