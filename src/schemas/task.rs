use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateTaskSchema {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub description: Option<String>,
}
