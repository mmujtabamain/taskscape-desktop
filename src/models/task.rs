use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
