//! The `taskscape-tray` binary: the background menu-bar / mini-window service.
//!
//! Owns the macOS menu-bar icon, the compact mini window, and the global hotkey.
//! It starts with no visible window and is the IPC *server*: it binds the shared
//! socket and accepts the main app as a client (see `common::ipc`).

mod app;

fn main() -> iced::Result {
    app::run()
}
