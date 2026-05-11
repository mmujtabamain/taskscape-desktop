use crate::thememanager::helpers::{color, with_alpha};
use crate::thememanager::theme_mode::ThemeMode;
use iced::{Color, Theme, theme};

#[derive(Debug, Clone, Copy)]
pub struct AppPalette {
    pub background_top: Color,
    pub background_bottom: Color,
    pub sidebar: Color,
    pub sidebar_active: Color,
    pub panel: Color,
    pub panel_alt: Color,
    pub panel_raised: Color,
    pub border: Color,
    pub border_strong: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub accent_text: Color,
    pub shadow: Color,
}

pub fn app_theme(mode: ThemeMode) -> Theme {
    let palette = match mode {
        ThemeMode::Dark => theme::Palette {
            background: color(0x23, 0x18, 0x14),
            text: color(0xF6, 0xE9, 0xD6),
            primary: color(0xFF, 0x7C, 0x5D),
            success: color(0x81, 0xB2, 0x7D),
            danger: color(0xE2, 0x7F, 0x67),
        },
        ThemeMode::Light => theme::Palette {
            background: color(0xF6, 0xED, 0xDE),
            text: color(0x38, 0x20, 0x17),
            primary: color(0xF2, 0x6E, 0x53),
            success: color(0x7E, 0xA2, 0x74),
            danger: color(0xD1, 0x6B, 0x59),
        },
    };

    Theme::custom(format!("Taskscape {}", mode.label()), palette)
}

pub fn tokens(mode: ThemeMode) -> AppPalette {
    match mode {
        ThemeMode::Dark => AppPalette {
            background_top: color(0x4A, 0x30, 0x1D),
            background_bottom: color(0x2B, 0x1C, 0x16),
            sidebar: color(0x35, 0x2A, 0x24),
            sidebar_active: color(0x5A, 0x31, 0x23),
            panel: color(0x2D, 0x1F, 0x18),
            panel_alt: color(0x3C, 0x2D, 0x26),
            panel_raised: color(0x2B, 0x21, 0x1B),
            border: with_alpha(color(0xC5, 0x9A, 0x74), 0.18),
            border_strong: with_alpha(color(0xF2, 0xA1, 0x75), 0.5),
            text_primary: color(0xF5, 0xE7, 0xD4),
            text_secondary: color(0xC9, 0xB1, 0x99),
            text_muted: color(0x9B, 0x7F, 0x6C),
            accent: color(0xFF, 0x7C, 0x5D),
            accent_soft: color(0x6A, 0x3C, 0x2E),
            accent_text: color(0x21, 0x11, 0x0E),
            shadow: with_alpha(color(0x08, 0x05, 0x04), 0.35),
        },
        ThemeMode::Light => AppPalette {
            background_top: color(0xF6, 0xE9, 0xD8),
            background_bottom: color(0xF1, 0xE5, 0xD4),
            sidebar: color(0xF4, 0xEE, 0xE2),
            sidebar_active: color(0xF6, 0xC9, 0xBA),
            panel: color(0xF3, 0xE5, 0xD4),
            panel_alt: color(0xF7, 0xF0, 0xE3),
            panel_raised: color(0xFB, 0xF7, 0xEF),
            border: with_alpha(color(0x8E, 0x63, 0x4E), 0.18),
            border_strong: with_alpha(color(0xE6, 0x8B, 0x6B), 0.42),
            text_primary: color(0x31, 0x1D, 0x15),
            text_secondary: color(0x7A, 0x61, 0x53),
            text_muted: color(0x9B, 0x84, 0x75),
            accent: color(0xF2, 0x6E, 0x53),
            accent_soft: color(0xF7, 0xD8, 0xCC),
            accent_text: color(0xFE, 0xF8, 0xF1),
            shadow: with_alpha(color(0x6B, 0x4D, 0x3C), 0.10),
        },
    }
}
