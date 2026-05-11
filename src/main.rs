mod theme;

use crate::theme::{ButtonKind, ThemeMode, app_theme, tokens};
use iced::alignment;
use iced::widget::{
    Space, button, column, container, pick_list, radio, row, scrollable, text,
    text_input,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Theme, executor,
};
use std::cmp::Reverse;
use std::fmt;

fn main() -> iced::Result {
    Taskscape::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
struct Task {
    title: String,
    priority: Priority,
    status: TaskStatus,
    due_date: Option<String>,
    tags: Vec<String>,
    archived: bool,
}

impl Task {
    fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Done)
    }

    fn matches_date_range(&self, from: &str, to: &str) -> bool {
        match self.due_date.as_deref() {
            Some(date) => {
                let from_match = from.trim().is_empty() || date >= from.trim();
                let to_match = to.trim().is_empty() || date <= to.trim();
                from_match && to_match
            }
            None => from.trim().is_empty() && to.trim().is_empty(),
        }
    }
}

#[derive(Debug, Clone)]
struct Taskscape {
    nav: NavSection,
    theme_mode: ThemeMode,
    show_filters: bool,
    title_input: String,
    due_date_input: String,
    tags_input: String,
    filter_search: String,
    filter_tag: String,
    filter_from: String,
    filter_to: String,
    composer_priority: Priority,
    completion_filter: CompletionFilter,
    priority_filter: PriorityFilter,
    status_filter: StatusFilter,
    sort_mode: SortMode,
    quick_date: QuickDate,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
enum Message {
    SetNav(NavSection),
    SetThemeMode(ThemeMode),
    ToggleTheme,
    ToggleFilters,
    TitleChanged(String),
    DueDateChanged(String),
    TagsChanged(String),
    FilterSearchChanged(String),
    FilterTagChanged(String),
    FilterFromChanged(String),
    FilterToChanged(String),
    ComposerPriorityChanged(Priority),
    CompletionFilterChanged(CompletionFilter),
    PriorityFilterChanged(PriorityFilter),
    StatusFilterChanged(StatusFilter),
    SortModeChanged(SortMode),
    QuickDateChanged(QuickDate),
    AddTask,
    ClearCompleted,
    ArchiveCompleted,
    ClearAll,
    ClearFilters,
    SaveFilters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum NavSection {
    #[default]
    Tasks,
    Properties,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

impl Priority {
    const ALL: [Self; 4] = [Self::Low, Self::Medium, Self::High, Self::Critical];

    fn label(self) -> &'static str {
        match self {
            Self::Low => "Low priority",
            Self::Medium => "Medium priority",
            Self::High => "High priority",
            Self::Critical => "Critical priority",
        }
    }

    fn short_label(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
            Self::Critical => 3,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PriorityFilter {
    All,
    Low,
    Medium,
    High,
    Critical,
}

impl PriorityFilter {
    const ALL: [Self; 5] = [
        Self::All,
        Self::Low,
        Self::Medium,
        Self::High,
        Self::Critical,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }

    fn matches(self, priority: Priority) -> bool {
        match self {
            Self::All => true,
            Self::Low => priority == Priority::Low,
            Self::Medium => priority == Priority::Medium,
            Self::High => priority == Priority::High,
            Self::Critical => priority == Priority::Critical,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
    Blocked,
}

impl TaskStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Todo => "Todo",
            Self::Doing => "Doing",
            Self::Done => "Done",
            Self::Blocked => "Blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusFilter {
    All,
    Todo,
    Doing,
    Done,
    Blocked,
}

impl StatusFilter {
    const ALL: [Self; 5] = [Self::All, Self::Todo, Self::Doing, Self::Done, Self::Blocked];

    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Todo => "Todo",
            Self::Doing => "Doing",
            Self::Done => "Done",
            Self::Blocked => "Blocked",
        }
    }

    fn matches(self, status: TaskStatus) -> bool {
        match self {
            Self::All => true,
            Self::Todo => status == TaskStatus::Todo,
            Self::Doing => status == TaskStatus::Doing,
            Self::Done => status == TaskStatus::Done,
            Self::Blocked => status == TaskStatus::Blocked,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompletionFilter {
    Active,
    Completed,
    Pending,
    Archived,
    All,
}

impl CompletionFilter {
    const ALL: [Self; 5] = [
        Self::Active,
        Self::Completed,
        Self::Pending,
        Self::Archived,
        Self::All,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Completed => "Completed",
            Self::Pending => "Pending",
            Self::Archived => "Archived",
            Self::All => "All",
        }
    }

    fn matches(self, task: &Task) -> bool {
        match self {
            Self::Active => !task.archived && !task.is_completed(),
            Self::Completed => task.is_completed(),
            Self::Pending => !task.archived && matches!(task.status, TaskStatus::Todo | TaskStatus::Doing),
            Self::Archived => task.archived,
            Self::All => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortMode {
    Manual,
    Newest,
    Oldest,
    DueDate,
    Priority,
    Alphabetical,
}

impl SortMode {
    const ALL: [Self; 6] = [
        Self::Manual,
        Self::Newest,
        Self::Oldest,
        Self::DueDate,
        Self::Priority,
        Self::Alphabetical,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Manual => "Manual",
            Self::Newest => "Newest",
            Self::Oldest => "Oldest",
            Self::DueDate => "Due date",
            Self::Priority => "Priority",
            Self::Alphabetical => "A-Z",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuickDate {
    None,
    Today,
    Overdue,
    ThisWeek,
}

impl QuickDate {
    const ALL: [Self; 4] = [Self::None, Self::Today, Self::Overdue, Self::ThisWeek];

    fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Today => "Today",
            Self::Overdue => "Overdue",
            Self::ThisWeek => "This week",
        }
    }

    fn matches(self, task: &Task) -> bool {
        match self {
            Self::None => true,
            Self::Today => matches!(task.due_date.as_deref(), Some("11/05/2026")),
            Self::Overdue => task
                .due_date
                .as_deref()
                .map(|date| date < "11/05/2026")
                .unwrap_or(false),
            Self::ThisWeek => task
                .due_date
                .as_deref()
                .map(|date| ("11/05/2026"..="17/05/2026").contains(&date))
                .unwrap_or(false),
        }
    }
}

impl Application for Taskscape {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Taskscape")
    }

    fn theme(&self) -> Self::Theme {
        app_theme(self.theme_mode)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetNav(nav) => self.nav = nav,
            Message::SetThemeMode(mode) => self.theme_mode = mode,
            Message::ToggleTheme => self.theme_mode = self.theme_mode.toggled(),
            Message::ToggleFilters => self.show_filters = !self.show_filters,
            Message::TitleChanged(value) => self.title_input = value,
            Message::DueDateChanged(value) => self.due_date_input = value,
            Message::TagsChanged(value) => self.tags_input = value,
            Message::FilterSearchChanged(value) => self.filter_search = value,
            Message::FilterTagChanged(value) => self.filter_tag = value,
            Message::FilterFromChanged(value) => self.filter_from = value,
            Message::FilterToChanged(value) => self.filter_to = value,
            Message::ComposerPriorityChanged(value) => self.composer_priority = value,
            Message::CompletionFilterChanged(value) => self.completion_filter = value,
            Message::PriorityFilterChanged(value) => self.priority_filter = value,
            Message::StatusFilterChanged(value) => self.status_filter = value,
            Message::SortModeChanged(value) => self.sort_mode = value,
            Message::QuickDateChanged(value) => self.quick_date = value,
            Message::AddTask => self.add_task(),
            Message::ClearCompleted => self.tasks.retain(|task| !task.is_completed()),
            Message::ArchiveCompleted => {
                for task in &mut self.tasks {
                    if task.is_completed() {
                        task.archived = true;
                    }
                }
            }
            Message::ClearAll => self.tasks.clear(),
            Message::ClearFilters => self.reset_filters(),
            Message::SaveFilters => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = row![self.sidebar(), self.main_area()]
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::shell_container(self.theme_mode))
            .into()
    }
}

impl Taskscape {
    fn add_task(&mut self) {
        let title = self.title_input.trim();

        if title.is_empty() {
            return;
        }

        let tags = self
            .tags_input
            .split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        self.tasks.push(Task {
            title: title.to_owned(),
            priority: self.composer_priority,
            status: TaskStatus::Todo,
            due_date: (!self.due_date_input.trim().is_empty()).then(|| self.due_date_input.trim().to_owned()),
            tags,
            archived: false,
        });

        self.title_input.clear();
        self.due_date_input.clear();
    }

    fn reset_filters(&mut self) {
        self.filter_search.clear();
        self.filter_tag.clear();
        self.filter_from.clear();
        self.filter_to.clear();
        self.completion_filter = CompletionFilter::Active;
        self.priority_filter = PriorityFilter::All;
        self.status_filter = StatusFilter::All;
        self.sort_mode = SortMode::Manual;
        self.quick_date = QuickDate::None;
    }

    fn filtered_tasks(&self) -> Vec<&Task> {
        let search = self.filter_search.to_lowercase();
        let tag_filter = self.filter_tag.to_lowercase();

        let mut tasks = self
            .tasks
            .iter()
            .filter(|task| self.completion_filter.matches(task))
            .filter(|task| self.priority_filter.matches(task.priority))
            .filter(|task| self.status_filter.matches(task.status))
            .filter(|task| self.quick_date.matches(task))
            .filter(|task| task.matches_date_range(&self.filter_from, &self.filter_to))
            .filter(|task| {
                search.is_empty()
                    || task.title.to_lowercase().contains(&search)
                    || task.tags.iter().any(|tag| tag.to_lowercase().contains(&search))
            })
            .filter(|task| {
                tag_filter.is_empty()
                    || task.tags.iter().any(|tag| tag.to_lowercase().contains(&tag_filter))
            })
            .collect::<Vec<_>>();

        match self.sort_mode {
            SortMode::Manual => {}
            SortMode::Newest => tasks.reverse(),
            SortMode::Oldest => {}
            SortMode::DueDate => tasks.sort_by_key(|task| task.due_date.as_deref().unwrap_or("99/99/9999")),
            SortMode::Priority => tasks.sort_by_key(|task| Reverse(task.priority.rank())),
            SortMode::Alphabetical => tasks.sort_by_key(|task| task.title.to_lowercase()),
        }

        tasks
    }

    fn visible_count(&self) -> usize {
        self.filtered_tasks().len()
    }

    fn open_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| !task.archived && !task.is_completed())
            .count()
    }

    fn archived_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.archived).count()
    }

    fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.is_completed()).count()
    }

