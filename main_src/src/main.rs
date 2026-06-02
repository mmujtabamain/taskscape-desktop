//! The `taskscape` binary: the main task window.
//!
//! It is the source of truth for the task list and the IPC *client* — it
//! connects to the tray service and, on connect, sends its full list so the
//! mini window mirrors it (see `common::ipc`).

mod app;

fn main() -> iced::Result {
    app::run()
}
