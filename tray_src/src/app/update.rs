use crate::app::tray::TrayCommand;
use crate::app::{AppTask, AttachTarget, Message, TrayApp};
use common::ipc::IpcMessage;
use common::models::Attachment;
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

/// Where to place the mini window when opening it.
#[derive(Debug, Clone, Copy)]
enum MiniSpawn {
    /// Anchored beneath the menu-bar icon (opened from the tray).
    Tray(TrayAnchor),
    /// At the current mouse cursor (summoned by the global hotkey).
    Mouse,
}

impl TrayApp {
    pub(crate) fn update(&mut self, message: Message) -> AppTask {
        match message {
            Message::FontLoaded => {}
            Message::TitleChanged(value) => self.title_input = value,
            Message::AddTask => {
                let title = self.title_input.trim().to_owned();
                if !title.is_empty() {
                    let attachments = std::mem::take(&mut self.staged_attachments);
                    self.tasks
                        .add_with_attachments(title.clone(), attachments.clone());
                    self.title_input.clear();
                    self.status_message = String::from("Task added.");
                    self.broadcast(IpcMessage::AddTask { title, attachments });
                    self.persist_local();
                }
            }
            Message::RemoveTask(index) => {
                if self.tasks.remove(index) {
                    self.status_message = String::from("Task removed.");
                    self.broadcast(IpcMessage::RemoveTask { index });
                    self.persist_local();
                }
            }
            Message::AttachFile(target) => {
                self.attaching = true;
                return Self::launch_file_attach_dialog(target, self.modifiers.alt());
            }
            Message::FileChosen { target, copy, path } => {
                self.attaching = false;
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
                if self.tasks.remove_attachment(task, attachment) {
                    self.status_message = String::from("Attachment removed.");
                    self.broadcast(IpcMessage::RemoveAttachment {
                        index: task,
                        attachment_index: attachment,
                    });
                    self.persist_local();
                }
            }
            Message::RemoveStagedAttachment(index) => {
                if index < self.staged_attachments.len() {
                    self.staged_attachments.remove(index);
                }
            }
            Message::OpenAttachment(path) => common::attachments::open_path(&path),
            Message::ToggleTaskCompleted(index, completed) => {
                if self.tasks.set_completed(index, completed) {
                    self.status_message = if completed {
                        String::from("Task marked complete.")
                    } else {
                        String::from("Task marked open.")
                    };
                    self.broadcast(IpcMessage::ToggleTaskCompleted { index, completed });
                    self.persist_local();
                }
            }
            Message::WindowOpened(window_id) => {
                // The mini window and the confirm popover are both transparent +
                // rounded: clip their content layer to a rounded rect (matching
                // the shell's 16px radius) so the corners aren't square. Must run
                // on the UI thread; window::run guarantees that.
                if self.mini_window_id == Some(window_id) {
                    // Round the corners, pin it over every Space (so it shows over
                    // full-screen apps, not just the desktop), pull it to the
                    // foreground and make it key — as a background (accessory) app
                    // the tray must activate itself or the window never accepts
                    // keyboard input — then put the cursor in the task input.
                    let prepare = window::run(window_id, |window| {
                        crate::app::tray::round_window(window, crate::app::ui::MINI_RADIUS as f64);
                        crate::app::tray::frost_window(window, crate::app::ui::MINI_RADIUS as f64);
                        crate::app::tray::pin_over_spaces(window);
                        crate::app::tray::focus_window(window);
                    })
                    .discard();
                    let focus_input =
                        iced::widget::operation::focus(crate::app::ui::MINI_INPUT_ID);
                    return Task::batch([prepare, focus_input]);
                }
                if self.confirm_window_id == Some(window_id) {
                    return window::run(window_id, |window| {
                        crate::app::tray::round_window(window, crate::app::ui::MINI_RADIUS as f64);
                        crate::app::tray::frost_window(window, crate::app::ui::MINI_RADIUS as f64);
                        crate::app::tray::pin_over_spaces(window);
                    })
                    .discard();
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
                    self.mini_focused = false;
                } else if self.confirm_window_id == Some(window_id) {
                    self.confirm_window_id = None;
                    self.confirm_focused = false;
                } else if self.bootstrap_window_id == Some(window_id) {
                    self.bootstrap_window_id = None;
                }
            }
            Message::WindowCloseRequested(id) => {
                // The mini window and the confirm popover just close.
                if self.mini_window_id == Some(id) {
                    self.mini_window_id = None;
                    return window::close(id);
                }
                if self.confirm_window_id == Some(id) {
                    self.confirm_window_id = None;
                    return window::close(id);
                }
            }
            Message::WindowEvent(id, event) => {
                if self.mini_window_id == Some(id) {
                    match event {
                        // Mark focused once it actually becomes key.
                        window::Event::Focused => self.mini_focused = true,
                        // Dismiss like a native popover when focus leaves — but
                        // only after it has been focused, so the transient unfocus
                        // during open doesn't close it instantly, and not while a
                        // file-attach dialog has taken focus.
                        window::Event::Unfocused if self.mini_focused && !self.attaching => {
                            self.mini_window_id = None;
                            self.mini_focused = false;
                            self.status_message = String::from("Mini window closed.");
                            return window::close(id);
                        }
                        _ => {}
                    }
                } else if self.confirm_window_id == Some(id) {
                    match event {
                        // Mark focused once it actually becomes key.
                        window::Event::Focused => self.confirm_focused = true,
                        // Dismiss like a native popover — but only after it has
                        // been focused, so the transient unfocus during open
                        // doesn't close it instantly.
                        window::Event::Unfocused if self.confirm_focused => {
                            self.confirm_window_id = None;
                            self.confirm_focused = false;
                            return window::close(id);
                        }
                        _ => {}
                    }
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
                    return self.toggle_mini_window(Some(MiniSpawn::Tray(anchor)));
                }
                TrayCommand::Quit => return self.open_quit_confirm(),
            },
            Message::QuitRequested => return self.open_quit_confirm(),
            Message::ConfirmQuit => return self.quit(),
            Message::CancelQuit => {
                self.confirm_focused = false;
                if let Some(id) = self.confirm_window_id.take() {
                    return window::close(id);
                }
            }
            Message::DragConfirm => {
                if let Some(id) = self.confirm_window_id {
                    return window::drag(id);
                }
            }
            Message::DragMini => {
                if let Some(id) = self.mini_window_id {
                    return window::drag(id);
                }
            }
            Message::ShowMainRequested => {
                if self.ipc_connected {
                    // Main app is running: ask it to come forward + open sidebar.
                    common::ipc::server::send(&common::ipc::IpcMessage::ShowMain);
                } else {
                    // Main app is closed: launch it; send ShowMain once it links.
                    self.pending_show_main = true;
                    crate::app::launch::launch_main();
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
                    HotkeyCommand::ToggleMini => {
                        return self.toggle_mini_window(Some(MiniSpawn::Mouse));
                    }
                }
            }
            Message::HotkeyInstalled(result) => {
                if let Err(error) = result {
                    self.status_message = format!("Global hotkey: {error}");
                }
            }
            Message::IpcEvent(event) => return self.handle_ipc(event),
            Message::KeyboardEvent(event) => {
                // Track modifier state so attach actions can read whether
                // Option/Alt is held (link vs. copy) at press time.
                if let keyboard::Event::ModifiersChanged(modifiers) = &event {
                    self.modifiers = *modifiers;
                    return Task::none();
                }
                if let keyboard::Event::KeyPressed { key, .. } = event {
                    use iced::keyboard::Key;
                    if matches!(key.as_ref(), Key::Named(iced::keyboard::key::Named::Escape)) {
                        // Esc dismisses the quit popover, else closes the mini window.
                        if let Some(id) = self.confirm_window_id.take() {
                            return window::close(id);
                        }
                        if let Some(id) = self.mini_window_id.take() {
                            return window::close(id);
                        }
                    }
                }
            }
        }

        Task::none()
    }

