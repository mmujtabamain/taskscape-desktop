pub mod header;
pub mod tasks;
pub mod workspace;

use crate::app::{AppElement, Taskscape};
use crate::thememanager::{panel_alt_container, shell_container, tokens};
use iced::Length;
use iced::widget::{column, container, row};

impl Taskscape {
    pub(crate) fn view_root(&self) -> AppElement<'_> {
        container(
            column![self.tasks_view(), self.status_bar()]
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(14)
                .padding(14),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(shell_container(self.theme_mode))
        .into()
    }

    fn status_bar(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        container(
            row![
                crate::widgets::t_body(&self.status_message, 14.0, palette.text_secondary),
                iced::widget::Space::new().width(Length::Fill),
                crate::widgets::t_caption(self.theme_mode.label(), 12.0, palette.text_muted),
            ]
            .spacing(12),
        )
        .padding([10, 14])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }
}
