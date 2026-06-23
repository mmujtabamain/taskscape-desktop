//! Theme mode, the "Concrete & Bronze" palette, and color helpers.
//!
//! A calm gray field with a single warm bronze signal, in two themes. Surfaces
//! separate by **fill / tonal step**, not outlines — `hairline` exists only for
//! places without a fill (or as a `ring` state).

use iced::{Border, Color, Shadow, Theme, Vector, theme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    /// Both modes, in selector order.
    pub const ALL: [ThemeMode; 2] = [ThemeMode::Dark, ThemeMode::Light];

    pub fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark mode",
            Self::Light => "Light mode",
        }
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// The named colors for one theme. Built once per `view` via [`palette`].
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    /// Window base (the deepest surface).
    pub bg: Color,
    /// Mid surface — filled panels, the sidebar, modal cards.
    pub surface: Color,
    /// Top surface — inputs, dropdowns, resting controls.
    pub raised: Color,
    /// Subtle line, used only where there is no fill (a divider).
    pub hairline: Color,
    /// Primary text.
    pub text: Color,
    /// Secondary / supporting text.
    pub text_dim: Color,
    /// Muted text — quietest labels (kept above the AA floor).
    pub text_muted: Color,
    /// The single warm signal: primary action, selection, focus.
    pub accent: Color,
    /// Hover/brightened accent.
    pub accent_hover: Color,
    /// Text/ink that sits on top of an accent fill.
    pub on_accent: Color,
    /// Focus / selection ring (a state cue, never a rest border).
    pub ring: Color,
    pub success: Color,
    pub danger: Color,
    pub warning: Color,
    /// Dim backdrop behind a modal.
    pub scrim: Color,
}

pub fn color(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

pub fn mix(from: Color, to: Color, amount: f32) -> Color {
    let t = amount.clamp(0.0, 1.0);
    Color {
        r: from.r + (to.r - from.r) * t,
        g: from.g + (to.g - from.g) * t,
        b: from.b + (to.b - from.b) * t,
        a: from.a + (to.a - from.a) * t,
    }
}

pub fn border(radius: f32, width: f32, color: Color) -> Border {
    Border {
        color,
        width,
        radius: radius.into(),
    }
}

pub fn shadow(color: Color, offset: Vector, blur_radius: f32) -> Shadow {
    Shadow {
        color,
        offset,
        blur_radius,
    }
}

/// The named colors for `mode`.
pub fn palette(mode: ThemeMode) -> Palette {
    match mode {
        ThemeMode::Dark => Palette {
            bg: color(0x16, 0x17, 0x19),
            surface: color(0x1D, 0x1F, 0x22),
            raised: color(0x26, 0x28, 0x2C),
            hairline: with_alpha(color(0xC8, 0xCE, 0xD6), 0.10),
            text: color(0xE7, 0xE8, 0xEA),
            text_dim: color(0xA2, 0xA7, 0xAE),
            text_muted: color(0x8A, 0x90, 0x98),
            accent: color(0xB5, 0x82, 0x5A),
            accent_hover: color(0xC6, 0x97, 0x71),
            on_accent: color(0x1A, 0x14, 0x10),
            ring: with_alpha(color(0xB5, 0x82, 0x5A), 0.55),
            success: color(0x72, 0xB0, 0x7D),
            danger: color(0xD6, 0x7D, 0x67),
            warning: color(0xD9, 0xA4, 0x45),
            scrim: with_alpha(color(0x00, 0x00, 0x00), 0.55),
        },
        ThemeMode::Light => Palette {
            bg: color(0xF2, 0xF3, 0xF4),
            surface: color(0xFA, 0xFA, 0xFB),
            raised: color(0xFF, 0xFF, 0xFF),
            hairline: with_alpha(color(0x1A, 0x1C, 0x20), 0.12),
            text: color(0x18, 0x1B, 0x1F),
            text_dim: color(0x55, 0x5B, 0x62),
            text_muted: color(0x6E, 0x74, 0x7C),
            accent: color(0x8A, 0x5A, 0x36),
            accent_hover: color(0x9E, 0x6A, 0x40),
            on_accent: color(0xFC, 0xF8, 0xF4),
            ring: with_alpha(color(0x8A, 0x5A, 0x36), 0.45),
            success: color(0x3E, 0x7D, 0x4E),
            danger: color(0xB8, 0x54, 0x3F),
            warning: color(0x8A, 0x60, 0x12),
            scrim: with_alpha(color(0x1A, 0x1C, 0x20), 0.35),
        },
    }
}

/// The base `iced::Theme` (drives any still-native widgets until they're replaced).
pub fn app_theme(mode: ThemeMode) -> Theme {
    let p = palette(mode);
    Theme::custom(
        format!("Taskscape {}", mode.label()),
        theme::Palette {
            background: p.bg,
            text: p.text,
            primary: p.accent,
            success: p.success,
            danger: p.danger,
            warning: p.warning,
        },
    )
}
