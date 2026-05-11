use crate::app::{AppElement, Message};
use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use iced::alignment;
use iced::widget::{button, text};

pub fn icon_button(
    theme_mode: ThemeMode,
    symbol: &'static str,
    count: Option<u32>,
    message: Option<Message>,
) -> AppElement<'static> {
    let palette = tokens(theme_mode);
    let label = count
        .map(|value| format!("{} {}", symbol, value))
        .unwrap_or_else(|| symbol.to_owned());

    button(
        text(label)
            .size(16)
            .style(palette.text_primary)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding([10, 12])
    .style(button_style(theme_mode, ButtonKind::Icon))
    .on_press_maybe(message)
    .into()
}
