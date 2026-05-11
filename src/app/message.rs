use crate::models::{
    CompletionFilter, NavSection, Priority, PriorityFilter, QuickDate, SortMode,
    StatusFilter,
};
use crate::thememanager::ThemeMode;

#[derive(Debug, Clone)]
pub enum Message {
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
