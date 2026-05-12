use crate::app::{AppElement, Message, Taskscape};
use crate::models::Priority;
use crate::thememanager::ButtonKind;
use crate::widgets::{app_input, labeled_button, metric_card, styled_dropdown};
use iced::Alignment;
use iced::Length;
use iced::widget::{column, row};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn tasks_view(&self) -> AppElement<'_> {
        let mut content = column![
            self.header(),
            self.composer_row(),
        ]
        .height(Length::Fill)
        .spacing(16);

        content = content
            .push(self.metrics_row())
            .push(self.actions_row())
            .push(self.task_list_panel());

        content.into()
    }

    fn composer_row(&self) -> AppElement<'_> {
        row![
            app_input(
                self.theme_mode,
                "Enter a task title and press Enter",
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
            labeled_button(
                self.theme_mode,
                Some(Icon::CirclePlus),
                "Add",
                ButtonKind::Primary,
                Some(Message::AddTask),
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center)
        .into()
    }

    fn metrics_row(&self) -> AppElement<'_> {
        row![
            metric_card(self.theme_mode, self.total_count().to_string(), "Total"),
            metric_card(self.theme_mode, self.open_count().to_string(), "Open"),
            metric_card(self.theme_mode, self.completed_count().to_string(), "Completed"),
        ]
        .spacing(8)
        .into()
    }

    fn actions_row(&self) -> AppElement<'_> {
        row![
            labeled_button(
                self.theme_mode,
                Some(Icon::CheckCheck),
                "Clear completed",
                ButtonKind::Ghost,
                Some(Message::ClearCompleted),
            ),
            labeled_button(
                self.theme_mode,
                Some(Icon::CircleX),
                "Clear all",
                ButtonKind::Ghost,
                Some(Message::ClearAll),
            ),
        ]
        .spacing(8)
        .into()
    }
}
