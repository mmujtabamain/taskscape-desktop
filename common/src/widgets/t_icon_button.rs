use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use crate::widgets::{t_body, lucide_icon};
use iced::Alignment;
use iced::Element;
use iced::widget::{button, row};
use lucide_icons::Icon;

/// A bordered, filled icon button (`ButtonKind::Icon`).
pub fn t_icon_button<M: Clone + 'static>(
    theme_mode: ThemeMode,
    symbol: Icon,
    count: Option<u32>,
    message: Option<M>,
) -> Element<'static, M> {
    t_icon_button_kind(theme_mode, symbol, count, message, ButtonKind::Icon, [10, 12])
}

/// A small, borderless, background-less icon button — only hover/press introduce
/// a faint background, like a ghost button. Compact, for actions nested inside
/// an already-styled row.
pub fn t_icon_button_ghost<M: Clone + 'static>(
    theme_mode: ThemeMode,
    symbol: Icon,
    message: Option<M>,
) -> Element<'static, M> {
    t_icon_button_kind(theme_mode, symbol, None, message, ButtonKind::Plain, [6, 7])
}

fn t_icon_button_kind<M: Clone + 'static>(
    theme_mode: ThemeMode,
    symbol: Icon,
    count: Option<u32>,
    message: Option<M>,
    kind: ButtonKind,
    padding: [u16; 2],
) -> Element<'static, M> {
    let palette = tokens(theme_mode);
    let content = if let Some(value) = count {
        row![
            lucide_icon(symbol, 16.0, palette.text_primary),
            t_body(value.to_string(), 14.0, palette.text_primary),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    } else {
        row![lucide_icon(symbol, 15.0, palette.text_primary)].align_y(Alignment::Center)
    };

    button(content)
        .padding(padding)
        .style(button_style(theme_mode, kind))
        .on_press_maybe(message)
        .into()
}
