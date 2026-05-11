use crate::models::Task;
use crate::utils::date::{THIS_WEEK_END, TODAY};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickDate {
    None,
    Today,
    Overdue,
    ThisWeek,
}

impl QuickDate {
    pub const ALL: [Self; 4] = [Self::None, Self::Today, Self::Overdue, Self::ThisWeek];

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Today => "Today",
            Self::Overdue => "Overdue",
            Self::ThisWeek => "This week",
        }
    }

    pub fn matches(self, task: &Task) -> bool {
        match self {
            Self::None => true,
            Self::Today => matches!(task.due_date.as_deref(), Some(TODAY)),
            Self::Overdue => task
                .due_date
                .as_deref()
                .map(|date| date < TODAY)
                .unwrap_or(false),
            Self::ThisWeek => task
                .due_date
                .as_deref()
                .map(|date| (TODAY..=THIS_WEEK_END).contains(&date))
                .unwrap_or(false),
        }
    }
}
