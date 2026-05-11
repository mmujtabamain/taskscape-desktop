use crate::app::{Message, Taskscape};

impl Taskscape {
    pub(crate) fn handle_message(&mut self, message: Message) {
        match message {
            Message::SetNav(nav) => self.nav = nav,
            Message::SetThemeMode(mode) => self.theme_mode = mode,
            Message::ToggleTheme => self.theme_mode = self.theme_mode.toggled(),
            Message::ToggleFilters => self.show_filters = !self.show_filters,
            Message::TitleChanged(value) => self.title_input = value,
            Message::DueDateChanged(value) => self.due_date_input = value,
            Message::TagsChanged(value) => self.tags_input = value,
            Message::FilterSearchChanged(value) => self.filter_search = value,
            Message::FilterTagChanged(value) => self.filter_tag = value,
            Message::FilterFromChanged(value) => self.filter_from = value,
            Message::FilterToChanged(value) => self.filter_to = value,
            Message::ComposerPriorityChanged(value) => self.composer_priority = value,
            Message::CompletionFilterChanged(value) => self.completion_filter = value,
            Message::PriorityFilterChanged(value) => self.priority_filter = value,
            Message::StatusFilterChanged(value) => self.status_filter = value,
            Message::SortModeChanged(value) => self.sort_mode = value,
            Message::QuickDateChanged(value) => self.quick_date = value,
            Message::AddTask => self.add_task(),
            Message::ClearCompleted => self.tasks.retain(|task| !task.is_completed()),
            Message::ArchiveCompleted => {
                for task in &mut self.tasks {
                    if task.is_completed() {
                        task.archived = true;
                    }
                }
            }
            Message::ClearAll => self.tasks.clear(),
            Message::ClearFilters => self.reset_filters(),
            Message::SaveFilters => {}
        }
    }
}
