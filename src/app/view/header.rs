use crate::app::{AppElement, Message, Taskscape};
use crate::thememanager::{panel_alt_container, tokens};
use crate::widgets::{body, heading, icon_button};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn header(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let controls = row![
            icon_button(
                self.theme_mode,
                Icon::FilePlus,
                None,
                Some(Message::FileNew),
            ),
            icon_button(
                self.theme_mode,
                Icon::Save,
                None,
                Some(Message::FileSave),
            ),
            icon_button(
                self.theme_mode,
                Icon::FolderOpen,
                None,
                Some(Message::FileLoad),
            ),
            icon_button(
                self.theme_mode,
                if self.theme_mode == crate::thememanager::ThemeMode::Dark {
                    Icon::Sun
                } else {
                    Icon::Moon
                },
                None,
                Some(Message::ToggleTheme),
            ),
            icon_button(
                self.theme_mode,
                Icon::Undo2,
                Some(self.undo_stack.len() as u32),
                Some(Message::EditUndo),
            ),
            icon_button(
                self.theme_mode,
                Icon::Redo2,
                Some(self.redo_stack.len() as u32),
                Some(Message::EditRedo),
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        column![
            row![
                column![
                    crate::widgets::caption("TODO", 11.0, palette.text_muted),
                    heading("Taskscape Desktop", 40.0, palette.text_primary),
                    body(
                        format!("{} tasks in this list.", self.total_count()),
                        15.0,
                        palette.text_secondary,
                    ),
                ]
                .spacing(6),
                Space::new().width(Length::Fill),
                controls,
            ]
            .align_y(Alignment::Start),
            container(Space::new().height(Length::Fixed(1.0)))
                .width(Length::Fill)
                .style(panel_alt_container(self.theme_mode)),
        ]
        .spacing(18)
        .into()
    }
}
