use crate::models::Priority;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub priority: Priority,
    pub due_date: Option<String>,
    pub completed: bool,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
