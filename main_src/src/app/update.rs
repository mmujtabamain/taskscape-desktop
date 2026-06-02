use crate::app::native_menu::NativeMenuCommand;
use crate::app::{AppTask, Message, Taskscape};
use common::ipc::IpcMessage;
use iced::{Task, keyboard, window};

impl Taskscape {
    pub(crate) fn update(&mut self, message: Message) -> AppTask {
        match message {
            Message::FontLoaded => {}
            Message::ToggleTheme => {
                self.theme_mode = self.theme_mode.toggled();
                self.status_message = format!("Switched to {}.", self.theme_mode.label());
            }
            Message::TitleChanged(value) => self.title_input = value,
            Message::FileNameChanged(value) => self.file_name_editing = value,
            Message::ToggleTitleEdit => {
                if self.editing_title {
                    self.file_name = self.file_name_editing.clone();
                }
                self.editing_title = !self.editing_title;
                if self.editing_title {
                    self.file_name_editing = self.file_name.clone();
                    return iced::widget::operation::move_cursor_to_end(
                        common::widgets::t_editable_title::TITLE_INPUT_ID,
                    );
                }
            }
            Message::CancelAllEditing => {
                if self.editing_title {
                    self.file_name_editing = self.file_name.clone();
                    self.editing_title = false;
                }
            }
            Message::ToggleTaskCompleted(index, completed) => {
                self.toggle_task_completed(index, completed);
                self.broadcast(IpcMessage::ToggleTaskCompleted { index, completed });
            }
            Message::AddTask => {
                if let Some(title) = self.add_task() {
                    self.broadcast(IpcMessage::AddTask { title });
                }
            }
            Message::ClearCompleted => {
                self.push_history();
                self.tasks.clear_completed();
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
                self.window_id = Some(window_id);
                // The native menu must be installed on the UI thread; window::run
                // guarantees that.
                return window::run(window_id, |window| {
                    crate::app::native_menu::install_for_window(window)
                })
                .map(Message::NativeMenuInstalled);
            }
            Message::WindowClosed(window_id) => {
                if self.window_id == Some(window_id) {
                    self.window_id = None;
                }
            }
            Message::NativeMenuEvent(command) => match command {
                NativeMenuCommand::FileNew => self.new_list("Started a new list."),
                NativeMenuCommand::FileSave => return self.launch_save_dialog(),
                NativeMenuCommand::FileLoad => return Self::launch_load_dialog(),
                NativeMenuCommand::EditUndo => self.undo(),
                NativeMenuCommand::EditRedo => self.redo(),
                NativeMenuCommand::ToggleTheme => {
                    self.theme_mode = self.theme_mode.toggled();
                    self.status_message = format!("Switched to {}.", self.theme_mode.label());
                }
            },
            Message::NativeMenuInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Native menu: {error}");
                }
            }
            Message::WindowCloseRequested(id) => {
                // macOS: minimize to the Dock instead of quitting, so clicking
                // the Dock icon restores it. Other platforms close normally.
                #[cfg(target_os = "macos")]
                {
                    self.status_message = String::from("Minimized to the Dock.");
                    return window::minimize(id, true);
                }
                #[cfg(not(target_os = "macos"))]
                return window::close(id);
            }
            Message::IpcEvent(event) => return self.handle_ipc(event),
            Message::KeyboardEvent(event) => {
                if let keyboard::Event::KeyPressed { key, modifiers, .. } = event {
                    use iced::keyboard::Key;

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

    /// Platform-specific window management shortcuts.
    fn platform_window_shortcuts(
        &self,
        key: &iced::keyboard::Key,
        modifiers: &keyboard::Modifiers,
    ) -> Option<AppTask> {
        use iced::keyboard::Key;

        let id = self.window_id?;

        #[cfg(target_os = "macos")]
        {
            match key.as_ref() {
                Key::Character("f") if modifiers.command() => {
                    return Some(window::toggle_maximize(id));
                }
                Key::Character("m") if modifiers.command() => {
                    return Some(window::minimize(id, true));
                }
                Key::Character("q") if modifiers.command() => {
                    return Some(window::close(id));
                }
                Key::Character("w") if modifiers.command() => {
                    return Some(window::minimize(id, true));
                }
                _ => {}
            }
        }

        #[cfg(target_os = "windows")]
        {
            let logo = modifiers.logo();
            match key.as_ref() {
                Key::Character("f") if logo => return Some(window::toggle_maximize(id)),
                Key::Character("m") if logo => return Some(window::minimize(id, true)),
                Key::Character("w") if logo => return Some(window::close(id)),
                Key::Named(iced::keyboard::key::Named::F4) if modifiers.alt() => {
                    return Some(window::close(id));
                }
                _ => {}
            }
        }

        #[cfg(target_os = "linux")]
        {
            let logo = modifiers.logo();
            match key.as_ref() {
                Key::Character("f") if logo => return Some(window::toggle_maximize(id)),
                Key::Character("m") if logo => return Some(window::minimize(id, true)),
                Key::Character("w") if logo => return Some(window::close(id)),
                Key::Named(iced::keyboard::key::Named::F4) if modifiers.alt() => {
                    return Some(window::close(id));
                }
                _ => {}
            }
        }

        let _ = (key, modifiers);
        None
    }
}
