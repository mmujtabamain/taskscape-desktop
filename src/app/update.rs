use crate::app::{AppTask, Message, Taskscape};
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

impl Taskscape {
    pub(crate) fn update(&mut self, message: Message) -> AppTask {
        match message {
            Message::FontLoaded => {} // Fonts loaded, trigger redraw via Task::none()
            Message::ToggleTheme => {
                self.theme_mode = self.theme_mode.toggled();
                self.status_message = format!("Switched to {}.", self.theme_mode.label());
            }
            Message::TitleChanged(value) => self.title_input = value,
            Message::FileNameChanged(value) => self.file_name_editing = value,
            Message::ToggleTitleEdit => {
                if self.editing_title {
                    // Exiting edit mode - save changes
                    self.file_name = self.file_name_editing.clone();
                }
                self.editing_title = !self.editing_title;
                if self.editing_title {
                    // Entering edit mode - prepare editing and move cursor to end
                    self.file_name_editing = self.file_name.clone();
                    return iced::widget::operation::move_cursor_to_end(
                        crate::widgets::t_editable_title::TITLE_INPUT_ID,
                    );
                }
            }
            Message::ToggleTitleEditCancel => {
                // Discard changes and exit edit mode
                self.file_name_editing = self.file_name.clone();
                self.editing_title = false;
            }
            Message::CancelAllEditing => {
                // Cancel all editing modes in the app
                if self.editing_title {
                    self.file_name_editing = self.file_name.clone();
                    self.editing_title = false;
                }
            }
            Message::DueDateChanged(value) => self.due_date_input = value,
            Message::ToggleTaskCompleted(index, completed) => {
                self.toggle_task_completed(index, completed);
                self.broadcast(crate::ipc::IpcMessage::ToggleTaskCompleted { index, completed });
            }
            Message::AddTask => {
                if let Some(title) = self.add_task() {
                    self.broadcast(crate::ipc::IpcMessage::AddTask { title });
                }
            }
            Message::RemoveTask(index) => {
                self.remove_task(index);
                self.broadcast(crate::ipc::IpcMessage::RemoveTask { index });
            }
            Message::ClearCompleted => {
                self.push_history();
                self.tasks.retain(|task| !task.is_completed());
                self.status_message = String::from("Completed tasks removed.");
            }
            Message::ClearAll => {
                self.push_history();
                self.tasks.clear();
                self.status_message = String::from("All tasks removed.");
            }
            Message::FileNew => self.new_list("Started a new list."),
            Message::FileSave => return self.launch_save_dialog(),
            Message::FileLoad => return Self::launch_load_dialog(),
            Message::FileSaveResult(Some(path)) => self.complete_save(path),
            Message::FileSaveResult(None) => {
                self.status_message = String::from("Save cancelled.");
            }
            Message::FileLoadResult(Some(path)) => self.complete_load(path),
            Message::FileLoadResult(None) => {
                self.status_message = String::from("Load cancelled.");
            }
            Message::EditUndo => self.undo(),
            Message::EditRedo => self.redo(),
            Message::WindowOpened(window_id) => {
                use crate::app::application::AppRole;

                // The mini window records its own id when opened (see TrayEvent),
                // so a window that isn't the mini window is this role's primary
                // window: the main app's task window, or the tray service's hidden
                // bootstrap window.
                if self.mini_window_id == Some(window_id) {
                    return Task::none();
                }

                match self.role {
                    AppRole::Main => {
                        self.window_id = Some(window_id);
                        // The native menu must be installed on the UI thread;
                        // window::run guarantees that.
                        return window::run(window_id, |window| {
                            crate::app::native_menu::install_for_window(window)
                        })
                        .map(Message::NativeMenuInstalled);
                    }
                    AppRole::Tray => {
                        // This is the hidden bootstrap window: install the tray
                        // icon and global hotkey on the UI thread, then close it.
                        // The daemon keeps running with zero windows; the mini
                        // window opens later on demand.
                        self.window_id = Some(window_id);
                        let install_tray =
                            window::run(window_id, |_window| crate::app::tray::install())
                                .map(Message::TrayInstalled);
                        let install_hotkey =
                            window::run(window_id, |_window| crate::app::hotkey::install())
                                .map(Message::HotkeyInstalled);
                        let close_bootstrap = window::close(window_id);
                        return Task::batch([install_tray, install_hotkey, close_bootstrap]);
                    }
                }
            }
            Message::WindowClosed(window_id) => {
                // Authoritative cleanup: whenever a window actually closes (by any
                // route), forget its id so the toggle logic stays correct.
                if self.mini_window_id == Some(window_id) {
                    self.mini_window_id = None;
                } else if self.window_id == Some(window_id) {
                    self.window_id = None;
                }
            }
            Message::NativeMenuEvent(command) => {
                use crate::app::native_menu::NativeMenuCommand;
                match command {
                    NativeMenuCommand::FileNew => self.new_list("Started a new list."),
                    NativeMenuCommand::FileSave => return self.launch_save_dialog(),
                    NativeMenuCommand::FileLoad => return Self::launch_load_dialog(),
                    NativeMenuCommand::EditUndo => self.undo(),
                    NativeMenuCommand::EditRedo => self.redo(),
                    NativeMenuCommand::ToggleTheme => {
                        self.theme_mode = self.theme_mode.toggled();
                        self.status_message = format!("Switched to {}.", self.theme_mode.label());
                    }
                }
            }
            Message::NativeMenuInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Native menu: {error}");
                }
            }
            Message::WindowCloseRequested(id) => {
                // The mini window simply closes and forgets its id, regardless of
                // platform — clicking the tray icon re-opens it.
                if self.mini_window_id == Some(id) {
                    self.mini_window_id = None;
                    return window::close(id);
                }

                // Main window — macOS: minimize to the Dock instead of quitting,
                // so clicking the Dock icon restores it (the standard behaviour
                // for closing a window while the app keeps running). Other
                // platforms close normally.
                #[cfg(target_os = "macos")]
                {
                    self.status_message = String::from("Minimized to the Dock.");
                    return window::minimize(id, true);
                }
                #[cfg(not(target_os = "macos"))]
                return window::close(id);
            }
            Message::TrayEvent(command) => {
                use crate::app::tray::TrayCommand;
                match command {
                    // Clicking the menu bar icon toggles the compact mini window,
                    // anchored just beneath the icon.
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
                }
            }
            Message::TrayInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Menu bar icon: {error}");
                }
            }
            Message::HotkeyEvent(command) => {
                use crate::app::hotkey::HotkeyCommand;
                match command {
                    // Global Cmd/Ctrl+` toggles the mini window, even when
                    // Taskscape is not the focused application.
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
                if let keyboard::Event::KeyPressed { key, modifiers, .. } = event {
                    use iced::keyboard::Key;

                    // Check for platform-specific window management shortcuts
                    if let Some(task) = self.platform_window_shortcuts(&key, &modifiers) {
                        return task;
                    }

                    let command = modifiers.command();

                    match key.as_ref() {
                        Key::Character("z") if command && modifiers.shift() => self.redo(),
                        Key::Character("z") if command => self.undo(),
                        Key::Character("s") if command => return self.launch_save_dialog(),
                        Key::Character("o") if command => return Self::launch_load_dialog(),
                        Key::Character("n") if command => self.new_list("Started a new list."),
                        Key::Character("t") if command => {
                            self.theme_mode = self.theme_mode.toggled();
                            self.status_message =
                                format!("Switched to {}.", self.theme_mode.label());
                        }
                        Key::Named(iced::keyboard::key::Named::Escape) if self.editing_title => {
                            // Discard title edit on ESC
                            self.file_name_editing = self.file_name.clone();
                            self.editing_title = false;
                        }
                        _ => {}
                    }
                }
            }
        }

        Task::none()
    }

    /// Toggles the compact mini window: open + focus it if hidden, close it if
    /// already showing. Shared by the menu bar icon and the keyboard shortcut.
    ///
    /// When `anchor` is given (a tray-icon click), the window opens just beneath
    /// the icon; otherwise (the keyboard shortcut) it falls back to centered.
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

        // Center of the icon, and its bottom edge, in logical points.
        let icon_center_x = (anchor.x + anchor.width / 2.0) / scale;
        let icon_bottom_y = (anchor.y + anchor.height) / scale;

        let mut x = icon_center_x as f32 - window_size.width / 2.0;
        let y = icon_bottom_y as f32 + GAP;

        // Keep the window fully on screen: clamp against the visible width of the
        // display the icon lives on (its logical width is x-of-right-edge ≈ the
        // monitor width). We don't have the monitor bounds here, so only guard
        // the left edge; iced clamps the right edge against the monitor.
        if x < GAP {
            x = GAP;
        }

        window::Position::Specific(iced::Point::new(x, y))
    }

    /// Platform-specific window management shortcuts
    fn platform_window_shortcuts(
        &self,
        key: &iced::keyboard::Key,
        modifiers: &keyboard::Modifiers,
    ) -> Option<AppTask> {
        use iced::keyboard::Key;

        let id = self.window_id?;

        // ===== macOS =====
        #[cfg(target_os = "macos")]
        {
            match key.as_ref() {
                // Globe key shortcuts
                Key::Character("f") if modifiers.command() => {
                    return Some(window::toggle_maximize(id));
                }
                Key::Character("m") if modifiers.command() => {
                    return Some(window::minimize(id, true));
                }
                Key::Character("q") if modifiers.command() => {
                    // Real quit.
                    return Some(window::close(id));
                }
                Key::Character("w") if modifiers.command() => {
                    // Minimize to the Dock rather than hide into the menu bar:
                    // a minimized window leaves a Dock thumbnail, so clicking the
                    // Dock icon restores it natively (winit/iced do not surface
                    // the macOS app-reopen event, so a hidden window could not be
                    // brought back this way).
                    return Some(window::minimize(id, true));
                }

                _ => {}
            }
        }

        // ===== Windows =====
        #[cfg(target_os = "windows")]
        {
            let logo = modifiers.logo(); // Windows key

            match key.as_ref() {
                Key::Character("f") if logo => return Some(window::toggle_maximize(id)),
                Key::Character("m") if logo => return Some(window::minimize(id, true)),
                Key::Character("w") if logo => return Some(window::close(id)),

                // Alt+F4 (Windows quit)
                Key::Named(iced::keyboard::key::Named::F4) if modifiers.alt() => {
                    return Some(window::close(id));
                }

                _ => {}
            }
        }

        // ===== Linux =====
        #[cfg(target_os = "linux")]
        {
            let logo = modifiers.logo(); // Super key

            match key.as_ref() {
                Key::Character("f") if logo => return Some(window::toggle_maximize(id)),
                Key::Character("m") if logo => return Some(window::minimize(id, true)),
                Key::Character("w") if logo => return Some(window::close(id)),

                // Alt+F4 (Linux quit)
                Key::Named(iced::keyboard::key::Named::F4) if modifiers.alt() => {
                    return Some(window::close(id));
                }

                _ => {}
            }
        }

        None
    }
}
