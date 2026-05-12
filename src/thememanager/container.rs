use crate::thememanager::helpers::{background_gradient, border, with_alpha};
use crate::thememanager::{ThemeMode, tokens};
use iced::widget::container;
use iced::Theme;

pub fn shell_container(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);

        container::Style::default()
            .color(palette.text_primary)
            .background(background_gradient(mode))
    }
}

pub fn panel_alt_container(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);

        container::Style::default()
            .color(palette.text_primary)
            .background(palette.panel_alt)
            .border(border(16.0, 1.0, palette.border))
    }
}

pub fn empty_state_container(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);

        container::Style::default()
            .color(palette.text_primary)
            .background(with_alpha(palette.panel_raised, 0.58))
            .border(border(18.0, 1.0, with_alpha(palette.border_strong, 0.55)))
    }
}
