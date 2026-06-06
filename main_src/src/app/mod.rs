//! The main-window application: state, message types, and the iced wiring.

mod actions;
mod launch;
mod native_menu;
mod queries;
mod snapshot;
mod sync;
mod update;
mod view;

use common::hotkey::HotkeySpec;
use common::storage::ListEntry;
use common::tasklist::TaskList;
use common::thememanager::{ThemeMode, app_theme};
use common::utils::fonts;
use iced::{Settings, Subscription, Theme, Size, daemon, keyboard, window};
use std::path::PathBuf;

pub use launch::ensure_tray_running;

pub type AppElement<'a> = iced::Element<'a, Message>;
pub type AppTask = iced::Task<Message>;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded,
    ToggleTheme,
    TitleChanged(String),
    ToggleTaskCompleted(usize, bool),
    RemoveTask(usize),
    AddTask,
    ClearCompleted,
    /// Entry point for "Clear all": either clears immediately or opens the
    /// confirmation modal, depending on the `confirm_clear_all` setting.
    RequestClearAll,
    /// Actually clear every task (the confirmed action).
    ClearAll,
    /// Dismiss the "Clear all" confirmation modal without clearing.
    CancelClearAll,
    EditUndo,
    EditRedo,
    // --- Settings ---
    /// Toggle the settings page (which replaces the task workspace).
    ToggleSettings,
    /// Leave the settings page, back to the tasks.
    CloseSettings,
    /// Choose the theme from the settings selector.
    SetTheme(ThemeMode),
    /// Toggle "reopen last list on launch".
    SetReopenLastList(bool),
    /// Toggle "confirm before Clear all".
    SetConfirmClearAll(bool),
    /// Enable/disable the mini-window global hotkey.
    SetHotkeyEnabled(bool),
    /// Begin live-capturing a new mini-window hotkey (the next key combo wins).
    StartRecordHotkey,
    /// Stop capturing without changing the hotkey.
    CancelRecordHotkey,
    /// Restore the built-in default mini-window hotkey.
    ResetHotkey,
    // --- Task-list management ---
    /// Show/hide the list sidebar.
    ToggleListPanel,
    /// Open an existing list by display name.
    OpenList(String),
    /// Text in the "new list" name input changed.
    NewListNameChanged(String),
    /// Create a new list from the name input.
    CreateList,
    /// Delete a list by display name.
    DeleteList(String),
    /// Begin renaming the list with this display name (reveals an inline input).
    StartRenameList(String),
    /// Text in the rename input changed.
    RenameInputChanged(String),
    /// Commit the rename of the list currently being renamed.
    CommitRenameList,
    /// Cancel an in-progress rename.
    CancelRenameList,
    /// Import a list from an external JSON file (opens a picker).
    ImportList,
    ImportListResult(Option<PathBuf>),
    /// Export the current list to an external JSON file (opens a picker).
    ExportList,
    ExportListResult(Option<PathBuf>),
    // --- Window / integration plumbing ---
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    WindowCloseRequested(window::Id),
    NativeMenuEvent(native_menu::NativeMenuCommand),
    NativeMenuInstalled(Result<(), String>),
    KeyboardEvent(keyboard::Event),
    /// A link event from the tray service (IPC client side).
    IpcEvent(common::ipc::IpcInbound),
}

#[derive(Debug, Clone)]
pub struct Taskscape {
    pub(crate) window_id: Option<window::Id>,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) title_input: String,
    pub(crate) status_message: String,
    pub(crate) undo_stack: Vec<snapshot::AppSnapshot>,
    pub(crate) redo_stack: Vec<snapshot::AppSnapshot>,
    pub(crate) tasks: TaskList,
    /// Display name of the currently open list, or `None` when none is open
    /// (the create/load empty-state prompt is shown in that case).
    pub(crate) current_list: Option<String>,
    /// Cached browser entries for the sidebar.
    pub(crate) lists: Vec<ListEntry>,
    /// Buffer for the "new list" name input (empty-state + sidebar "New").
    pub(crate) new_list_name: String,
    /// The list currently being renamed (its old display name) and the buffer
    /// holding the new name, while an inline rename is in progress.
    pub(crate) renaming: Option<(String, String)>,
    /// Whether the list sidebar is visible.
    pub(crate) show_list_panel: bool,
    /// Whether the settings page is showing in place of the task workspace.
    pub(crate) show_settings: bool,
    /// Whether we are live-capturing a new mini-window hotkey.
    pub(crate) recording_hotkey: bool,
    /// Whether the "Clear all" confirmation modal is open.
    pub(crate) confirming_clear_all: bool,
    /// Reopen the last-used list on launch (persisted setting).
    pub(crate) reopen_last_list: bool,
    /// Ask before the destructive "Clear all" (persisted setting).
    pub(crate) confirm_clear_all: bool,
    /// Whether the mini-window global hotkey is enabled (persisted setting).
    pub(crate) hotkey_enabled: bool,
    /// The mini-window global hotkey (persisted setting; registered by the tray).
    pub(crate) hotkey: HotkeySpec,
    /// Whether the tray service is currently linked over IPC.
    pub(crate) ipc_connected: bool,
    /// Set while applying a mutation received from the tray service, so the same
    /// change is not echoed straight back and looped.
    pub(crate) applying_remote: bool,
}

