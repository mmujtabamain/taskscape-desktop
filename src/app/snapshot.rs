use crate::models::Task;

#[derive(Debug, Clone)]
pub struct AppSnapshot {
    pub tasks: Vec<Task>,
}
