//! Text fields. Filled (no rest border, fill-over-outline); focus brings a bronze
//! ring. Native `text_input` is kept (caret/selection/IME), styled to the system.

use crate::ui::theme::{ThemeMode, border, mix, palette, with_alpha};
use crate::ui::tokens::radius;
use crate::utils::fonts::montserrat_regular;
use iced::widget::text_input;
use iced::{Element, Length, Theme};

pub fn text_input_style(
    mode: ThemeMode,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style + Clone {
    move |_t: &Theme, status| {
        let p = palette(mode);
        let mut style = text_input::Style {
            background: p.raised.into(),
            border: border(radius::MD, 0.0, p.raised),
            icon: p.text_dim,
            placeholder: p.text_muted,
            value: p.text,
            selection: with_alpha(p.accent, 0.28),
        };

        match status {
            text_input::Status::Active => style,
            text_input::Status::Hovered => {
                style.background = mix(p.raised, p.text, 0.04).into();
                style
            }
            text_input::Status::Focused { .. } => {
                style.border = border(radius::MD, 1.5, p.accent);
                style
            }
            text_input::Status::Disabled => {
                style.background = with_alpha(p.raised, 0.5).into();
                style.value = p.text_muted;
                style
            }
        }
    }
}

pub fn t_input_box<'a, M: Clone + 'a>(
    theme_mode: ThemeMode,
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> M + 'a,
    width: Length,
    on_submit: Option<M>,
) -> Element<'a, M> {
    let mut field = text_input(placeholder, value)
        .width(width)
        .padding([12, 14])
        .size(15)
        .font(montserrat_regular())
        .on_input(on_input)
        .style(text_input_style(theme_mode));

    if let Some(message) = on_submit {
        field = field.on_submit(message);
    }

    field.into()
}
