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
                self.persist_settings();
                self.broadcast(IpcMessage::SetTheme(self.theme_mode));
            }
            Message::TitleChanged(value) => self.title_input = value,
            Message::ToggleTaskCompleted(index, completed) => {
                self.toggle_task_completed(index, completed);
                self.broadcast(IpcMessage::ToggleTaskCompleted { index, completed });
            }
            Message::AddTask => {
                if let Some((title, attachments)) = self.add_task() {
                    self.broadcast(IpcMessage::AddTask { title, attachments });
                }
            }
            Message::RemoveTask(index) => {
                self.remove_task(index);
                self.broadcast(IpcMessage::RemoveTask { index });
            }
            Message::AttachFile(target) => {
                return Self::launch_file_attach_dialog(target, self.modifiers.alt());
            }
            Message::FileChosen { target, copy, path } => {
                if let Some(path) = path {
                    match common::attachments::attachment_from_path(&path, copy) {
                        Ok(attachment) => self.attach_to_target(target, attachment),
                        Err(error) => self.status_message = error,
                    }
                } else {
                    self.status_message = String::from("Attachment cancelled.");
                }
            }
            Message::AttachScreenshot(target) => {
                return Task::perform(
                    async { common::attachments::capture_screenshot() },
                    move |result| Message::ScreenshotCaptured { target, result },
                );
            }
            Message::ScreenshotCaptured { target, result } => match result {
                Ok(attachment) => self.attach_to_target(target, attachment),
                Err(error) => self.status_message = error,
            },
            Message::RemoveTaskAttachment { task, attachment } => {
                self.remove_task_attachment(task, attachment);
            }
            Message::RemoveStagedAttachment(index) => {
                if index < self.staged_attachments.len() {
                    self.staged_attachments.remove(index);
                }
            }
            Message::OpenAttachment(path) => common::attachments::open_path(&path),
            Message::ClearCompleted => {
                self.clear_completed();
                self.resync_tray();
            }
            Message::RequestClearAll => {
                if self.confirm_clear_all {
                    self.confirming_clear_all = true;
                } else {
                    self.clear_all();
                    self.resync_tray();
                }
            }
            Message::ClearAll => {
                self.confirming_clear_all = false;
                self.clear_all();
                self.resync_tray();
            }
            Message::CancelClearAll => self.confirming_clear_all = false,
            Message::EditUndo => {
                self.undo();
                self.resync_tray();
            }
            Message::EditRedo => {
                self.redo();
                self.resync_tray();
            }
            // --- Settings ---
            Message::ToggleSettings => {
                self.show_settings = !self.show_settings;
                if !self.show_settings {
                    self.recording_hotkey = false;
                }
            }
            Message::CloseSettings => self.leave_settings(),
            Message::SetTheme(mode) => {
                self.theme_mode = mode;
                self.status_message = format!("Switched to {}.", mode.label());
                self.persist_settings();
                self.broadcast(IpcMessage::SetTheme(mode));
            }
            Message::SetReopenLastList(on) => {
                self.reopen_last_list = on;
                self.persist_settings();
            }
            Message::SetConfirmClearAll(on) => {
                self.confirm_clear_all = on;
                self.persist_settings();
            }
            Message::SetReduceMotion(on) => {
                self.reduce_motion = on;
                common::ui::motion::set_reduce_motion(on);
                self.persist_settings();
            }
            Message::SetHotkeyEnabled(on) => {
                self.hotkey_enabled = on;
                self.persist_settings();
                self.send_hotkey_config();
                self.status_message = if on {
                    String::from("Mini-window hotkey enabled.")
                } else {
                    String::from("Mini-window hotkey disabled.")
                };
            }
            Message::StartRecordHotkey => {
                self.recording_hotkey = true;
                self.status_message = String::from("Press the new shortcut… (Esc to cancel)");
            }
            Message::CancelRecordHotkey => {
                self.recording_hotkey = false;
                self.status_message = String::from("Shortcut unchanged.");
            }
            Message::ResetHotkey => {
                self.recording_hotkey = false;
                self.hotkey = common::hotkey::HotkeySpec::default_mini_toggle();
                self.persist_settings();
                self.send_hotkey_config();
                self.status_message = format!("Shortcut reset to {}.", self.hotkey.label());
            }
            // --- List management ---
            Message::ToggleListPanel => self.show_list_panel = !self.show_list_panel,
            Message::OpenList(name) => {
                self.leave_settings();
                self.open_list(&name);
                self.resync_tray();
            }
            Message::NewListNameChanged(value) => self.new_list_name = value,
            Message::CreateList => {
                self.leave_settings();
                self.create_list();
                self.resync_tray();
            }
            Message::DeleteList(name) => {
                self.delete_list(&name);
                self.resync_tray();
            }
            Message::StartRenameList(name) => {
                self.renaming = Some((name.clone(), name));
                // Open the rename modal with its input focused and the cursor at
                // the end of the existing name.
                use crate::app::ui::RENAME_INPUT_ID;
                return iced::widget::operation::focus(RENAME_INPUT_ID)
                    .chain(iced::widget::operation::move_cursor_to_end(RENAME_INPUT_ID));
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
            Message::DragWindow => {
                if let Some(id) = self.window_id {
                    return window::drag(id);
                }
            }
            Message::WindowOpened(window_id) => {
                self.window_id = Some(window_id);
                // Both run on the UI thread (window::run guarantees it): style the
                // window chrome (transparent title bar + frosted backdrop) and
                // install the native menu.
                let chrome = window::run(window_id, |window| {
                    crate::app::chrome::apply(window);
                })
                .discard();
                let menu = window::run(window_id, |window| {
                    crate::app::native_menu::install_for_window(window)
                })
                .map(Message::NativeMenuInstalled);
                return Task::batch([chrome, menu]);
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
            Message::WindowCloseRequested(_id) => {
                // Closing the main window quits the app entirely. The menu-bar
                // tray keeps running and relaunches us when the user clicks the
                // mini window's list title.
                return iced::exit();
            }
            Message::IpcEvent(event) => return self.handle_ipc(event),
            Message::KeyboardEvent(event) => {
                // Track modifier state so attach actions can read whether
                // Option/Alt is held (link vs. copy) at press time.
                if let keyboard::Event::ModifiersChanged(modifiers) = &event {
                    self.modifiers = *modifiers;
                    return Task::none();
                }
                // While capturing a new hotkey, the keyboard belongs to the
                // recorder — no app shortcuts fire.
                if self.recording_hotkey {
                    self.capture_hotkey(&event);
                    return Task::none();
                }
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
                            self.persist_settings();
                        }
                        Key::Named(iced::keyboard::key::Named::Escape)
                            if self.renaming.is_some() =>
                        {
                            self.renaming = None;
                        }
                        Key::Named(iced::keyboard::key::Named::Escape)
                            if self.confirming_clear_all =>
                        {
                            self.confirming_clear_all = false;
                        }
                        Key::Named(iced::keyboard::key::Named::Escape) if self.show_settings => {
                            self.leave_settings();
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
                // Quitting and closing the window both exit the app entirely.
                Key::Character("q") if modifiers.command() => {
                    return Some(iced::exit());
                }
                Key::Character("w") if modifiers.command() => {
                    return Some(iced::exit());
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
                Key::Character("w") if logo => return Some(iced::exit()),
                Key::Named(iced::keyboard::key::Named::F4) if modifiers.alt() => {
                    return Some(iced::exit());
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
                Key::Character("w") if logo => return Some(iced::exit()),
                Key::Named(iced::keyboard::key::Named::F4) if modifiers.alt() => {
                    return Some(iced::exit());
                }
                _ => {}
            }
        }

        let _ = (key, modifiers);
        None
    }

    /// Consumes a key event while the settings page is recording a new
    /// mini-window hotkey. Holds for modifier-only presses, rejects keys that
    /// can't be bound, and commits a valid combo (persisting + syncing to the
    /// tray). Esc cancels.
    fn capture_hotkey(&mut self, event: &keyboard::Event) {
        use iced::keyboard::key::{Named, Physical};
        use iced::keyboard::Key;

        let keyboard::Event::KeyPressed {
            physical_key,
            modifiers,
            key,
            ..
        } = event
        else {
            return;
        };

        // Esc on its own cancels recording.
        if matches!(key.as_ref(), Key::Named(Named::Escape))
            && !modifiers.alt()
            && !modifiers.control()
            && !modifiers.logo()
        {
            self.recording_hotkey = false;
            self.status_message = String::from("Shortcut unchanged.");
            return;
        }

        // The physical code maps directly to a W3C `code` name via its Debug form
        // (e.g. `Backquote`, `KeyK`), which is what we store and the tray parses.
        let code = match physical_key {
            Physical::Code(code) => *code,
            Physical::Unidentified(_) => return, // keep waiting
        };
        let code_name = format!("{code:?}");

        // Still only holding modifiers — wait for the actual key.
        if common::hotkey::is_modifier_code(&code_name) {
            return;
        }

        let spec = common::hotkey::HotkeySpec {
            alt: modifiers.alt(),
            ctrl: modifiers.control(),
            shift: modifiers.shift(),
            meta: modifiers.logo(),
            code: code_name,
        };

        if common::hotkey::key_code_label(&spec.code).is_none() {
            self.status_message = String::from("That key can't be used — try another.");
            return; // keep recording
        }
        if !spec.has_strong_modifier() {
            self.status_message = String::from("Hold ⌘, ⌥, or ⌃ with the key.");
            return; // keep recording
        }

        self.hotkey = spec;
        self.recording_hotkey = false;
        self.persist_settings();
        self.send_hotkey_config();
        self.status_message = format!("Shortcut set to {}.", self.hotkey.label());
    }
}
