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

/// Like [`shell_container`], but with rounded corners and a hairline border for
/// the borderless, transparent mini window. The window must be opened with
/// `transparent: true` so the corners outside this radius stay see-through.
pub fn mini_shell_container(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);

        container::Style::default()
            .color(palette.text_primary)
            .background(background_gradient(mode))
            .border(border(16.0, 1.0, with_alpha(palette.border_strong, 0.6)))
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

/// A list row. When `selected`, it gets an accent-tinted fill and an accent
/// border so the open list reads clearly (the row text uses `palette.accent`,
/// not the near-black `accent_text` which is only legible on a solid accent
/// button).
pub fn list_row_container(
    mode: ThemeMode,
    selected: bool,
) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);
        if selected {
            container::Style::default()
                .color(palette.text_primary)
                .background(with_alpha(palette.accent, 0.16))
                .border(border(12.0, 1.0, with_alpha(palette.accent, 0.55)))
        } else {
            container::Style::default()
                .color(palette.text_primary)
                .background(with_alpha(palette.panel_raised, 0.5))
                .border(border(12.0, 1.0, palette.border))
        }
    }
}

/// The left sidebar (both the collapsed rail and the expanded panel): square
/// corners, no border, and a soft shadow cast onto the content area to its right
/// so it reads as a raised surface.
pub fn sidebar_container(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);
        container::Style::default()
            .color(palette.text_primary)
            .background(palette.panel_alt)
            .border(border(0.0, 0.0, palette.border))
            .shadow(iced::Shadow {
                color: palette.shadow,
                // Cast to the right, onto the content area.
                offset: iced::Vector::new(3.0, 0.0),
                blur_radius: 12.0,
            })
    }
}
