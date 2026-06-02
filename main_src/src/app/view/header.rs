use crate::app::{AppElement, Message, Taskscape};
use common::thememanager::{ThemeMode, panel_alt_container, tokens};
use common::widgets::{t_body, t_heading, t_icon_button};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn header(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let list_name = self.current_list.as_deref().unwrap_or("No list open");

        let controls = row![
            t_icon_button(
                self.theme_mode,
                Icon::PanelLeft,
                None,
                Some(Message::ToggleListPanel),
            ),
            t_icon_button(self.theme_mode, Icon::Import, None, Some(Message::ImportList)),
            t_icon_button(self.theme_mode, Icon::Upload, None, Some(Message::ExportList)),
            t_icon_button(
                self.theme_mode,
                if self.theme_mode == ThemeMode::Dark {
                    Icon::Sun
                } else {
                    Icon::Moon
                },
                None,
                Some(Message::ToggleTheme),
            ),
            t_icon_button(
                self.theme_mode,
                Icon::Undo2,
                Some(self.undo_stack.len() as u32),
                Some(Message::EditUndo),
            ),
            t_icon_button(
                self.theme_mode,
                Icon::Redo2,
                Some(self.redo_stack.len() as u32),
                Some(Message::EditRedo),
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        column![
            row![
                {
                    let title_section = column![
                        t_heading(list_name, 26.0, palette.text_primary),
                        t_body(
                            format!("{} tasks in this list", self.total_count()),
                            13.0,
                            palette.text_muted,
                        ),
                    ]
                    .spacing(2)
                    .width(Length::Fill);

                    container(title_section).width(Length::Fill)
                },
                controls,
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            container(Space::new().height(Length::Fixed(1.0)))
                .width(Length::Fill)
                .style(panel_alt_container(self.theme_mode)),
        ]
        .spacing(10)
        .into()
    }
}
