use crate::app::{AppElement, Message};
use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use crate::widgets::{t_body, lucide_icon};
use iced::Alignment;
use iced::widget::{button, row};
use lucide_icons::Icon;

pub fn t_icon_button(
    theme_mode: ThemeMode,
    symbol: Icon,
    count: Option<u32>,
    message: Option<Message>,
) -> AppElement<'static> {
    let palette = tokens(theme_mode);
    let content = if let Some(value) = count {
        row![
            lucide_icon(symbol, 16.0, palette.text_primary),
            t_body(value.to_string(), 14.0, palette.text_primary),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    } else {
        row![lucide_icon(symbol, 16.0, palette.text_primary)]
            .align_y(Alignment::Center)
    };

    button(content)
    .padding([10, 12])
    .style(button_style(theme_mode, ButtonKind::Icon))
    .on_press_maybe(message)
    .into()
}
