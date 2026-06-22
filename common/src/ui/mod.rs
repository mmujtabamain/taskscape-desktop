//! The redesigned UI layer — "Concrete & Bronze".
//!
//! A self-contained design system: geometry/type [`tokens`], the gray+bronze
//! [`theme`], [`motion`] presets, and the animated [`components`] toolkit. This
//! replaces the legacy `thememanager` + `widgets` modules once the screens migrate.

pub mod components;
pub mod motion;
pub mod theme;
pub mod tokens;

pub use components::*;
pub use theme::{
    Palette, ThemeMode, app_theme, border, color, mix, palette, shadow, with_alpha,
};
