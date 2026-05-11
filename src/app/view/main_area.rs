use crate::app::{AppElement, Taskscape};
use crate::models::NavSection;
use iced::Length;
use iced::widget::{container, scrollable};

impl Taskscape {
    pub(crate) fn main_area(&self) -> AppElement<'_> {
        let content = match self.nav {
            NavSection::Tasks => self.tasks_view(),
            NavSection::Properties => self.properties_view(),
        };

        container(scrollable(content).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
