use crate::app::{AppElement, Message};
use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use crate::widgets::icon_badge;
use iced::Alignment;
use iced::Length;
use iced::widget::{button, column, row, text};

pub fn sidebar_button<'a>(
    theme_mode: ThemeMode,
    title: &'a str,
    subtitle: &'a str,
    icon: &'a str,
    active: bool,
    message: Message,
) -> AppElement<'a> {
    let palette = tokens(theme_mode);

    button(
        row![
            icon_badge(theme_mode, icon, active),
            column![
                text(title).size(18).style(palette.text_primary),
                text(subtitle).size(13).style(palette.text_secondary),
            ]
            .spacing(2),
        ]
        .spacing(12)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .padding(10)
    .style(button_style(theme_mode, ButtonKind::Sidebar(active)))
    .on_press(message)
    .into()
}
