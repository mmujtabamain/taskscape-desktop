use crate::app::Taskscape;
use crate::models::{Task, TaskStatus};
use crate::utils::tags::parse_tags;

impl Taskscape {
    pub(crate) fn add_task(&mut self) {
        let title = self.title_input.trim();

        if title.is_empty() {
            return;
        }

        self.tasks.push(Task {
            title: title.to_owned(),
            priority: self.composer_priority,
            status: TaskStatus::Todo,
            due_date: (!self.due_date_input.trim().is_empty())
                .then(|| self.due_date_input.trim().to_owned()),
            tags: parse_tags(&self.tags_input),
            archived: false,
        });

        self.title_input.clear();
        self.due_date_input.clear();
    }

    pub(crate) fn reset_filters(&mut self) {
        self.filter_search.clear();
        self.filter_tag.clear();
        self.filter_from.clear();
        self.filter_to.clear();
        self.completion_filter = crate::models::CompletionFilter::Active;
        self.priority_filter = crate::models::PriorityFilter::All;
        self.status_filter = crate::models::StatusFilter::All;
        self.sort_mode = crate::models::SortMode::Manual;
        self.quick_date = crate::models::QuickDate::None;
    }
}
