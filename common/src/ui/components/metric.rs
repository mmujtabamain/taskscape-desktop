//! A flat metric readout (value + label). Deliberately *not* a card — metrics live
//! inline in the status bar.

use crate::ui::components::typography::{t_body, t_caption};
use crate::ui::theme::{ThemeMode, palette};
use crate::ui::tokens::{space, text};
use iced::widget::row;
use iced::{Alignment, Element};

/// `value` in body weight, `label` muted beside it.
pub fn t_metric<'a, M: 'a>(theme_mode: ThemeMode, value: impl Into<String>, label: &'a str) -> Element<'a, M> {
    let p = palette(theme_mode);
    row![
        t_body(value.into(), text::LABEL, p.text_dim),
        t_caption(label, text::CAPTION, p.text_muted),
    ]
    .spacing(space::XS)
    .align_y(Alignment::Center)
    .into()
}
