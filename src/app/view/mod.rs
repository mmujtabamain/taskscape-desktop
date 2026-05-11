pub mod filters;
pub mod header;
pub mod main_area;
pub mod properties;
pub mod sidebar;
pub mod tasks;
pub mod workspace;

use crate::app::{AppElement, Taskscape};
use crate::thememanager::shell_container;
use iced::Length;
use iced::widget::{container, row};

impl Taskscape {
    pub(crate) fn view_root(&self) -> AppElement<'_> {
        let content = row![self.sidebar(), self.main_area()]
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(shell_container(self.theme_mode))
            .into()
    }
}
