//! The tray-service application: state, message types, and the iced wiring.

mod hotkey;
mod launch;
mod mini;
mod sync;
mod tray;
mod update;

use common::models::Attachment;
use common::tasklist::TaskList;
use common::thememanager::{ThemeMode, app_theme};
use common::utils::fonts;
use iced::{Settings, Size, Subscription, Theme, daemon, keyboard, window};
use std::path::PathBuf;

pub type AppElement<'a> = iced::Element<'a, Message>;
pub type AppTask = iced::Task<Message>;

/// What an attach action targets: an existing task by index, or the composer
/// (staged onto the next task created with Enter).
#[derive(Debug, Clone, Copy)]
pub enum AttachTarget {
    Task(usize),
    Composer,
}

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded,
    TitleChanged(String),
    ToggleTaskCompleted(usize, bool),
    RemoveTask(usize),
    AddTask,
    // --- Attachments ---
    /// Open the file picker to attach a file to the given target. Holding
    /// Option/Alt while pressing copies into Taskscape instead of linking.
    AttachFile(AttachTarget),
    /// The file picker returned (or was cancelled). `copy` is captured from the
    /// modifier state at press time.
    FileChosen {
        target: AttachTarget,
        copy: bool,
        path: Option<PathBuf>,
    },
    /// Capture a full-screen screenshot and attach it to the given target.
    AttachScreenshot(AttachTarget),
    /// The screenshot capture finished.
    ScreenshotCaptured {
        target: AttachTarget,
        result: Result<Attachment, String>,
    },
    /// Remove the attachment at `attachment` from the task at `task`.
    RemoveTaskAttachment { task: usize, attachment: usize },
    /// Remove the composer-staged attachment at this index.
    RemoveStagedAttachment(usize),
    /// Open an attachment in the OS default app.
    OpenAttachment(PathBuf),
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    WindowCloseRequested(window::Id),
    /// Any window event (used to dismiss the quit popover when it loses focus).
    WindowEvent(window::Id, window::Event),
    TrayEvent(tray::TrayCommand),
    TrayInstalled(Result<(), String>),
    HotkeyEvent(hotkey::HotkeyCommand),
    HotkeyInstalled(Result<(), String>),
    KeyboardEvent(keyboard::Event),
    /// Quit requested (power button or tray menu) — opens the confirm popover.
    QuitRequested,
    /// Confirm quitting — exits the process.
    ConfirmQuit,
    /// Dismiss the quit confirmation popover.
    CancelQuit,
    /// Start dragging the quit popover window (it is borderless, so it has no
    /// title bar to drag).
    DragConfirm,
    /// Start dragging the mini window (also borderless — its header doubles as
    /// the drag handle).
    DragMini,
    /// Clicking the list title: bring the main app forward + open its sidebar
    /// (launching it first if it is closed).
    ShowMainRequested,
    /// A link event from the main app (IPC server side).
    IpcEvent(common::ipc::IpcInbound),
}

#[derive(Debug, Clone)]
pub struct TrayApp {
    /// The hidden window opened at boot purely to install the tray icon and
    /// hotkey on the UI thread; closed immediately afterwards.
    pub(crate) bootstrap_window_id: Option<window::Id>,
    /// The compact mini window, when open.
    pub(crate) mini_window_id: Option<window::Id>,
    /// Whether the mini window has gained focus yet. Like `confirm_focused`, we
    /// only auto-close it on *losing* focus after it has been focused, so the
    /// transient unfocus while it opens doesn't close it instantly.
    pub(crate) mini_focused: bool,
    /// Set while a native file-attach dialog is open. The dialog steals focus
    /// from the mini window, which would otherwise auto-close on blur and yank
    /// the picker's parent away — so we suppress the blur-close while it is set.
    pub(crate) attaching: bool,
    /// The small "Quit Taskscape?" confirmation popover window, when open.
    pub(crate) confirm_window_id: Option<window::Id>,
    /// Whether the confirm popover has gained focus yet — we only auto-close it on
    /// *losing* focus after it has been focused, to avoid the transient unfocus
    /// during open from closing it instantly.
    pub(crate) confirm_focused: bool,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) title_input: String,
    /// Attachments staged in the composer, applied to the next task added.
    pub(crate) staged_attachments: Vec<Attachment>,
    /// Latest keyboard modifier state, so attach actions can read whether
    /// Option/Alt was held at press time (link vs. copy).
    pub(crate) modifiers: keyboard::Modifiers,
    pub(crate) status_message: String,
    pub(crate) tasks: TaskList,
    /// Display name of the open list (mirrored from the main app via `Hello`);
    /// `None` when none is open.
    pub(crate) current_list: Option<String>,
    /// Whether the main app is currently linked over IPC.
    pub(crate) ipc_connected: bool,
    /// Set while applying a mutation received from the main app, so the same
    /// change is not echoed straight back and looped.
    pub(crate) applying_remote: bool,
    /// Set when the user asked to show the main app but it was not linked: we
    /// launch it and send `ShowMain` once it connects.
    pub(crate) pending_show_main: bool,
}

