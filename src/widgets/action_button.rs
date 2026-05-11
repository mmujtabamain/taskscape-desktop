use crate::app::{AppElement, Message};
use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use iced::Alignment;
use iced::widget::{button, row, text};

pub fn labeled_button<'a>(
    theme_mode: ThemeMode,
    icon: &'a str,
    label: &'a str,
    kind: ButtonKind,
    message: Option<Message>,
) -> AppElement<'a> {
    let palette = tokens(theme_mode);

    button(
        row![
            text(icon).size(15).style(palette.text_primary),
            text(label).size(16).style(palette.text_primary),
        ]
        .spacing(8)
        .align_items(Alignment::Center),
    )
    .padding([10, 14])
    .style(button_style(theme_mode, kind))
    .on_press_maybe(message)
    .into()
}
