//! The standalone "Quit Taskscape?" confirmation popover (its own window).

use crate::app::{AppElement, Message, TrayApp};
use common::ui::tokens::{space, text};
use common::ui::{ButtonKind, Icon, mini_shell, palette, t_button, t_caption, t_heading};
use iced::widget::{Space, column, container, mouse_area, row};
use iced::{Alignment, Length};

impl TrayApp {
    pub(crate) fn quit_confirm_view(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        let card = column![
            t_heading("Quit Taskscape?", text::TITLE, p.text),
            t_caption(
                "The menu-bar icon and mini window will close.",
                text::LABEL,
                p.text_dim,
            ),
            Space::new().height(Length::Fill),
            row![
                Space::new().width(Length::Fill),
                t_button(self.theme_mode, None, "Cancel", ButtonKind::Ghost, Some(Message::CancelQuit)),
                t_button(
                    self.theme_mode,
                    Some(Icon::Power),
                    "Quit",
                    ButtonKind::Primary,
                    Some(Message::ConfirmQuit),
                ),
            ]
            .spacing(space::MD)
            .align_y(Alignment::Center),
        ]
        .spacing(space::MD);

        let panel = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(space::XL)
            .style(mini_shell(self.theme_mode));

        mouse_area(panel).on_press(Message::DragConfirm).into()
    }
}
