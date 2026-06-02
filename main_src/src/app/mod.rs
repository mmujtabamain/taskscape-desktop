//! The main-window application: state, message types, and the iced wiring.

mod actions;
mod native_menu;
mod queries;
mod snapshot;
mod sync;
mod update;
mod view;

use common::tasklist::TaskList;
use common::thememanager::{ThemeMode, app_theme};
use common::utils::fonts;
use iced::{Settings, Subscription, Theme, Size, daemon, keyboard, window};
use std::path::PathBuf;

pub type AppElement<'a> = iced::Element<'a, Message>;
pub type AppTask = iced::Task<Message>;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded,
    ToggleTheme,
    TitleChanged(String),
    FileNameChanged(String),
    ToggleTitleEdit,
    CancelAllEditing,
    ToggleTaskCompleted(usize, bool),
    AddTask,
    ClearCompleted,
    ClearAll,
    FileNew,
    FileSave,
    FileLoad,
    FileSaveResult(Option<PathBuf>),
    FileLoadResult(Option<PathBuf>),
    EditUndo,
    EditRedo,
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
    pub(crate) file_name: String,
    pub(crate) file_name_editing: String,
    pub(crate) editing_title: bool,
    /// Whether the tray service is currently linked over IPC.
    pub(crate) ipc_connected: bool,
    /// Set while applying a mutation received from the tray service, so the same
    /// change is not echoed straight back and looped.
    pub(crate) applying_remote: bool,
}

impl Default for Taskscape {
    fn default() -> Self {
        const DEFAULT_FILE_NAME: &str = "Untitled";

        Self {
            window_id: None,
            theme_mode: ThemeMode::Dark,
            title_input: String::new(),
            status_message: String::from("Ready."),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            tasks: TaskList::new(),
            file_name: String::from(DEFAULT_FILE_NAME),
            file_name_editing: String::from(DEFAULT_FILE_NAME),
            editing_title: false,
            ipc_connected: false,
            applying_remote: false,
        }
    }
}

impl Taskscape {
    pub(crate) fn is_any_editing(&self) -> bool {
        self.editing_title
    }

    fn main_window_settings() -> window::Settings {
        window::Settings {
            min_size: Some(Size::new(980.0, 680.0)),
            // On macOS we intercept the close request to minimize to the Dock
            // instead of quitting; other platforms close = quit.
            exit_on_close_request: !cfg!(target_os = "macos"),
            ..window::Settings::default()
        }
    }

    fn boot() -> (Self, AppTask) {
        let load_fonts = iced::Task::batch([
            iced::font::load(fonts::INTER_REGULAR_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(fonts::POPPINS_SEMIBOLD_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(lucide_icons::LUCIDE_FONT_BYTES).map(|_| Message::FontLoaded),
        ]);
        let (_id, open) = window::open(Self::main_window_settings());
        (
            Self::default(),
            iced::Task::batch([load_fonts, open.map(Message::WindowOpened)]),
        )
    }

    fn title(&self, _window: window::Id) -> String {
        String::from("Taskscape")
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
