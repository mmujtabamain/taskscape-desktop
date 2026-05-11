use crate::app::Message;
use crate::models::{
    CompletionFilter, NavSection, Priority, PriorityFilter, QuickDate, SortMode,
    StatusFilter, Task,
};
use crate::thememanager::{ThemeMode, app_theme};
use iced::{Application, Command, Settings, Theme, executor};

pub struct Taskscape {
    pub(crate) nav: NavSection,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) show_filters: bool,
    pub(crate) title_input: String,
    pub(crate) due_date_input: String,
    pub(crate) tags_input: String,
    pub(crate) filter_search: String,
    pub(crate) filter_tag: String,
    pub(crate) filter_from: String,
    pub(crate) filter_to: String,
    pub(crate) composer_priority: Priority,
    pub(crate) completion_filter: CompletionFilter,
    pub(crate) priority_filter: PriorityFilter,
    pub(crate) status_filter: StatusFilter,
    pub(crate) sort_mode: SortMode,
    pub(crate) quick_date: QuickDate,
    pub(crate) tasks: Vec<Task>,
}

impl Default for Taskscape {
    fn default() -> Self {
        Self {
            nav: NavSection::Tasks,
            theme_mode: ThemeMode::Dark,
            show_filters: false,
            title_input: String::new(),
            due_date_input: String::new(),
            tags_input: String::from("launch, inbox"),
            filter_search: String::new(),
            filter_tag: String::new(),
            filter_from: String::new(),
            filter_to: String::new(),
            composer_priority: Priority::Medium,
            completion_filter: CompletionFilter::Active,
            priority_filter: PriorityFilter::All,
            status_filter: StatusFilter::All,
            sort_mode: SortMode::Manual,
            quick_date: QuickDate::None,
            tasks: Vec::new(),
        }
    }
}

impl Application for Taskscape {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Taskscape")
    }

    fn theme(&self) -> Self::Theme {
        app_theme(self.theme_mode)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.handle_message(message);
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        self.view_root()
    }
}

pub fn run() -> iced::Result {
    Taskscape::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
