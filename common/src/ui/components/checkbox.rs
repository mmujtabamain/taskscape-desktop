//! A checkbox built on [`Interactive`]: an outlined box that becomes a bronze fill
//! with a sharp check when set. Hover/press are animated; the checked state is the
//! caller's (the complete-task "draw-in" is handled as an app-state moment).

use crate::ui::components::icon::{Icon, icon};
use crate::ui::components::interactive::{Interactive, Style, Surface};
use crate::ui::theme::{ThemeMode, mix, palette, with_alpha};
use crate::ui::tokens::radius;
use iced::widget::{Space, container};
use iced::{Element, Length};

/// `on_toggle` should be the message for the *new* state (caller passes the toggle).
pub fn t_checkbox<'a, M: Clone + 'a>(
    theme_mode: ThemeMode,
    checked: bool,
    on_toggle: M,
    size: f32,
) -> Element<'a, M> {
    let p = palette(theme_mode);

    let style = if checked {
        Style {
            rest: Surface::new(p.accent, 0.0),
            hover: Surface::new(p.accent_hover, 0.0),
            pressed: Surface::new(mix(p.accent, p.on_accent, 0.14), 0.0),
            radius: radius::SM,
            ring: None,
        }
    } else {
        Style {
            rest: Surface::new(with_alpha(p.text, 0.0), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.11), 0.0),
            pressed: Surface::new(with_alpha(p.text, 0.16), 0.0),
            radius: radius::SM,
            ring: Some((with_alpha(p.text, 0.28), 1.5)),
        }
    };

    let glyph: Element<'a, M> = if checked {
        icon(Icon::Check, size * 0.72, p.on_accent).into()
    } else {
        Space::new().width(Length::Fixed(0.0)).height(Length::Fixed(0.0)).into()
    };

    let body = container(glyph)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .center_x(Length::Fixed(size))
        .center_y(Length::Fixed(size));

    Interactive::new(body, style).on_press(on_toggle).into()
}