    fn active_todos_count(&self) -> usize {
        self.tasks.iter().filter(|task| !task.archived).count()
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        let brand = row![
            self.icon_badge("▣", true),
            column![
                text("DASHBOARD")
                    .size(10)
                    .style(palette.text_muted),
                text("TaskScape").size(24).style(palette.text_primary)
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            self.icon_counter("‹", None, None),
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let tasks_item = self.sidebar_button(
            "Tasks",
            "0 visible in Todos",
            "☷",
            self.nav == NavSection::Tasks,
            Message::SetNav(NavSection::Tasks),
        );

        let properties_item = self.sidebar_button(
            "Properties",
            "Lists, persistence, import and export",
            "⚙",
            self.nav == NavSection::Properties,
            Message::SetNav(NavSection::Properties),
        );

        container(
            column![brand, tasks_item, properties_item, Space::with_height(Length::Fill)]
                .spacing(10)
                .padding([12, 10, 12, 10]),
        )
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(theme::sidebar_container(self.theme_mode))
        .into()
    }

    fn main_area(&self) -> Element<'_, Message> {
        let content = match self.nav {
            NavSection::Tasks => self.tasks_view(),
            NavSection::Properties => self.properties_view(),
        };

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn tasks_view(&self) -> Element<'_, Message> {
        let visible_tasks = self.filtered_tasks();

        let mut content = column![
            self.header("MULTI LIST PLANNER", "TaskScape", "Active list: Todos · Showing"),
            self.composer_row(),
        ]
        .spacing(16)
        .padding([22, 24, 30, 24]);

        if self.show_filters {
            content = content.push(self.filters_panel());
        }

        content = content
            .push(self.metrics_row())
            .push(self.actions_row())
            .push(self.workspace_panel(&visible_tasks));

        container(content)
            .width(Length::Fill)
            .style(theme::shell_container(self.theme_mode))
            .into()
    }

    fn properties_view(&self) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        let theme_group = row![
            radio(
                ThemeMode::Dark.label(),
                ThemeMode::Dark,
                Some(self.theme_mode),
                Message::SetThemeMode,
            )
            .text_size(15)
            .spacing(12)
            .style(theme::radio_style(self.theme_mode)),
            radio(
                ThemeMode::Light.label(),
                ThemeMode::Light,
                Some(self.theme_mode),
                Message::SetThemeMode,
            )
            .text_size(15)
            .spacing(12)
            .style(theme::radio_style(self.theme_mode)),
        ]
        .spacing(24)
        .align_items(Alignment::Center);

        let properties = column![
            self.header("WORKSPACE PROPERTIES", "Collections", "Manage appearance, persistence and workflow controls for TaskScape."),
            container(
                column![
                    self.section_heading("Appearance"),
                    text("Global theme is shared across the dashboard, fields, buttons, sidebar and overlays.")
                        .size(15)
                        .style(palette.text_secondary),
                    theme_group,
                ]
                .spacing(18)
                .padding(22),
            )
            .style(theme::panel_container(self.theme_mode)),
            row![
                self.info_card("Persistence", "Lists stay lightweight and local-first, with room for import and export flows."),
                self.info_card("Custom widgets", "Buttons, dropdowns, inputs, radio controls and sidebar tiles all share one design system."),
            ]
            .spacing(14),
            self.info_card(
                "Design language",
                "Warm editorial gradients, softened borders and a palette-led dark/light mode keep the desktop app aligned with the supplied reference images.",
            ),
        ]
        .spacing(18)
        .padding([22, 24, 30, 24]);

        container(properties)
            .width(Length::Fill)
            .style(theme::shell_container(self.theme_mode))
            .into()
    }

