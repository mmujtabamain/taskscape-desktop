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

/// A dense task row in the mini window: subtle raised fill, tight radius, faint
/// border. `done` mutes it to read as completed.
pub fn mini_row(mode: ThemeMode, done: bool) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);
        let alpha = if done { 0.28 } else { 0.55 };
        container::Style::default()
            .color(palette.text_primary)
            .background(with_alpha(palette.panel_raised, alpha))
            .border(border(9.0, 1.0, with_alpha(palette.border, if done { 0.6 } else { 1.0 })))
    }
}

/// A small rounded count pill for the mini header.
pub fn mini_chip(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);
        container::Style::default()
            .color(palette.text_primary)
            .background(with_alpha(palette.accent, 0.16))
            .border(border(8.0, 1.0, with_alpha(palette.accent, 0.4)))
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

/// Dimmed full-window backdrop behind a modal dialog.
pub fn modal_backdrop(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);
        container::Style::default().background(with_alpha(palette.shadow, 0.55))
    }
}

/// A centered modal dialog card: raised surface, rounded, bordered, with a soft
/// shadow.
pub fn modal_card(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_theme: &Theme| {
        let palette = tokens(mode);
        container::Style::default()
            .color(palette.text_primary)
            .background(palette.panel_alt)
            .border(border(16.0, 1.0, with_alpha(palette.border_strong, 0.6)))
            .shadow(iced::Shadow {
                color: palette.shadow,
                offset: iced::Vector::new(0.0, 8.0),
                blur_radius: 28.0,
            })
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
