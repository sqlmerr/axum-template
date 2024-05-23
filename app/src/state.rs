use crate::repositories::task::TaskRepository;

#[derive(Clone)]
pub struct AppState {
    pub task_repository: TaskRepository,
}