    fn header(&self, eyebrow: &'static str, title: &'static str, subtitle_prefix: &'static str) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);
        let summary = format!(
            "{} {} of {} todos",
            subtitle_prefix,
            self.visible_count(),
            self.tasks.len()
        );

        let controls = row![
            self.icon_counter(
                if self.theme_mode == ThemeMode::Dark { "☼" } else { "☾" },
                None,
                Some(Message::ToggleTheme),
            ),
            self.icon_counter("↺", Some(0), None),
            self.icon_counter("↻", Some(0), None),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        column![
            row![
                column![
                    text(eyebrow).size(11).style(palette.text_muted),
                    text(title).size(52).style(palette.text_primary),
                    text(summary).size(16).style(palette.text_secondary)
                ]
                .spacing(6),
                Space::with_width(Length::Fill),
                controls,
            ]
            .align_items(Alignment::Start),
            container(Space::with_height(Length::Fixed(1.0)))
                .width(Length::Fill)
                .style(theme::panel_alt_container(self.theme_mode)),
        ]
        .spacing(18)
        .into()
    }

    fn composer_row(&self) -> Element<'_, Message> {
        row![
            self.app_input(
                "Add a focused task, then press Enter",
                &self.title_input,
                Message::TitleChanged,
                Length::Fill,
                Some(Message::AddTask),
            ),
            self.priority_pick_list(),
            self.app_input(
                "dd/mm/yyyy",
                &self.due_date_input,
                Message::DueDateChanged,
                Length::Fixed(150.0),
                None,
            ),
            self.app_input(
                "tags: launch, inbox",
                &self.tags_input,
                Message::TagsChanged,
                Length::Fixed(150.0),
                None,
            ),
            self.labeled_button("☷", "Filters", ButtonKind::Secondary, Some(Message::ToggleFilters)),
            self.labeled_button("✦", "Add", ButtonKind::Primary, Some(Message::AddTask)),
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn filters_panel(&self) -> Element<'_, Message> {
        let save_row = row![
            Space::with_width(Length::Fill),
            self.labeled_button("⌁", "Save", ButtonKind::Ghost, Some(Message::SaveFilters)),
            self.labeled_button("×", "Clear", ButtonKind::Ghost, Some(Message::ClearFilters)),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let filters = column![
            save_row,
            row![
                self.filter_block(
                    "COMPLETION",
                    self.segmented_group(
                        &CompletionFilter::ALL,
                        self.completion_filter,
                        CompletionFilter::label,
                        Message::CompletionFilterChanged,
                    ),
                ),
                self.filter_block(
                    "PRIORITY",
                    self.segmented_group(
                        &PriorityFilter::ALL,
                        self.priority_filter,
                        PriorityFilter::label,
                        Message::PriorityFilterChanged,
                    ),
                ),
                self.filter_block(
                    "STATUS",
                    self.segmented_group(
                        &StatusFilter::ALL,
                        self.status_filter,
                        StatusFilter::label,
                        Message::StatusFilterChanged,
                    ),
                ),
            ]
            .spacing(20),
            row![
                self.filter_block(
                    "QUICK DATE",
                    self.segmented_group(
                        &QuickDate::ALL,
                        self.quick_date,
                        QuickDate::label,
                        Message::QuickDateChanged,
                    ),
                ),
                self.filter_block(
                    "SORT BY",
                    self.segmented_group(
                        &SortMode::ALL,
                        self.sort_mode,
                        SortMode::label,
                        Message::SortModeChanged,
                    ),
                ),
            ]
            .spacing(20),
            row![
                self.filter_field("SEARCH TEXT", "Search todos and notes", &self.filter_search, Message::FilterSearchChanged, Length::FillPortion(3)),
                self.filter_field("SEARCH TAG", "tag name", &self.filter_tag, Message::FilterTagChanged, Length::FillPortion(3)),
                self.filter_field("FROM", "dd/mm/yyyy", &self.filter_from, Message::FilterFromChanged, Length::FillPortion(2)),
                self.filter_field("TO", "dd/mm/yyyy", &self.filter_to, Message::FilterToChanged, Length::FillPortion(2)),
            ]
            .spacing(12),
        ]
        .spacing(18)
        .padding(14);

        container(filters)
            .style(theme::panel_container(self.theme_mode))
            .into()
    }

    fn metrics_row(&self) -> Element<'_, Message> {
        row![
            self.metric_card(self.visible_count().to_string(), "Visible"),
            self.metric_card(self.active_todos_count().to_string(), "In Todos"),
            self.metric_card(self.completed_count().to_string(), "Completed"),
            self.metric_card(self.open_count().to_string(), "Open"),
            self.metric_card(self.archived_count().to_string(), "Archived"),
            self.metric_card(String::from("14.2 KB"), "Storage used"),
        ]
        .spacing(8)
        .into()
    }

    fn actions_row(&self) -> Element<'_, Message> {
        row![
            self.labeled_button("✓", "Clear completed", ButtonKind::Ghost, Some(Message::ClearCompleted)),
            self.labeled_button("□", "Archive completed", ButtonKind::Ghost, Some(Message::ArchiveCompleted)),
            self.labeled_button("⌫", "Clear all", ButtonKind::Ghost, Some(Message::ClearAll)),
        ]
        .spacing(8)
        .into()
    }

