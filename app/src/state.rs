use crate::services::task::TaskService;

#[derive(Clone)]
pub struct AppState {
    pub task_service: TaskService,
}
