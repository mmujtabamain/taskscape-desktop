use crate::models::{Priority, TaskStatus};

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub due_date: Option<String>,
    pub tags: Vec<String>,
    pub archived: bool,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Done)
    }

    pub fn matches_date_range(&self, from: &str, to: &str) -> bool {
        match self.due_date.as_deref() {
            Some(date) => {
                let from_match = from.trim().is_empty() || date >= from.trim();
                let to_match = to.trim().is_empty() || date <= to.trim();
                from_match && to_match
            }
            None => from.trim().is_empty() && to.trim().is_empty(),
        }
    }
}
