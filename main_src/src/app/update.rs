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
            Message::ToggleTaskCompleted(index, completed) => {
                self.toggle_task_completed(index, completed);
                self.broadcast(IpcMessage::ToggleTaskCompleted { index, completed });
            }
            Message::AddTask => {
                if let Some(title) = self.add_task() {
                    self.broadcast(IpcMessage::AddTask { title });
                }
            }
            Message::RemoveTask(index) => {
                self.remove_task(index);
                self.broadcast(IpcMessage::RemoveTask { index });
            }
            Message::ClearCompleted => {
                self.clear_completed();
                self.resync_tray();
            }
            Message::ClearAll => {
                self.clear_all();
                self.resync_tray();
            }
            Message::EditUndo => {
                self.undo();
                self.resync_tray();
            }
            Message::EditRedo => {
                self.redo();
                self.resync_tray();
            }
            // --- List management ---
            Message::ToggleListPanel => self.show_list_panel = !self.show_list_panel,
            Message::OpenList(name) => {
                self.open_list(&name);
                self.resync_tray();
            }
            Message::NewListNameChanged(value) => self.new_list_name = value,
            Message::CreateList => {
                self.create_list();
                self.resync_tray();
            }
            Message::DeleteList(name) => {
                self.delete_list(&name);
                self.resync_tray();
            }
            Message::StartRenameList(name) => {
                self.renaming = Some((name.clone(), name));
            }
            Message::RenameInputChanged(value) => {
                if let Some((_, new_name)) = self.renaming.as_mut() {
                    *new_name = value;
                }
            }
            Message::CommitRenameList => self.commit_rename(),
            Message::CancelRenameList => self.renaming = None,
            Message::ImportList => return Self::launch_import_dialog(),
            Message::ImportListResult(Some(path)) => {
                self.complete_import(path);
                self.resync_tray();
            }
            Message::ImportListResult(None) => {
                self.status_message = String::from("Import cancelled.");
            }
            Message::ExportList => return self.launch_export_dialog(),
            Message::ExportListResult(Some(path)) => self.complete_export(path),
            Message::ExportListResult(None) => {
                self.status_message = String::from("Export cancelled.");
            }
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
                // "New List" opens the sidebar so the user can name + create one.
                NativeMenuCommand::FileNew => self.show_list_panel = true,
                NativeMenuCommand::FileSave => return self.launch_export_dialog(),
                NativeMenuCommand::FileLoad => return Self::launch_import_dialog(),
                NativeMenuCommand::EditUndo => {
                    self.undo();
                    self.resync_tray();
                }
                NativeMenuCommand::EditRedo => {
                    self.redo();
                    self.resync_tray();
                }
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
                        Key::Character("z") if command && modifiers.shift() => {
                            self.redo();
                            self.resync_tray();
                        }
                        Key::Character("z") if command => {
                            self.undo();
                            self.resync_tray();
                        }
                        Key::Character("e") if command => return self.launch_export_dialog(),
                        Key::Character("o") if command => return Self::launch_import_dialog(),
                        Key::Character("n") if command => self.show_list_panel = true,
                        Key::Character("l") if command => {
                            self.show_list_panel = !self.show_list_panel;
                        }
                        Key::Character("t") if command => {
                            self.theme_mode = self.theme_mode.toggled();
                            self.status_message =
                                format!("Switched to {}.", self.theme_mode.label());
                        }
                        Key::Named(iced::keyboard::key::Named::Escape)
                            if self.renaming.is_some() =>
                        {
                            self.renaming = None;
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
