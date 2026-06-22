use crate::app::{AppElement, AttachTarget, Message, Taskscape};
use common::ui::tokens::space;
use common::ui::{
    ButtonKind, Icon, t_attachment_chip, t_button, t_icon_button_ghost, t_input_box,
};
use iced::Alignment;
use iced::Length;
use iced::widget::{column, row};

impl Taskscape {
    pub(crate) fn tasks_view(&self) -> AppElement<'_> {
        column![self.header(), self.composer_row()]
            .height(Length::Fill)
            .spacing(space::LG)
            .push(self.task_list_panel())
            .push(self.actions_row())
            .into()
    }

    fn composer_row(&self) -> AppElement<'_> {
        let input_row = row![
            t_input_box(
                self.theme_mode,
                "Enter a task title and press Enter",
                &self.title_input,
                Message::TitleChanged,
                Length::Fill,
                Some(Message::AddTask),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Attach,
                Some(Message::AttachFile(AttachTarget::Composer)),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Camera,
                Some(Message::AttachScreenshot(AttachTarget::Composer)),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::AddCircle),
                "Add",
                ButtonKind::Primary,
                Some(Message::AddTask),
            ),
        ]
        .spacing(space::MD)
        .align_y(Alignment::Center);

        if self.staged_attachments.is_empty() {
            input_row.into()
        } else {
            let chips = self.staged_attachments.iter().enumerate().fold(
                row![].spacing(space::SM),
                |chips, (index, attachment)| {
                    chips.push(t_attachment_chip(
                        self.theme_mode,
                        attachment,
                        Message::OpenAttachment(attachment.path.clone()),
                        Message::RemoveStagedAttachment(index),
                    ))
                },
            );
            column![input_row, chips].spacing(space::MD).into()
        }
    }

    fn actions_row(&self) -> AppElement<'_> {
        row![
            t_button(
                self.theme_mode,
                Some(Icon::CheckAll),
                "Clear completed",
                ButtonKind::Ghost,
                Some(Message::ClearCompleted),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::Cancel),
                "Clear all",
                ButtonKind::Ghost,
                Some(Message::RequestClearAll),
            ),
        ]
        .spacing(space::MD)
        .into()
    }
}
