use crate::app::{AppTask, Message};
use crate::models::Task;
use crate::thememanager::{ThemeMode, app_theme};
use crate::utils::fonts;
use iced::{Settings, Size, Subscription, Theme, daemon, keyboard, window};

/// Which of the two processes this instance is. Both run the same `Taskscape`
/// state and `update`/`view` code; the role decides which window opens, which
/// installers run, and which side of the IPC link is used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppRole {
    /// The main-window app: source of truth, IPC client.
    Main,
    /// The background tray service: tray icon + mini window + hotkey, IPC server.
    Tray,
}

#[derive(Debug, Clone)]
pub struct Taskscape {
    /// Which process this is (see [`AppRole`]).
    pub(crate) role: AppRole,
    pub(crate) window_id: Option<window::Id>,
    /// The compact "mini" window toggled from the menu bar icon. Owned by the
    /// tray service; `None` when it is not open (and always `None` for `Main`).
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
    /// Whether the peer process is currently linked over IPC.
    pub(crate) ipc_connected: bool,
    /// Set while applying a mutation received from the peer, so the same change
    /// is not echoed straight back and looped.
    pub(crate) applying_remote: bool,
}

impl Taskscape {
    fn new(role: AppRole) -> Self {
        const DEFAULT_FILE_NAME: &'static str = "Untitled";

        Self {
            role,
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
            ipc_connected: false,
            applying_remote: false,
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
            // Transparent so the rounded corners drawn by `mini_shell_container`
            // show through instead of sitting on an opaque rectangle.
            transparent: true,
            level: window::Level::AlwaysOnTop,
            position: window::Position::Centered,
            exit_on_close_request: false,
            ..window::Settings::default()
        }
    }

    /// A hidden, throwaway window the **tray service** opens at boot purely to
    /// get a main-thread context for installing the tray icon and global hotkey
    /// (both must run on the UI thread). It is closed immediately afterwards; a
    /// `daemon` keeps running with zero windows.
    fn bootstrap_window_settings() -> window::Settings {
        window::Settings {
            size: Size::new(1.0, 1.0),
            visible: false,
            decorations: false,
            exit_on_close_request: false,
            ..window::Settings::default()
        }
    }

    pub(crate) fn boot(role: AppRole) -> (Self, AppTask) {
        // Load custom fonts via Task so they are guaranteed to be available
        // before the first view call, complementing the builder .font() calls.
        let load_fonts = iced::Task::batch([
            iced::font::load(fonts::INTER_REGULAR_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(fonts::POPPINS_SEMIBOLD_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(lucide_icons::LUCIDE_FONT_BYTES).map(|_| Message::FontLoaded),
        ]);

        // A `daemon` opens no window on its own. The main app opens its window
        // here; the tray service opens a hidden bootstrap window (see above) and
        // otherwise waits for tray/hotkey interactions to open the mini window.
        let open = match role {
            AppRole::Main => {
                let (_id, task) = window::open(Self::main_window_settings());
                task.map(Message::WindowOpened)
            }
            AppRole::Tray => {
                let (_id, task) = window::open(Self::bootstrap_window_settings());
                task.map(Message::WindowOpened)
            }
        };

        (Self::new(role), iced::Task::batch([load_fonts, open]))
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
    /// The tray service only ever draws the mini window; the main app the root.
    pub(crate) fn view_window(&self, window: window::Id) -> crate::app::AppElement<'_> {
        if self.mini_window_id == Some(window) {
            self.mini_view()
        } else {
            self.view_root()
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        // Window/keyboard plumbing is shared; the role decides which integrations
        // and which side of the IPC link are wired up.
        let mut subs = vec![
            keyboard::listen().map(Message::KeyboardEvent),
            window::open_events().map(Message::WindowOpened),
            window::close_events().map(Message::WindowClosed),
            window::close_requests().map(Message::WindowCloseRequested),
        ];

        match self.role {
            AppRole::Main => {
                subs.push(crate::app::native_menu::subscription().map(Message::NativeMenuEvent));
                subs.push(crate::ipc::client::subscription().map(Message::IpcEvent));
            }
            AppRole::Tray => {
                subs.push(crate::app::tray::subscription().map(Message::TrayEvent));
                subs.push(crate::app::hotkey::subscription().map(Message::HotkeyEvent));
                subs.push(crate::ipc::server::subscription().map(Message::IpcEvent));
            }
        }

        Subscription::batch(subs)
    }
}

/// Runs one of the two apps as an `iced` daemon. Both processes share this crate
/// and the `Taskscape` state/update/view code; `role` selects the behaviour.
pub fn run(role: AppRole) -> iced::Result {
    daemon(
        move || Taskscape::boot(role),
        Taskscape::update,
        Taskscape::view_window,
    )
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
