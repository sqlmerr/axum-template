use crate::repositories::task::{CreateTaskDTO, TaskRepository, UpdateTaskDTO};
use crate::schemas::task::{CreateTaskSchema, TaskSchema, UpdateTaskSchema};
use crate::utils::errors::NotFound;

#[derive(Clone)]
pub struct TaskService {
    pub repository: TaskRepository,
}

impl TaskService {
    pub async fn create_task(&self, data: CreateTaskSchema) -> TaskSchema {
        let response = self
            .repository
            .create(CreateTaskDTO {
                title: data.title,
                description: data.description,
            })
            .await;

        TaskSchema {
            id: response.id,
            title: response.title,
            description: response.description,
        }
    }

    pub async fn find_one_task(&self, id: &i32) -> Option<TaskSchema> {
        let response = self.repository.find_one(id).await;
        match response {
            None => None,
            Some(task) => Some(TaskSchema {
                id: task.id,
                title: task.title,
                description: task.description,
            }),
        }
    }

    pub async fn find_all_tasks(&self) -> Vec<TaskSchema> {
        let response = self.repository.find_all().await;
        let tasks: Vec<TaskSchema> = response
            .iter()
            .map(|t| TaskSchema {
                id: t.id,
                title: t.to_owned().title,
                description: t.to_owned().description,
            })
            .collect();
        tasks
    }

    pub async fn delete_task(&self, id: &i32) -> Result<(), NotFound> {
        let task = self.repository.find_one(id).await;
        if task.is_none() {
            return Err(NotFound {
                message: format!("Task with id {id} not found").to_string(),
            });
        }

        self.repository.delete(id).await;
        Ok(())
    }

    pub async fn update_task(&self, id: &i32, data: UpdateTaskSchema) -> Result<(), NotFound> {
        let dto = UpdateTaskDTO {
            title: data.title,
            description: data.description,
        };
        self.repository.update(id, dto).await
    }
}
