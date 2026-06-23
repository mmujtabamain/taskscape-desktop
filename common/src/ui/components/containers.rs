//! Container style factories for the redesign.
//!
//! Fill-over-outline: filled surfaces carry **no border** — separation is by tone.
//! Both windows are solid matte: the main window is full-bleed (`shell`), the mini
//! window/popover a rounded opaque panel (`mini_shell`).

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

/// The mini-window / popover shell: a solid opaque fill with rounded corners (no
/// border, fill-over-outline). The window is opened transparent so the corners
/// outside this rounded fill read as transparent (clipped via `round_window`).
pub fn mini_shell(mode: ThemeMode) -> impl Fn(&Theme) -> container::Style + Clone {
    move |_t: &Theme| {
        let p = palette(mode);
        base(&p)
            .background(p.bg)
            .border(border(radius::XL, 0.0, p.bg))
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
