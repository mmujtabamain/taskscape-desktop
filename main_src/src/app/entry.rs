//! Entry point for the **main-window app** (`taskscape`).
//!
//! Owns the full task window, the native menu, and file/undo/redo. It is the
//! source of truth for the task list and the IPC *client*: it connects to the
//! tray service and, on connect, sends its full list (see `crate::ipc`).

use crate::app::application::{AppRole, run};

pub fn run_main() -> iced::Result {
    run(AppRole::Main)
}
