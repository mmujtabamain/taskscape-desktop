use crate::models::{Task, TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionFilter {
    Active,
    Completed,
    Pending,
    Archived,
    All,
}

impl CompletionFilter {
    pub const ALL: [Self; 5] = [
        Self::Active,
        Self::Completed,
        Self::Pending,
        Self::Archived,
        Self::All,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Completed => "Completed",
            Self::Pending => "Pending",
            Self::Archived => "Archived",
            Self::All => "All",
        }
    }

    pub fn matches(self, task: &Task) -> bool {
        match self {
            Self::Active => !task.archived && !task.is_completed(),
            Self::Completed => task.is_completed(),
            Self::Pending => {
                !task.archived && matches!(task.status, TaskStatus::Todo | TaskStatus::Doing)
            }
            Self::Archived => task.archived,
            Self::All => true,
        }
    }
}
