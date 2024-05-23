use crate::db::db_connection;
use crate::models::task;
use crate::schemas::task::{CreateTaskSchema, UpdateTaskSchema};
use crate::utils::errors::NotFound;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};


#[derive(Clone)]
pub struct TaskRepository {}

impl TaskRepository {
    pub async fn create(&self, data: CreateTaskSchema) -> task::Model {
        let db = db_connection().await.unwrap();
        let task = task::ActiveModel {
            title: Set(data.title),
            description: Set(data.description),
            ..Default::default()
        };
        task.insert(&db).await.unwrap()
    }

    pub async fn find_one(&self, id: &i32) -> Option<task::Model> {
        let db = db_connection().await.unwrap();
        task::Entity::find_by_id(id.to_owned()).one(&db).await.unwrap()
    }

    pub async fn find_all(&self) -> Vec<task::Model> {
        let db = db_connection().await.unwrap();
        task::Entity::find().all(&db).await.unwrap()
    }

    pub async fn delete(&self, id: &i32) {
        let db = db_connection().await.unwrap();
        task::Entity::delete_by_id(id.to_owned()).exec(&db).await.unwrap();
    }

    pub async fn update(&self, id: &i32, data: UpdateTaskSchema) -> Result<(), NotFound> {
        let db = db_connection().await.unwrap();
        let task = self.find_one(id).await;
        if task.is_none() {
            return Err(NotFound {
                message: format!("Task with id {} not found", id),
            });
        }

        let mut task: task::ActiveModel = task.unwrap().into();
        if let Some(title) = data.title {
            task.title = Set(title)
        }
        if let Some(description) = data.description {
            task.description = Set(description);
        }

        task.update(&db).await.unwrap();

        Ok(())
    }
}
