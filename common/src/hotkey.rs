//! A serializable, process-neutral description of a global hotkey.
//!
//! The main app *captures* a hotkey (from an iced key event) and *displays* it,
//! but only the tray service actually registers it with the OS. So the binding
//! travels between them — persisted in the config and sent over IPC — as this
//! small [`HotkeySpec`] (modifier flags + a physical key-code name), which the
//! tray converts into a real `global_hotkey::HotKey`.
//!
//! `code` is a W3C UI Events `code` name (e.g. `"Backquote"`, `"KeyK"`, `"F2"`),
//! which is exactly what iced's `keyboard::key::Code` debug-prints *and* what
//! `global-hotkey`'s parser accepts, so the two ends agree without a lookup table.

use serde::{Deserialize, Serialize};

/// A global-hotkey binding: which modifiers are held plus the main key's code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotkeySpec {
    #[serde(default)]
    pub alt: bool,
    #[serde(default)]
    pub ctrl: bool,
    #[serde(default)]
    pub shift: bool,
    /// Command (⌘) on macOS, Super/Windows elsewhere.
    #[serde(default)]
    pub meta: bool,
    /// W3C key-code name, e.g. `"Backquote"`, `"KeyK"`, `"F2"`.
    pub code: String,
}

impl HotkeySpec {
    /// The app default for the mini-window toggle: Option/Alt + backtick.
    pub fn default_mini_toggle() -> Self {
        Self {
            alt: true,
            ctrl: false,
            shift: false,
            meta: false,
            code: String::from("Backquote"),
        }
    }

    /// True if at least one "strong" modifier (⌘/⌥/⌃) is held. A global hotkey
    /// without one would hijack a bare key everywhere, so capture requires it.
    pub fn has_strong_modifier(&self) -> bool {
        self.alt || self.ctrl || self.meta
    }

    /// A compact human label such as `"⌥ `"` or `"⌃⌥⇧ K"`. Falls back to `"?"`
    /// for the key if `code` is not recognised.
    pub fn label(&self) -> String {
        let mut out = String::new();
        // macOS convention orders the glyphs ⌃⌥⇧⌘.
        if self.ctrl {
            out.push('⌃');
        }
        if self.alt {
            out.push('⌥');
        }
        if self.shift {
            out.push('⇧');
        }
        if self.meta {
            out.push('⌘');
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(key_code_label(&self.code).unwrap_or("?"));
        out
    }
}

/// True if `code` names a modifier key (which on its own can't be a hotkey's main
/// key). Used while capturing to keep waiting for the *next*, real key press.
pub fn is_modifier_code(code: &str) -> bool {
    matches!(
        code,
        "AltLeft"
            | "AltRight"
            | "ControlLeft"
            | "ControlRight"
            | "ShiftLeft"
            | "ShiftRight"
            | "SuperLeft"
            | "SuperRight"
            | "Meta"
            | "MetaLeft"
            | "MetaRight"
            | "Hyper"
            | "Fn"
            | "FnLock"
    )
}

/// A short, human-friendly label for a W3C key code, or `None` if the code isn't
/// one the tray can register (so this doubles as capture-time validation).
pub fn key_code_label(code: &str) -> Option<&'static str> {
    let label = match code {
        "Backquote" => "`",
        "Backslash" => "\\",
        "BracketLeft" => "[",
        "BracketRight" => "]",
        "Comma" => ",",
        "Equal" => "=",
        "Minus" => "-",
        "Period" => ".",
        "Quote" => "'",
        "Semicolon" => ";",
        "Slash" => "/",
        "Digit0" => "0",
        "Digit1" => "1",
        "Digit2" => "2",
        "Digit3" => "3",
        "Digit4" => "4",
        "Digit5" => "5",
        "Digit6" => "6",
        "Digit7" => "7",
        "Digit8" => "8",
        "Digit9" => "9",
        "KeyA" => "A",
        "KeyB" => "B",
        "KeyC" => "C",
        "KeyD" => "D",
        "KeyE" => "E",
        "KeyF" => "F",
        "KeyG" => "G",
        "KeyH" => "H",
        "KeyI" => "I",
        "KeyJ" => "J",
        "KeyK" => "K",
        "KeyL" => "L",
        "KeyM" => "M",
        "KeyN" => "N",
        "KeyO" => "O",
        "KeyP" => "P",
        "KeyQ" => "Q",
        "KeyR" => "R",
        "KeyS" => "S",
        "KeyT" => "T",
        "KeyU" => "U",
        "KeyV" => "V",
        "KeyW" => "W",
        "KeyX" => "X",
        "KeyY" => "Y",
        "KeyZ" => "Z",
        "Space" => "Space",
        "Enter" => "Enter",
        "Tab" => "Tab",
        "Backspace" => "Backspace",
        "Delete" => "Delete",
        "Escape" => "Esc",
        "Home" => "Home",
        "End" => "End",
        "PageUp" => "PageUp",
        "PageDown" => "PageDown",
        "Insert" => "Insert",
        "ArrowUp" => "↑",
        "ArrowDown" => "↓",
        "ArrowLeft" => "←",
        "ArrowRight" => "→",
        "F1" => "F1",
        "F2" => "F2",
        "F3" => "F3",
        "F4" => "F4",
        "F5" => "F5",
        "F6" => "F6",
        "F7" => "F7",
        "F8" => "F8",
        "F9" => "F9",
        "F10" => "F10",
        "F11" => "F11",
        "F12" => "F12",
        _ => return None,
    };
    Some(label)
}
