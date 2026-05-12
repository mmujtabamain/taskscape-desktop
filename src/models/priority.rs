use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

impl Priority {
    pub const ALL: [Self; 4] = [Self::Low, Self::Medium, Self::High, Self::Critical];

    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "Low priority",
            Self::Medium => "Medium priority",
            Self::High => "High priority",
            Self::Critical => "Critical priority",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}
