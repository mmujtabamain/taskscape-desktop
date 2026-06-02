//! Entry point for the **background tray service** (`taskscape-tray`).
//!
//! Owns the macOS menu-bar icon, the compact mini window, and the global hotkey.
//! It starts with no visible window and is the IPC *server*: it binds the shared
//! socket and accepts the main app as a client (see `crate::ipc`).

use crate::app::application::{AppRole, run};

pub fn run_tray() -> iced::Result {
    run(AppRole::Tray)
}
