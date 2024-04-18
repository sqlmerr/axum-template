use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskSchema {
    pub title: String,
    pub description: Option<String>
}

#[derive(Deserialize)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub description: Option<String>
}