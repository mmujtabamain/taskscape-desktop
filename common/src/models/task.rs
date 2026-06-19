use crate::models::Attachment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub completed: bool,
    /// Files attached to the task. `#[serde(default)]` keeps lists saved before
    /// attachments existed loadable.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
            attachments: Vec::new(),
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
