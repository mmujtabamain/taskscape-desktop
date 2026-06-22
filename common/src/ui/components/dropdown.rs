//! Select / dropdown. Filled field (no rest border); hover/open brings a ring.

use crate::ui::theme::{ThemeMode, border, mix, palette};
use crate::ui::tokens::radius;
use iced::widget::pick_list;
use iced::{Element, Length, Theme};
use std::borrow::Borrow;

pub fn pick_list_style(
    mode: ThemeMode,
) -> impl Fn(&Theme, pick_list::Status) -> pick_list::Style + Clone {
    move |_t: &Theme, status| {
        let p = palette(mode);
        let mut style = pick_list::Style {
            text_color: p.text,
            placeholder_color: p.text_muted,
            handle_color: p.text_dim,
            background: p.raised.into(),
            border: border(radius::MD, 0.0, p.raised),
        };

        if matches!(
            status,
            pick_list::Status::Hovered | pick_list::Status::Opened { .. }
        ) {
            style.background = mix(p.raised, p.text, 0.04).into();
            style.border = border(radius::MD, 1.5, p.accent);
        }

        style
    }
}

pub fn t_dropdown<'a, M, T, L, V>(
    theme_mode: ThemeMode,
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> M + 'a,
    width: Length,
) -> Element<'a, M>
where
    M: Clone + 'a,
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
{
    pick_list(options, selected, on_select)
        .width(width)
        .padding([12, 14])
        .text_size(15)
        .style(pick_list_style(theme_mode))
        .into()
}
