use crate::models::TaskStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusFilter {
    All,
    Todo,
    Doing,
    Done,
    Blocked,
}

impl StatusFilter {
    pub const ALL: [Self; 5] = [Self::All, Self::Todo, Self::Doing, Self::Done, Self::Blocked];

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Todo => "Todo",
            Self::Doing => "Doing",
            Self::Done => "Done",
            Self::Blocked => "Blocked",
        }
    }

    pub fn matches(self, status: TaskStatus) -> bool {
        match self {
            Self::All => true,
            Self::Todo => status == TaskStatus::Todo,
            Self::Doing => status == TaskStatus::Doing,
            Self::Done => status == TaskStatus::Done,
            Self::Blocked => status == TaskStatus::Blocked,
        }
    }
}
