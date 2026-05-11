use crate::app::Taskscape;
use crate::models::{SortMode, Task};
use std::cmp::Reverse;

impl Taskscape {
    pub(crate) fn filtered_tasks(&self) -> Vec<&Task> {
        let search = self.filter_search.to_lowercase();
        let tag_filter = self.filter_tag.to_lowercase();

        let mut tasks = self
            .tasks
            .iter()
            .filter(|task| self.completion_filter.matches(task))
            .filter(|task| self.priority_filter.matches(task.priority))
            .filter(|task| self.status_filter.matches(task.status))
            .filter(|task| self.quick_date.matches(task))
            .filter(|task| task.matches_date_range(&self.filter_from, &self.filter_to))
            .filter(|task| {
                search.is_empty()
                    || task.title.to_lowercase().contains(&search)
                    || task
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&search))
            })
            .filter(|task| {
                tag_filter.is_empty()
                    || task
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&tag_filter))
            })
            .collect::<Vec<_>>();

        match self.sort_mode {
            SortMode::Manual => {}
            SortMode::Newest => tasks.reverse(),
            SortMode::Oldest => {}
            SortMode::DueDate => {
                tasks.sort_by_key(|task| task.due_date.as_deref().unwrap_or("99/99/9999"));
            }
            SortMode::Priority => tasks.sort_by_key(|task| Reverse(task.priority.rank())),
            SortMode::Alphabetical => tasks.sort_by_key(|task| task.title.to_lowercase()),
        }

        tasks
    }

    pub(crate) fn visible_count(&self) -> usize {
        self.filtered_tasks().len()
    }

    pub(crate) fn open_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| !task.archived && !task.is_completed())
            .count()
    }

    pub(crate) fn archived_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.archived).count()
    }

    pub(crate) fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.is_completed()).count()
    }

    pub(crate) fn active_todos_count(&self) -> usize {
        self.tasks.iter().filter(|task| !task.archived).count()
    }
}