    /// Attaches a file/screenshot to the given target. For a task it mirrors the
    /// change over IPC and persists; for the composer it just stages it.
    fn attach_to_target(&mut self, target: AttachTarget, attachment: Attachment) {
        match target {
            AttachTarget::Composer => {
                self.staged_attachments.push(attachment);
                self.status_message = String::from("Attachment staged.");
            }
            AttachTarget::Task(index) => {
                if self.tasks.add_attachment(index, attachment.clone()) {
                    self.status_message = String::from("Attachment added.");
                    self.broadcast(IpcMessage::AddAttachment { index, attachment });
                    self.persist_local();
                }
            }
        }
    }

    /// Opens the native file picker to attach a file to `target`. `copy`
    /// (captured from the Option/Alt modifier at press time) forces a copy into
    /// Taskscape rather than linking to the original.
    fn launch_file_attach_dialog(target: AttachTarget, copy: bool) -> AppTask {
        Task::perform(
            async {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Attach File")
                    .pick_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            move |path| Message::FileChosen { target, copy, path },
        )
    }

    /// Opens the quit-confirmation popover window (or focuses it if already up).
    fn open_quit_confirm(&mut self) -> AppTask {
        if let Some(id) = self.confirm_window_id {
            return window::gain_focus(id);
        }
        let (id, open) = window::open(Self::confirm_window_settings());
        self.confirm_window_id = Some(id);
        self.confirm_focused = false;
        Task::batch([open.map(Message::WindowOpened), window::gain_focus(id)])
    }

    /// Quits Taskscape. Asks the main app to exit too (so quitting takes the whole
    /// app down), then exits this process.
    fn quit(&mut self) -> AppTask {
        if self.ipc_connected {
            common::ipc::server::send(&IpcMessage::Shutdown);
        }
        iced::exit()
    }

    /// Toggles the compact mini window: open + focus it if hidden, close it if
    /// already showing. Shared by the menu bar icon and the keyboard shortcut.
    fn toggle_mini_window(&mut self, spawn: Option<MiniSpawn>) -> AppTask {
        if let Some(id) = self.mini_window_id.take() {
            self.mini_focused = false;
            self.status_message = String::from("Mini window closed.");
            return window::close(id);
        }

        let mut settings = Self::mini_window_settings();
        match spawn {
            // Opened from the tray: anchor it beneath the menu-bar icon.
            Some(MiniSpawn::Tray(anchor)) => {
                settings.position = Self::mini_window_position(&anchor, settings.size);
            }
            // Summoned by the hotkey: drop it at the mouse cursor.
            Some(MiniSpawn::Mouse) => {
                if let Some(position) = Self::mouse_window_position() {
                    settings.position = position;
                }
            }
            None => {}
        }

        let (id, open) = window::open(settings);
        self.mini_window_id = Some(id);
        self.mini_focused = false;
        self.status_message = String::from("Mini window opened.");
        // Deliberately no `gain_focus` here: it activates the app, and doing that
        // *before* `WindowOpened` marks the window `canJoinAllSpaces` makes macOS
        // switch Spaces away from a full-screen app (so the window never lands on
        // it). Activation happens in `focus_window`, after `pin_over_spaces`.
        open.map(Message::WindowOpened)
    }

    /// Computes a window position (in logical points) that places the mini
    /// window's top-left corner at the current mouse cursor — used when the
    /// window is summoned by the global hotkey. Returns `None` if the cursor
    /// location can't be read, leaving the default position.
    fn mouse_window_position() -> Option<window::Position> {
        let (mouse_x, mouse_y) = crate::app::tray::mouse_position_top_left()?;
        Some(window::Position::Specific(iced::Point::new(
            mouse_x as f32,
            mouse_y as f32,
        )))
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