impl Default for Taskscape {
    fn default() -> Self {
        Self {
            window_id: None,
            theme_mode: ThemeMode::Dark,
            title_input: String::new(),
            status_message: String::from("Ready."),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            tasks: TaskList::new(),
            current_list: None,
            lists: Vec::new(),
            new_list_name: String::new(),
            renaming: None,
            show_list_panel: false,
            show_settings: false,
            recording_hotkey: false,
            confirming_clear_all: false,
            reopen_last_list: true,
            confirm_clear_all: true,
            hotkey_enabled: true,
            hotkey: HotkeySpec::default_mini_toggle(),
            ipc_connected: false,
            applying_remote: false,
        }
    }
}

impl Taskscape {
    fn main_window_settings() -> window::Settings {
        window::Settings {
            min_size: Some(Size::new(980.0, 680.0)),
            // Intercept the close request ourselves (see `WindowCloseRequested`)
            // so closing the window quits the app rather than leaving the daemon
            // running windowless.
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

        // Populate the sidebar and restore saved settings.
        let mut state = Self::default();
        state.lists = common::storage::list_all();

        let config = common::storage::load_config();
        if let Some(theme) = config.theme {
            state.theme_mode = theme;
        }
        state.reopen_last_list = config.reopen_last_list;
        state.confirm_clear_all = config.confirm_clear_all;
        state.hotkey_enabled = config.hotkey_enabled;
        if let Some(hotkey) = config.hotkey {
            state.hotkey = hotkey;
        }

        // Reopen the last-used list (if enabled and it still exists).
        if state.reopen_last_list {
            if let Some(last) = config.last_open {
                if state.lists.iter().any(|e| e.name == last) {
                    state.open_list_quiet(&last);
                }
            }
        }
        if state.current_list.is_none() {
            state.status_message = String::from("Create a new list or load one to begin.");
        }

        let (_id, open) = window::open(Self::main_window_settings());
        (state, iced::Task::batch([load_fonts, open.map(Message::WindowOpened)]))
    }

    fn title(&self, _window: window::Id) -> String {
        match &self.current_list {
            Some(name) => format!("Taskscape — {name}"),
            None => String::from("Taskscape"),
        }
    }

    fn theme(&self, _window: window::Id) -> Theme {
        app_theme(self.theme_mode)
    }

    fn view_window(&self, _window: window::Id) -> AppElement<'_> {
        self.view_root()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::listen().map(Message::KeyboardEvent),
            window::open_events().map(Message::WindowOpened),
            window::close_events().map(Message::WindowClosed),
            window::close_requests().map(Message::WindowCloseRequested),
            native_menu::subscription().map(Message::NativeMenuEvent),
            common::ipc::client::subscription().map(Message::IpcEvent),
        ])
    }
}

pub fn run() -> iced::Result {
    // A `daemon` (rather than `application`) lets the window be opened
    // programmatically and the process keep running across window events.
    daemon(Taskscape::boot, Taskscape::update, Taskscape::view_window)
        .title(Taskscape::title)
        .theme(Taskscape::theme)
        .subscription(Taskscape::subscription)
        .font(fonts::INTER_REGULAR_BYTES)
        .font(fonts::POPPINS_SEMIBOLD_BYTES)
        .font(lucide_icons::LUCIDE_FONT_BYTES)
        .default_font(fonts::inter_regular())
        .settings(Settings::default())
        .antialiasing(true)
        .run()
}
