//! Icon-only buttons, built on the animated [`Interactive`] surface.

use crate::ui::components::button::{ButtonKind, surface_style};
use crate::ui::components::icon::{Icon, icon};
use crate::ui::components::interactive::Interactive;
use crate::ui::components::typography::t_body;
use crate::ui::theme::{ThemeMode, palette};
use crate::ui::tokens::{space, text};
use iced::widget::row;
use iced::{Alignment, Element};

/// A filled-feeling icon button, optionally with a trailing count.
pub fn t_icon_button<M: Clone + 'static>(
    theme_mode: ThemeMode,
    symbol: Icon,
    count: Option<u32>,
    message: Option<M>,
) -> Element<'static, M> {
    let p = palette(theme_mode);
    let content = match count {
        Some(value) => row![
            icon(symbol, text::BODY + 2.0, p.text),
            t_body(value.to_string(), text::BODY, p.text),
        ]
        .spacing(space::SM)
        .align_y(Alignment::Center),
        None => row![icon(symbol, text::BODY + 2.0, p.text)].align_y(Alignment::Center),
    };

    Interactive::new(content, surface_style(theme_mode, ButtonKind::Icon))
        .padding([8, 10])
        .on_press_maybe(message)
        .into()
}

/// A borderless, transparent-until-hover icon button for nested row actions.
pub fn t_icon_button_ghost<M: Clone + 'static>(
    theme_mode: ThemeMode,
    symbol: Icon,
    message: Option<M>,
) -> Element<'static, M> {
    let p = palette(theme_mode);
    Interactive::new(
        row![icon(symbol, text::BODY + 1.0, p.text_dim)].align_y(Alignment::Center),
        surface_style(theme_mode, ButtonKind::Plain),
    )
    .padding([6, 7])
    .on_press_maybe(message)
    .into()
}