    fn workspace_panel<'a>(&'a self, tasks: &[&'a Task]) -> Element<'a, Message> {
        let palette = tokens(self.theme_mode);

        let content: Element<'a, Message> = if tasks.is_empty() {
            container(
                column![
                    text("Your runway is clear")
                        .size(30)
                        .style(palette.text_primary),
                    text("Use the composer, import panel, or saved filters to build your workspace.")
                        .size(17)
                        .style(palette.text_secondary),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fixed(280.0))
            .center_x()
            .center_y()
            .style(theme::empty_state_container(self.theme_mode))
            .into()
        } else {
            let list = tasks.iter().fold(column![].spacing(12), |column, task| {
                column.push(self.task_card(task))
            });

            container(list)
                .width(Length::Fill)
                .style(theme::empty_state_container(self.theme_mode))
                .padding(14)
                .into()
        };

        content
    }

    fn task_card<'a>(&'a self, task: &'a Task) -> Element<'a, Message> {
        let palette = tokens(self.theme_mode);

        let chips = task.tags.iter().fold(row![].spacing(6), |row, tag| {
            row.push(self.small_chip(tag, false))
        });

        let meta = row![
            self.small_chip(task.priority.short_label(), true),
            self.small_chip(task.status.label(), false),
            match task.due_date.as_deref() {
                Some(date) => self.small_chip(date, false),
                None => self.small_chip("No date", false),
            },
        ]
        .spacing(6)
        .align_items(Alignment::Center);

        container(
            column![
                row![
                    column![
                        text(&task.title).size(20).style(palette.text_primary),
                        text("Captured in the current list workspace")
                            .size(14)
                            .style(palette.text_secondary)
                    ]
                    .spacing(4),
                    Space::with_width(Length::Fill),
                    meta,
                ]
                .align_items(Alignment::Center),
                chips,
            ]
            .spacing(12),
        )
        .padding(16)
        .style(theme::panel_alt_container(self.theme_mode))
        .into()
    }

    fn metric_card(&self, value: String, label: &'static str) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        container(
            column![
                text(value).size(30).style(palette.text_primary),
                text(label).size(14).style(palette.text_secondary),
            ]
            .spacing(4),
        )
        .width(Length::Fill)
        .padding(12)
        .style(theme::panel_alt_container(self.theme_mode))
        .into()
    }

