use crate::app::{AppTask, Message, Taskscape};
use iced::{Task, keyboard, window};

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
                self.toggle_task_completed(index, completed)
            }
            Message::AddTask => self.add_task(),
            Message::RemoveTask(index) => self.remove_task(index),
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
                // The mini window records its own id when opened (see TrayEvent),
                // so anything else is the main window. Only the main window gets
                // the native menu and menu bar icon installed.
                if self.mini_window_id == Some(window_id) {
                    return Task::none();
                }
                self.window_id = Some(window_id);
                // Both installers must run on the UI thread; window::run guarantees that.
                let install_menu = window::run(window_id, |window| {
                    crate::app::native_menu::install_for_window(window)
                })
                .map(Message::NativeMenuInstalled);
                let install_tray = window::run(window_id, |_window| crate::app::tray::install())
                    .map(Message::TrayInstalled);
                return Task::batch([install_menu, install_tray]);
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

                // Main window — macOS: hide into the menu bar instead of quitting.
                // Clicking the tray icon brings the window back. Other platforms
                // close normally.
                #[cfg(target_os = "macos")]
                {
                    self.status_message = String::from("Hidden to the menu bar.");
                    return window::set_mode(id, window::Mode::Hidden);
                }
                #[cfg(not(target_os = "macos"))]
                return window::close(id);
            }
            Message::TrayEvent(command) => {
                use crate::app::tray::TrayCommand;
                match command {
                    // Clicking the menu bar icon toggles the compact mini window:
                    // open + focus it if hidden, close it if already showing.
                    TrayCommand::ShowWindow => {
                        if let Some(id) = self.mini_window_id.take() {
                            self.status_message = String::from("Mini window closed.");
                            return window::close(id);
                        }

                        let (id, open) = window::open(Self::mini_window_settings());
                        self.mini_window_id = Some(id);
                        self.status_message = String::from("Mini window opened.");
                        return Task::batch([
                            open.map(Message::WindowOpened),
                            window::gain_focus(id),
                        ]);
                    }
                }
            }
            Message::TrayInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Menu bar icon: {error}");
                }
            }
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
                    // Close window = hide into the menu bar.
                    return Some(window::set_mode(id, window::Mode::Hidden));
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
