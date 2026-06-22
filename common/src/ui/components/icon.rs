//! Sharp-cornered icons (Material Symbols Sharp subset).
//!
//! Each [`Icon`] maps to a glyph codepoint in the embedded subset font. To add an
//! icon: add a variant + its codepoint here, add the Material Symbols name to
//! `assets/fonts/MaterialSymbols/used-icons.txt`, and run `regen-subset.sh`.

use crate::utils::fonts::icon_font;
use iced::Color;
use iced::widget::{Text, text};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    PanelToggle,
    PanelOpen,
    PanelClose,
    Import,
    Export,
    ThemeLight,
    ThemeDark,
    Undo,
    Redo,
    Delete,
    Attach,
    Camera,
    Add,
    AddCircle,
    Settings,
    Edit,
    Check,
    CheckAll,
    ListAdd,
    Cancel,
    Dot,
    Power,
    Enter,
    Checklist,
    ChevronDown,
    Keyboard,
    Reset,
    Close,
}

impl Icon {
    /// The Material Symbols Sharp codepoint for this icon.
    pub const fn glyph(self) -> char {
        match self {
            Icon::PanelToggle => '\u{e9bd}',  // menu_open
            Icon::PanelOpen => '\u{f716}',    // left_panel_open
            Icon::PanelClose => '\u{f717}',   // left_panel_close
            Icon::Import => '\u{f090}',       // download
            Icon::Export => '\u{f09b}',       // upload
            Icon::ThemeLight => '\u{e518}',   // light_mode
            Icon::ThemeDark => '\u{e51c}',    // dark_mode
            Icon::Undo => '\u{e166}',         // undo
            Icon::Redo => '\u{e15a}',         // redo
            Icon::Delete => '\u{e92e}',       // delete
            Icon::Attach => '\u{e226}',       // attach_file
            Icon::Camera => '\u{e412}',       // photo_camera
            Icon::Add => '\u{e145}',          // add
            Icon::AddCircle => '\u{e990}',    // add_circle
            Icon::Settings => '\u{e8b8}',     // settings
            Icon::Edit => '\u{f097}',         // edit
            Icon::Check => '\u{e668}',        // check
            Icon::CheckAll => '\u{e877}',     // done_all
            Icon::ListAdd => '\u{e03b}',      // playlist_add
            Icon::Cancel => '\u{e888}',       // cancel
            Icon::Dot => '\u{e061}',          // fiber_manual_record
            Icon::Power => '\u{f8c7}',        // power_settings_new
            Icon::Enter => '\u{e31b}',        // keyboard_return
            Icon::Checklist => '\u{e6b1}',    // checklist
            Icon::ChevronDown => '\u{e313}',  // keyboard_arrow_down
            Icon::Keyboard => '\u{e312}',     // keyboard
            Icon::Reset => '\u{f053}',        // restart_alt
            Icon::Close => '\u{e5cd}',        // close
        }
    }
}

/// A single icon glyph rendered in the icon font at `size`/`color`.
pub fn icon(symbol: Icon, size: f32, color: Color) -> Text<'static> {
    text(symbol.glyph().to_string())
        .font(icon_font())
        .size(size)
        .color(color)
}
