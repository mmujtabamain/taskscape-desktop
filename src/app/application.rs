use crate::app::{AppTask, Message};
use crate::models::{Priority, Task};
use crate::thememanager::{ThemeMode, app_theme};
use crate::utils::fonts;
use iced::{Settings, Size, Subscription, Theme, application, keyboard, window};

#[derive(Debug, Clone)]
pub struct Taskscape {
    pub(crate) theme_mode: ThemeMode,
    pub(crate) title_input: String,
    pub(crate) due_date_input: String,
    pub(crate) composer_priority: Priority,
    pub(crate) status_message: String,
    pub(crate) undo_stack: Vec<crate::app::snapshot::AppSnapshot>,
    pub(crate) redo_stack: Vec<crate::app::snapshot::AppSnapshot>,
    pub(crate) tasks: Vec<Task>,
}

impl Default for Taskscape {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            title_input: String::new(),
            due_date_input: String::new(),
            composer_priority: Priority::Medium,
            status_message: String::from("Ready."),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            tasks: Vec::new(),
        }
    }
}

impl Taskscape {
    pub(crate) fn boot() -> (Self, AppTask) {
        // Load custom fonts via Task so they are guaranteed to be available
        // before the first view call, complementing the builder .font() calls.
        let load_fonts = iced::Task::batch([
            iced::font::load(fonts::INTER_REGULAR_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(fonts::POPPINS_SEMIBOLD_BYTES).map(|_| Message::FontLoaded),
            iced::font::load(lucide_icons::LUCIDE_FONT_BYTES).map(|_| Message::FontLoaded),
        ]);
        (Self::default(), load_fonts)
    }

    pub(crate) fn title(&self) -> String {
        String::from("Taskscape")
    }

    pub(crate) fn theme(&self) -> Theme {
        app_theme(self.theme_mode)
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::listen().map(Message::KeyboardEvent),
            window::open_events().map(Message::WindowOpened),
            crate::app::native_menu::subscription().map(Message::NativeMenuEvent),
        ])
    }
}

pub fn run() -> iced::Result {
    application(Taskscape::boot, Taskscape::update, Taskscape::view_root)
        .title(Taskscape::title)
        .theme(Taskscape::theme)
        .subscription(Taskscape::subscription)
        .window(window::Settings {
            min_size: Some(Size::new(980.0, 680.0)),
            ..window::Settings::default()
        })
        .font(fonts::INTER_REGULAR_BYTES)
        .font(fonts::POPPINS_SEMIBOLD_BYTES)
        .font(lucide_icons::LUCIDE_FONT_BYTES)
        .default_font(fonts::inter_regular())
        .settings(Settings::default())
        .antialiasing(true)
        .centered()
        .run()
}
