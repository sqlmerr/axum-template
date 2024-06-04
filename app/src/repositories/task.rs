use super::Repository;
use crate::models::task;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};

pub struct CreateTaskDTO {
    pub title: String,
    pub description: String,
}

pub struct UpdateTaskDTO {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct TaskRepository {
    pub db_connection: DbConn,
}

#[async_trait::async_trait]
impl Repository for TaskRepository {
    type Model = task::Model;
    type Id = i32;
    type CreateDTO = CreateTaskDTO;
    type UpdateDTO = UpdateTaskDTO;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model {
        let db = &self.db_connection;
        let task = task::ActiveModel {
            title: Set(data.title),
            description: Set(data.description),
            ..Default::default()
        };
        task.insert(db).await.unwrap()
    }

    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model> {
        let db = &self.db_connection;
        task::Entity::find_by_id(id.to_owned())
            .one(db)
            .await
            .unwrap()
    }

    async fn find_all(&self) -> Vec<Self::Model> {
        let db = &self.db_connection;
        task::Entity::find().all(db).await.unwrap()
    }

    async fn delete(&self, id: &Self::Id) {
        let db = &self.db_connection;
        task::Entity::delete_by_id(id.to_owned())
            .exec(db)
            .await
            .unwrap();
    }

    async fn update(&self, id: &Self::Id, data: Self::UpdateDTO) {
        let db = &self.db_connection;
        let task = self.find_one(id).await;

        let mut task: task::ActiveModel = task.unwrap().into();
        if let Some(title) = data.title {
            task.title = Set(title)
        }
        if let Some(description) = data.description {
            task.description = Set(description);
        }

        task.update(db).await.unwrap();
    }
}
