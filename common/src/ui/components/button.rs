//! Buttons — built on the animated [`Interactive`] surface (fill-over-outline; the
//! fill lifts/brightens on hover, settles on press).

use crate::ui::components::icon::{Icon, icon};
use crate::ui::components::interactive::{Interactive, Style, Surface};
use crate::ui::components::typography::t_body;
use crate::ui::theme::{ThemeMode, mix, palette, with_alpha};
use crate::ui::tokens::{radius, space, text};
use iced::widget::row;
use iced::{Alignment, Element};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// The single primary action — bronze fill.
    Primary,
    /// Secondary action — a faint fill that strengthens on hover.
    Ghost,
    /// Bordered-feeling icon action (same family as Ghost).
    Icon,
    /// Transparent until hover — for actions nested in an already-styled row.
    Plain,
}

/// The interactive fill ramp for a button kind.
pub fn surface_style(mode: ThemeMode, kind: ButtonKind) -> Style {
    let p = palette(mode);
    match kind {
        ButtonKind::Primary => Style {
            rest: Surface::new(p.accent, 0.0),
            hover: Surface::new(p.accent_hover, -2.0),
            pressed: Surface::new(mix(p.accent, p.on_accent, 0.14), 1.0),
            radius: radius::MD,
            ring: None,
        },
        ButtonKind::Ghost | ButtonKind::Icon => Style {
            rest: Surface::new(with_alpha(p.text, 0.05), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.12), -2.0),
            pressed: Surface::new(with_alpha(p.text, 0.16), 1.0),
            radius: radius::MD,
            ring: None,
        },
        ButtonKind::Plain => Style {
            rest: Surface::new(with_alpha(p.text, 0.0), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.09), -1.5),
            pressed: Surface::new(with_alpha(p.text, 0.14), 1.0),
            radius: radius::MD,
            ring: None,
        },
    }
}

fn content_color(mode: ThemeMode, kind: ButtonKind) -> iced::Color {
    let p = palette(mode);
    match kind {
        ButtonKind::Primary => p.on_accent,
        _ => p.text,
    }
}

/// A labeled button (optional leading icon).
pub fn t_button<'a, M: Clone + 'a>(
    theme_mode: ThemeMode,
    leading: Option<Icon>,
    label: &'a str,
    kind: ButtonKind,
    message: Option<M>,
) -> Element<'a, M> {
    let color = content_color(theme_mode, kind);

    let content = match leading {
        Some(symbol) => row![
            icon(symbol, text::BODY + 2.0, color),
            t_body(label, text::BODY, color),
        ]
        .spacing(space::MD)
        .align_y(Alignment::Center),
        None => row![t_body(label, text::BODY, color)].align_y(Alignment::Center),
    };

    Interactive::new(content, surface_style(theme_mode, kind))
        .padding([10, 14])
        .on_press_maybe(message)
        .into()
}
