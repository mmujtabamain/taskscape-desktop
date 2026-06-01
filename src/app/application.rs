use crate::app::{AppTask, Message};
use crate::models::Task;
use crate::thememanager::{ThemeMode, app_theme};
use crate::utils::fonts;
use iced::{Settings, Size, Subscription, Theme, daemon, keyboard, window};

#[derive(Debug, Clone)]
pub struct Taskscape {
    pub(crate) window_id: Option<window::Id>,
    /// The compact "mini" window toggled from the menu bar icon. Shares the
    /// same `tasks` as the main window; `None` when it is not open.
    pub(crate) mini_window_id: Option<window::Id>,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) title_input: String,
    pub(crate) due_date_input: String,
    pub(crate) status_message: String,
    pub(crate) undo_stack: Vec<crate::app::snapshot::AppSnapshot>,
    pub(crate) redo_stack: Vec<crate::app::snapshot::AppSnapshot>,
    pub(crate) tasks: Vec<Task>,
    pub(crate) file_name: String,
    pub(crate) file_name_editing: String,
    pub(crate) editing_title: bool,
}

impl Default for Taskscape {
    fn default() -> Self {
        const DEFAULT_FILE_NAME: &'static str = "Untitled";

        Self {
            window_id: None,
            mini_window_id: None,
            theme_mode: ThemeMode::Dark,
            title_input: String::new(),
            due_date_input: String::new(),
            status_message: String::from("Ready."),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            tasks: Vec::new(),
            file_name: String::from(DEFAULT_FILE_NAME),
            file_name_editing: String::from(DEFAULT_FILE_NAME),
            editing_title: false,
        }
    }
}

impl Taskscape {
    /// Check if any editing mode is active in the app
    pub(crate) fn is_any_editing(&self) -> bool {
        self.editing_title
    }

    /// The settings used for the main window. Mirrors the previous single-window
    /// configuration so behaviour is unchanged for the primary window.
    pub(crate) fn main_window_settings() -> window::Settings {
        window::Settings {
            min_size: Some(Size::new(980.0, 680.0)),
            // On macOS we intercept the close request to hide the window into the
            // menu bar instead of quitting. Other platforms keep the default
            // (close = quit) until their tray support lands.
            exit_on_close_request: !cfg!(target_os = "macos"),
            ..window::Settings::default()
        }
    }

    /// The settings used for the compact "mini" window: no title bar, fixed
    /// small size, floating above other windows.
    pub(crate) fn mini_window_settings() -> window::Settings {
        let size = Size::new(360.0, 480.0);
        window::Settings {
            size,
            min_size: Some(size),
            max_size: Some(Size::new(360.0, 900.0)),
            decorations: false,
            resizable: false,
            level: window::Level::AlwaysOnTop,
            position: window::Position::Centered,
            exit_on_close_request: false,
            ..window::Settings::default()
        }
    }

    pub(crate) fn boot() -> (Self, AppTask) {
        // Load custom fonts via Task so they are guaranteed to be available
        // before the first view call, complementing the builder .font() calls.
        let load_fonts = iced::Task::batch([
            iced::font::load(fonts::INTER_REGULAR_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(fonts::POPPINS_SEMIBOLD_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(lucide_icons::LUCIDE_FONT_BYTES).map(|_| Message::FontLoaded),
        ]);
        // Unlike `application`, a `daemon` opens no window on its own — we open
        // the main window here and discard the returned id (it arrives again via
        // the `WindowOpened` subscription, where installers are wired up).
        let (_id, open_main) = window::open(Self::main_window_settings());
        (
            Self::default(),
            iced::Task::batch([load_fonts, open_main.map(Message::WindowOpened)]),
        )
    }

    pub(crate) fn title(&self, window: window::Id) -> String {
        if self.mini_window_id == Some(window) {
            String::from("Taskscape Mini")
        } else {
            String::from("Taskscape")
        }
    }

    pub(crate) fn theme(&self, _window: window::Id) -> Theme {
        app_theme(self.theme_mode)
    }

    /// Dispatches to the mini or full view depending on which window is drawing.
    pub(crate) fn view_window(&self, window: window::Id) -> crate::app::AppElement<'_> {
        if self.mini_window_id == Some(window) {
            self.mini_view()
        } else {
            self.view_root()
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::listen().map(Message::KeyboardEvent),
            window::open_events().map(Message::WindowOpened),
            window::close_events().map(Message::WindowClosed),
            window::close_requests().map(Message::WindowCloseRequested),
            crate::app::native_menu::subscription().map(Message::NativeMenuEvent),
            crate::app::tray::subscription().map(Message::TrayEvent),
            crate::app::hotkey::subscription().map(Message::HotkeyEvent),
        ])
    }
}

pub fn run() -> iced::Result {
    // A `daemon` (rather than `application`) lets us drive more than one window:
    // the full main window plus the compact "mini" window toggled from the menu
    // bar icon. Both windows share the same `Taskscape` state, so editing tasks
    // in one is immediately reflected in the other. Windows are opened
    // programmatically (see `boot` / the tray handler) instead of via a builder.
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