impl Default for TrayApp {
    fn default() -> Self {
        Self {
            bootstrap_window_id: None,
            mini_window_id: None,
            mini_focused: false,
            attaching: false,
            confirm_window_id: None,
            confirm_focused: false,
            theme_mode: ThemeMode::Dark,
            title_input: String::new(),
            staged_attachments: Vec::new(),
            modifiers: keyboard::Modifiers::default(),
            status_message: String::from("Ready."),
            tasks: TaskList::new(),
            current_list: None,
            ipc_connected: false,
            applying_remote: false,
            pending_show_main: false,
        }
    }
}

impl TrayApp {
    /// The settings used for the compact mini window: no title bar, fixed small
    /// size, transparent (for rounded corners), floating above other windows.
    pub(crate) fn mini_window_settings() -> window::Settings {
        let size = Size::new(360.0, 480.0);
        window::Settings {
            size,
            min_size: Some(size),
            max_size: Some(Size::new(360.0, 900.0)),
            decorations: false,
            resizable: false,
            transparent: true,
            level: window::Level::AlwaysOnTop,
            position: window::Position::Centered,
            exit_on_close_request: false,
            ..window::Settings::default()
        }
    }

    /// Settings for the small "Quit Taskscape?" confirmation popover window.
    pub(crate) fn confirm_window_settings() -> window::Settings {
        let size = Size::new(300.0, 150.0);
        window::Settings {
            size,
            min_size: Some(size),
            max_size: Some(size),
            decorations: false,
            resizable: false,
            transparent: true,
            level: window::Level::AlwaysOnTop,
            position: window::Position::Centered,
            exit_on_close_request: false,
            ..window::Settings::default()
        }
    }

    /// A hidden, throwaway window opened at boot to get a main-thread context for
    /// installing the tray icon and global hotkey. Closed immediately afterwards;
    /// a `daemon` keeps running with zero windows.
    fn bootstrap_window_settings() -> window::Settings {
        window::Settings {
            size: Size::new(1.0, 1.0),
            visible: false,
            decorations: false,
            exit_on_close_request: false,
            ..window::Settings::default()
        }
    }

    fn boot() -> (Self, AppTask) {
        let load_fonts = iced::Task::batch([
            iced::font::load(fonts::INTER_REGULAR_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(fonts::POPPINS_SEMIBOLD_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(lucide_icons::LUCIDE_FONT_BYTES).map(|_| Message::FontLoaded),
        ]);
        // No visible window at startup: open the hidden bootstrap window so the
        // tray/hotkey installers can run on the UI thread (see update.rs).
        let (_id, open) = window::open(Self::bootstrap_window_settings());

        // Match the theme the main app saved, and adopt the last-open list, so the
        // mini window agrees on launch and — crucially — has a list to edit and
        // persist to while the main app is closed. When the main app is linked it
        // re-sends this as the source of truth via `Hello`.
        let mut state = Self::default();
        let config = common::storage::load_config();
        if let Some(theme) = config.theme {
            state.theme_mode = theme;
        }
        if config.reopen_last_list
            && let Some(last) = config.last_open
            && let Ok(file) = common::storage::load(&last)
        {
            state.current_list = Some(file.name);
            state.tasks.replace(file.tasks);
        }
        (
            state,
            iced::Task::batch([load_fonts, open.map(Message::WindowOpened)]),
        )
    }

    fn title(&self, _window: window::Id) -> String {
        String::from("Taskscape Mini")
    }

    fn theme(&self, _window: window::Id) -> Theme {
        app_theme(self.theme_mode)
    }

    fn view_window(&self, window: window::Id) -> AppElement<'_> {
        if self.confirm_window_id == Some(window) {
            self.quit_confirm_view()
        } else {
            self.mini_view()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::listen().map(Message::KeyboardEvent),
            window::open_events().map(Message::WindowOpened),
            window::close_events().map(Message::WindowClosed),
            window::close_requests().map(Message::WindowCloseRequested),
            window::events().map(|(id, event)| Message::WindowEvent(id, event)),
            tray::subscription().map(Message::TrayEvent),
            hotkey::subscription().map(Message::HotkeyEvent),
            common::ipc::server::subscription().map(Message::IpcEvent),
        ])
    }
}

pub fn run() -> iced::Result {
    daemon(TrayApp::boot, TrayApp::update, TrayApp::view_window)
        .title(TrayApp::title)
        .theme(TrayApp::theme)
        .subscription(TrayApp::subscription)
        .font(fonts::INTER_REGULAR_BYTES)
        .font(fonts::POPPINS_SEMIBOLD_BYTES)
        .font(lucide_icons::LUCIDE_FONT_BYTES)
        .default_font(fonts::inter_regular())
        .settings(Settings::default())
        .antialiasing(true)
        .run()
}
