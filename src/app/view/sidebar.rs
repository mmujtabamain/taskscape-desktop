use crate::app::{AppElement, Message, Taskscape};
use crate::models::NavSection;
use crate::thememanager::{sidebar_container, tokens};
use crate::widgets::{icon_button, sidebar_button};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row, text};

impl Taskscape {
    pub(crate) fn sidebar(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let brand = row![
            crate::widgets::icon_badge(self.theme_mode, "▣", true),
            column![
                text("DASHBOARD").size(10).style(palette.text_muted),
                text("TaskScape").size(24).style(palette.text_primary)
            ]
            .spacing(2),
            Space::with_width(Length::Fill),
            icon_button(self.theme_mode, "‹", None, None),
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let tasks_item = sidebar_button(
            self.theme_mode,
            "Tasks",
            "0 visible in Todos",
            "☷",
            self.nav == NavSection::Tasks,
            Message::SetNav(NavSection::Tasks),
        );

        let properties_item = sidebar_button(
            self.theme_mode,
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
        .style(sidebar_container(self.theme_mode))
        .into()
    }
}
