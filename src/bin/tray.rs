//! The `taskscape-tray` binary: the background menu-bar / mini-window service.

fn main() -> iced::Result {
    taskscape::app::tray_app::run_tray()
}
