use crate::app::{AppElement, Message, Taskscape};
use crate::models::Priority;
use crate::thememanager::{ButtonKind, shell_container};
use crate::widgets::{app_input, labeled_button, metric_card, styled_dropdown};
use iced::Alignment;
use iced::Length;
use iced::widget::{column, container, row};

impl Taskscape {
    pub(crate) fn tasks_view(&self) -> AppElement<'_> {
        let visible_tasks = self.filtered_tasks();

        let mut content = column![
            self.header("MULTI LIST PLANNER", "TaskScape"),
            self.composer_row(),
        ]
        .spacing(16)
        .padding([22, 24, 30, 24]);

        if self.show_filters {
            content = content.push(self.filters_panel());
        }

        content = content
            .push(self.metrics_row())
            .push(self.actions_row())
            .push(self.workspace_panel(&visible_tasks));

        container(content)
            .width(Length::Fill)
            .style(shell_container(self.theme_mode))
            .into()
    }

    fn composer_row(&self) -> AppElement<'_> {
        row![
            app_input(
                self.theme_mode,
                "Add a focused task, then press Enter",
                &self.title_input,
                Message::TitleChanged,
                Length::Fill,
                Some(Message::AddTask),
            ),
            styled_dropdown(
                self.theme_mode,
                &Priority::ALL[..],
                Some(self.composer_priority),
                Message::ComposerPriorityChanged,
                Length::Fixed(190.0),
            ),
            app_input(
                self.theme_mode,
                "dd/mm/yyyy",
                &self.due_date_input,
                Message::DueDateChanged,
                Length::Fixed(150.0),
                None,
            ),
            app_input(
                self.theme_mode,
                "tags: launch, inbox",
                &self.tags_input,
                Message::TagsChanged,
                Length::Fixed(150.0),
                None,
            ),
            labeled_button(
                self.theme_mode,
                "☷",
                "Filters",
                ButtonKind::Secondary,
                Some(Message::ToggleFilters),
            ),
            labeled_button(
                self.theme_mode,
                "✦",
                "Add",
                ButtonKind::Primary,
                Some(Message::AddTask),
            ),
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn metrics_row(&self) -> AppElement<'_> {
        row![
            metric_card(self.theme_mode, self.completed_count().to_string(), "Completed"),
            metric_card(self.theme_mode, self.open_count().to_string(), "Open"),
            metric_card(self.theme_mode, self.archived_count().to_string(), "Archived"),
        ]
        .spacing(8)
        .into()
    }

    fn actions_row(&self) -> AppElement<'_> {
        row![
            labeled_button(
                self.theme_mode,
                "✓",
                "Clear completed",
                ButtonKind::Ghost,
                Some(Message::ClearCompleted),
            ),
            labeled_button(
                self.theme_mode,
                "□",
                "Archive completed",
                ButtonKind::Ghost,
                Some(Message::ArchiveCompleted),
            ),
            labeled_button(
                self.theme_mode,
                "⌫",
                "Clear all",
                ButtonKind::Ghost,
                Some(Message::ClearAll),
            ),
        ]
        .spacing(8)
        .into()
    }
}
