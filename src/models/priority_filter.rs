use crate::models::Priority;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityFilter {
    All,
    Low,
    Medium,
    High,
    Critical,
}

impl PriorityFilter {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::Low,
        Self::Medium,
        Self::High,
        Self::Critical,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }

    pub fn matches(self, priority: Priority) -> bool {
        match self {
            Self::All => true,
            Self::Low => priority == Priority::Low,
            Self::Medium => priority == Priority::Medium,
            Self::High => priority == Priority::High,
            Self::Critical => priority == Priority::Critical,
        }
    }
}
