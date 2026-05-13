use crate::app::{AppElement, Message};
use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use crate::widgets::{t_body, lucide_icon};
use iced::Alignment;
use iced::widget::{button, row};
use lucide_icons::Icon;

pub fn t_button<'a>(
    theme_mode: ThemeMode,
    icon: Option<Icon>,
    label: &'a str,
    kind: ButtonKind,
    message: Option<Message>,
) -> AppElement<'a> {
    let palette = tokens(theme_mode);

    let content = if let Some(icon) = icon {
        row![
            lucide_icon(icon, 15.0, palette.text_primary),
            t_body(label, 16.0, palette.text_primary),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    } else {
        row![t_body(label, 16.0, palette.text_primary)]
            .spacing(8)
            .align_y(Alignment::Center)
    };

    button(content)
    .padding([10, 14])
    .style(button_style(theme_mode, kind))
    .on_press_maybe(message)
    .into()
}
