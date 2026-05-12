use crate::thememanager::helpers::{border, with_alpha};
use crate::thememanager::{ThemeMode, tokens};
use iced::widget::text_input;
use iced::Theme;

pub fn text_input_style(
    mode: ThemeMode,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style + Clone {
    move |_theme: &Theme, status| {
        let palette = tokens(mode);
        let mut style = text_input::Style {
            background: palette.panel_raised.into(),
            border: border(12.0, 1.0, palette.border),
            icon: palette.text_secondary,
            placeholder: palette.text_muted,
            value: palette.text_primary,
            selection: with_alpha(palette.accent, 0.28),
        };

        match status {
            text_input::Status::Active => style,
            text_input::Status::Hovered => {
                style.border.color = palette.border_strong;
                style
            }
            text_input::Status::Focused { .. } => {
                style.border.color = palette.accent;
                style
            }
            text_input::Status::Disabled => {
                style.background = with_alpha(palette.panel_raised, 0.5).into();
                style.value = palette.text_muted;
                style
            }
        }
    }
}
