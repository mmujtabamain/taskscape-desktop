use crate::app::{AppElement, Message, Taskscape};
use common::ui::tokens::{space, text};
use common::ui::{Icon, ThemeMode, divider, palette, t_body, t_heading, t_icon_button};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row};

impl Taskscape {
    pub(crate) fn header(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        let list_name = self.current_list.as_deref().unwrap_or("No list open");

        let controls = row![
            t_icon_button(
                self.theme_mode,
                Icon::PanelToggle,
                None,
                Some(Message::ToggleListPanel),
            ),
            t_icon_button(self.theme_mode, Icon::Import, None, Some(Message::ImportList)),
            t_icon_button(self.theme_mode, Icon::Export, None, Some(Message::ExportList)),
            t_icon_button(
                self.theme_mode,
                if self.theme_mode == ThemeMode::Dark {
                    Icon::ThemeLight
                } else {
                    Icon::ThemeDark
                },
                None,
                Some(Message::ToggleTheme),
            ),
            t_icon_button(
                self.theme_mode,
                Icon::Undo,
                Some(self.undo_stack.len() as u32),
                Some(Message::EditUndo),
            ),
            t_icon_button(
                self.theme_mode,
                Icon::Redo,
                Some(self.redo_stack.len() as u32),
                Some(Message::EditRedo),
            ),
        ]
        .spacing(space::SM)
        .align_y(Alignment::Center);

        column![
            row![
                container(
                    column![
                        t_heading(list_name, text::HEADING, p.text),
                        t_body(
                            format!("{} tasks in this list", self.total_count()),
                            text::SMALL,
                            p.text_muted,
                        ),
                    ]
                    .spacing(space::XS)
                    .width(Length::Fill)
                )
                .width(Length::Fill),
                controls,
            ]
            .spacing(space::LG)
            .align_y(Alignment::Center),
            container(Space::new().height(Length::Fixed(1.0)))
                .width(Length::Fill)
                .style(divider(self.theme_mode)),
        ]
        .spacing(space::LG)
        .into()
    }
}
