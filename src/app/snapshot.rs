use crate::models::{Priority, Task};
use crate::thememanager::ThemeMode;

#[derive(Debug, Clone)]
pub struct AppSnapshot {
    pub theme_mode: ThemeMode,
    pub title_input: String,
    pub due_date_input: String,
    pub composer_priority: Priority,
    pub tasks: Vec<Task>,
}