    fn info_card(&self, title: &'static str, body: &'static str) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        container(
            column![
                text(title).size(22).style(palette.text_primary),
                text(body).size(15).style(palette.text_secondary),
            ]
            .spacing(8),
        )
        .width(Length::Fill)
        .padding(20)
        .style(theme::panel_container(self.theme_mode))
        .into()
    }

    fn section_heading(&self, label: &'static str) -> Element<'_, Message> {
        text(label).size(12).style(tokens(self.theme_mode).text_muted).into()
    }

    fn filter_block<'a>(&self, title: &'static str, content: Element<'a, Message>) -> Element<'a, Message> {
        let heading = text(title)
            .size(12)
            .style(tokens(self.theme_mode).text_muted);

        column![heading, content]
            .spacing(10)
            .width(Length::Fill)
            .into()
    }

    fn filter_field(
        &self,
        label: &'static str,
        placeholder: &'static str,
        value: &str,
        on_input: fn(String) -> Message,
        width: Length,
    ) -> Element<'_, Message> {
        column![
            self.section_heading(label),
            self.app_input(placeholder, value, on_input, width, None),
        ]
        .spacing(8)
        .into()
    }

    fn priority_pick_list(&self) -> Element<'_, Message> {
        pick_list(
            &Priority::ALL[..],
            Some(self.composer_priority),
            Message::ComposerPriorityChanged,
        )
        .width(Length::Fixed(190.0))
        .padding([12, 14])
        .text_size(16)
        .style(theme::pick_list_style(self.theme_mode))
        .into()
    }

    fn app_input(
        &self,
        placeholder: &'static str,
        value: &str,
        on_input: fn(String) -> Message,
        width: Length,
        on_submit: Option<Message>,
    ) -> Element<'_, Message> {
        let mut field = text_input(placeholder, value)
            .width(width)
            .padding([12, 14])
            .size(16)
            .on_input(on_input)
            .style(theme::text_input_style(self.theme_mode));

        if let Some(message) = on_submit {
            field = field.on_submit(message);
        }

        field.into()
    }

    fn icon_badge(&self, symbol: &'static str, accent: bool) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        container(
            text(symbol)
                .size(18)
                .style(if accent { palette.accent_text } else { palette.text_primary }),
        )
        .width(Length::Fixed(42.0))
        .height(Length::Fixed(42.0))
        .center_x()
        .center_y()
        .style(if accent {
            theme::panel_alt_container(self.theme_mode)
        } else {
            theme::panel_raised_container(self.theme_mode)
        })
        .into()
    }

    fn icon_counter(
        &self,
        symbol: &'static str,
        count: Option<u32>,
        message: Option<Message>,
    ) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);
        let label = count
            .map(|value| format!("{} {}", symbol, value))
            .unwrap_or_else(|| symbol.to_owned());

        button(
            text(label)
                .size(16)
                .style(palette.text_primary)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding([10, 12])
        .style(theme::button_style(self.theme_mode, ButtonKind::Icon))
        .on_press_maybe(message)
        .into()
    }

    fn labeled_button(
        &self,
        icon: &'static str,
        label: &'static str,
        kind: ButtonKind,
        message: Option<Message>,
    ) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        button(
            row![
                text(icon).size(15).style(palette.text_primary),
                text(label).size(16).style(palette.text_primary),
            ]
            .spacing(8)
            .align_items(Alignment::Center),
        )
        .padding([10, 14])
        .style(theme::button_style(self.theme_mode, kind))
        .on_press_maybe(message)
        .into()
    }

    fn sidebar_button(
        &self,
        title: &'static str,
        subtitle: &'static str,
        icon: &'static str,
        active: bool,
        message: Message,
    ) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        button(
            row![
                self.icon_badge(icon, active),
                column![
                    text(title).size(18).style(palette.text_primary),
                    text(subtitle).size(13).style(palette.text_secondary),
                ]
                .spacing(2),
            ]
            .spacing(12)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .padding(10)
        .style(theme::button_style(self.theme_mode, ButtonKind::Sidebar(active)))
        .on_press(message)
        .into()
    }

    fn small_chip(&self, label: &str, accent: bool) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        container(
            text(label)
                .size(13)
                .style(if accent { palette.accent_text } else { palette.text_secondary }),
        )
        .padding([6, 10])
        .style(if accent {
            theme::panel_alt_container(self.theme_mode)
        } else {
            theme::panel_raised_container(self.theme_mode)
        })
        .into()
    }

    fn segmented_group<T>(
        &self,
        options: &[T],
        selected: T,
        label: fn(T) -> &'static str,
        on_select: fn(T) -> Message,
    ) -> Element<'_, Message>
    where
        T: Copy + PartialEq,
    {
        options
            .iter()
            .copied()
            .fold(row![].spacing(8).align_items(Alignment::Center), |row, option| {
                row.push(self.segmented_button(label(option), option == selected, on_select(option)))
            })
            .into()
    }

    fn segmented_button(
        &self,
        label: &'static str,
        selected: bool,
        message: Message,
    ) -> Element<'_, Message> {
        let palette = tokens(self.theme_mode);

        button(text(label).size(14).style(if selected {
            palette.accent_text
        } else {
            palette.text_secondary
        }))
        .padding([7, 12])
        .style(theme::button_style(self.theme_mode, ButtonKind::Chip(selected)))
        .on_press(message)
        .into()
    }
}
