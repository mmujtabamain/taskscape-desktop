//! Container style factories for the redesign.
//!
//! Fill-over-outline: filled surfaces carry **no border** — separation is by tone.
//! The mini window is the one glass surface (transparent fill + a defining edge,
//! since it has no opaque fill). The main window is solid matte.

use crate::ui::theme::{Palette, ThemeMode, border, palette, shadow};
use crate::ui::tokens::{HAIRLINE_WIDTH, radius};
use iced::widget::container;
use iced::{Theme, Vector};

fn base(p: &Palette) -> container::Style {
    container::Style::default().color(p.text)
}

/// The solid main-window shell (matte, opaque).
pub fn shell(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p).background(p.bg)
    }
}

/// The frosted mini-window shell: a faint tint + defining edge laid over the native
/// vibrancy. The window itself is opened transparent so the blur shows through.
pub fn glass_shell(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p)
            .background(p.glass_tint)
            .border(border(radius::XL, 1.0, p.glass_edge))
    }
}

/// The frosted **main-window** shell: just the faint glass tint, full-bleed with
/// no border or radius (the native window frame supplies the edge + rounded
/// corners). Laid over the native vibrancy backdrop (`chrome::apply`) so the
/// desktop shows through, blurred.
pub fn frosted_shell(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p).background(p.glass_tint)
    }
}

/// A mid surface (filled, no border) — panels, the task-list area.
pub fn surface(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p).background(p.surface).border(border(radius::LG, 0.0, p.surface))
    }
}

/// A top surface (filled, no border) — quiet raised areas.
pub fn raised(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p).background(p.raised).border(border(radius::MD, 0.0, p.raised))
    }
}

/// The dimmed backdrop behind a modal.
pub fn modal_backdrop(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        container::Style::default().background(p.scrim)
    }
}

/// A centered modal card (one of the few legitimate cards) — filled, soft shadow.
pub fn modal_card(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p)
            .background(p.surface)
            .border(border(radius::XL, 0.0, p.surface))
            .shadow(shadow(p.scrim, Vector::new(0.0, 12.0), 32.0))
    }
}

/// The left sidebar — filled (no border), full-bleed square outer edge, with a soft
/// shadow cast onto the content area so it reads as a raised plane.
pub fn sidebar(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p)
            .background(p.surface)
            .shadow(shadow(p.scrim, Vector::new(3.0, 0.0), 14.0))
    }
}

/// A thin status/footer bar — filled, no border.
pub fn bar(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p).background(p.surface)
    }
}

/// A hairline divider (a place with *no* fill, so a line is sanctioned).
pub fn divider(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        container::Style::default()
            .background(p.hairline)
            .border(border(0.0, HAIRLINE_WIDTH, p.hairline))
    }
}
