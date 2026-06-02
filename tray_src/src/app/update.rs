use crate::app::tray::TrayCommand;
use crate::app::{AppTask, Message, TrayApp};
use common::ipc::IpcMessage;
use iced::{Task, keyboard, window};

/// The menu bar icon's screen rect (top-left + size, in physical pixels), used
/// to anchor the mini window beneath the icon when it is opened from the tray.
#[derive(Debug, Clone, Copy)]
struct TrayAnchor {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl TrayApp {
    pub(crate) fn update(&mut self, message: Message) -> AppTask {
        match message {
            Message::FontLoaded => {}
            Message::TitleChanged(value) => self.title_input = value,
            Message::AddTask => {
                let title = self.title_input.trim().to_owned();
                if !title.is_empty() {
                    self.tasks.add(title.clone());
                    self.title_input.clear();
                    self.status_message = String::from("Task added.");
                    self.broadcast(IpcMessage::AddTask { title });
                }
            }
            Message::RemoveTask(index) => {
                if self.tasks.remove(index) {
                    self.status_message = String::from("Task removed.");
                    self.broadcast(IpcMessage::RemoveTask { index });
                }
            }
            Message::ToggleTaskCompleted(index, completed) => {
                if self.tasks.set_completed(index, completed) {
                    self.status_message = if completed {
                        String::from("Task marked complete.")
                    } else {
                        String::from("Task marked open.")
                    };
                    self.broadcast(IpcMessage::ToggleTaskCompleted { index, completed });
                }
            }
            Message::WindowOpened(window_id) => {
                if self.mini_window_id == Some(window_id) {
                    return Task::none();
                }
                // This is the hidden bootstrap window: install the tray icon and
                // global hotkey on the UI thread, then close it. The daemon keeps
                // running with zero windows; the mini window opens on demand.
                self.bootstrap_window_id = Some(window_id);
                let install_tray = window::run(window_id, |_window| crate::app::tray::install())
                    .map(Message::TrayInstalled);
                let install_hotkey =
                    window::run(window_id, |_window| crate::app::hotkey::install())
                        .map(Message::HotkeyInstalled);
                let close_bootstrap = window::close(window_id);
                return Task::batch([install_tray, install_hotkey, close_bootstrap]);
            }
            Message::WindowClosed(window_id) => {
                if self.mini_window_id == Some(window_id) {
                    self.mini_window_id = None;
                } else if self.bootstrap_window_id == Some(window_id) {
                    self.bootstrap_window_id = None;
                }
            }
            Message::WindowCloseRequested(id) => {
                // The mini window simply closes and forgets its id; clicking the
                // tray icon (or the hotkey) re-opens it.
                if self.mini_window_id == Some(id) {
                    self.mini_window_id = None;
                    return window::close(id);
                }
            }
            Message::TrayEvent(command) => match command {
                TrayCommand::ShowWindow {
                    icon_x,
                    icon_y,
                    icon_width,
                    icon_height,
                } => {
                    let anchor = TrayAnchor {
                        x: icon_x,
                        y: icon_y,
                        width: icon_width,
                        height: icon_height,
                    };
                    return self.toggle_mini_window(Some(anchor));
                }
                TrayCommand::Quit => return self.quit(),
            },
            Message::QuitRequested => return self.quit(),
            Message::TrayInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Menu bar icon: {error}");
                }
            }
            Message::HotkeyEvent(command) => {
                use crate::app::hotkey::HotkeyCommand;
                match command {
                    HotkeyCommand::ToggleMini => return self.toggle_mini_window(None),
                }
            }
            Message::HotkeyInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Global hotkey: {error}");
                }
            }
            Message::IpcEvent(event) => return self.handle_ipc(event),
            Message::KeyboardEvent(event) => {
                if let keyboard::Event::KeyPressed { key, .. } = event {
                    use iced::keyboard::Key;
                    // Esc closes the mini window if it is open.
                    if let (Key::Named(iced::keyboard::key::Named::Escape), Some(id)) =
                        (key.as_ref(), self.mini_window_id)
                    {
                        self.mini_window_id = None;
                        return window::close(id);
                    }
                }
            }
        }

        Task::none()
    }

    /// Quits the tray service. Tells the main app we're going (best-effort) so it
    /// shows "service offline", then exits the process.
    fn quit(&mut self) -> AppTask {
        if self.ipc_connected {
            common::ipc::server::send(&IpcMessage::Bye);
        }
        iced::exit()
    }

    /// Toggles the compact mini window: open + focus it if hidden, close it if
    /// already showing. Shared by the menu bar icon and the keyboard shortcut.
    fn toggle_mini_window(&mut self, anchor: Option<TrayAnchor>) -> AppTask {
        if let Some(id) = self.mini_window_id.take() {
            self.status_message = String::from("Mini window closed.");
            return window::close(id);
        }

        let mut settings = Self::mini_window_settings();
        if let Some(anchor) = anchor {
            settings.position = Self::mini_window_position(&anchor, settings.size);
        }

        let (id, open) = window::open(settings);
        self.mini_window_id = Some(id);
        self.status_message = String::from("Mini window opened.");
        Task::batch([open.map(Message::WindowOpened), window::gain_focus(id)])
    }

    /// Computes a window position (in logical points) that horizontally centers
    /// the mini window under the menu bar icon and tucks it just below the menu
    /// bar, converting the icon's physical-pixel rect via the display scale.
    fn mini_window_position(anchor: &TrayAnchor, window_size: iced::Size) -> window::Position {
        // Gap between the menu bar and the window's top edge, in logical points.
        const GAP: f32 = 6.0;

        let scale = crate::app::tray::main_screen_scale().max(1.0);

        let icon_center_x = (anchor.x + anchor.width / 2.0) / scale;
        let icon_bottom_y = (anchor.y + anchor.height) / scale;

        let mut x = icon_center_x as f32 - window_size.width / 2.0;
        let y = icon_bottom_y as f32 + GAP;

        // Keep the left edge on screen; iced clamps the right edge against the
        // monitor.
        if x < GAP {
            x = GAP;
        }

        window::Position::Specific(iced::Point::new(x, y))
    }
}
