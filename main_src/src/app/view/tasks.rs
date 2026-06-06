use crate::app::{AppElement, Message, Taskscape};
use common::thememanager::ButtonKind;
use common::widgets::{t_button, t_input_box};
use iced::Alignment;
use iced::Length;
use iced::widget::{column, row};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn tasks_view(&self) -> AppElement<'_> {
        let mut content = column![self.header(), self.composer_row(),]
            .height(Length::Fill)
            .spacing(10);

        content = content
            .push(self.task_list_panel())
            .push(self.actions_row());

        content.into()
    }

    fn composer_row(&self) -> AppElement<'_> {
        row![
            t_input_box(
                self.theme_mode,
                "Enter a task title and press Enter",
                &self.title_input,
                Message::TitleChanged,
                Length::Fill,
                Some(Message::AddTask),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::CirclePlus),
                "Add",
                ButtonKind::Primary,
                Some(Message::AddTask),
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    }

    fn actions_row(&self) -> AppElement<'_> {
        row![
            t_button(
                self.theme_mode,
                Some(Icon::CheckCheck),
                "Clear completed",
                ButtonKind::Ghost,
                Some(Message::ClearCompleted),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::CircleX),
                "Clear all",
                ButtonKind::Ghost,
                Some(Message::RequestClearAll),
            ),
        ]
        .spacing(8)
        .into()
    }
}
