use crate::app::{AppElement, Message, Taskscape};
use crate::thememanager::{panel_alt_container, tokens};
use crate::widgets::{t_body, t_heading, t_icon_button};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn header(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let controls = row![
            t_icon_button(
                self.theme_mode,
                Icon::FilePlus,
                None,
                Some(Message::FileNew),
            ),
            t_icon_button(self.theme_mode, Icon::Save, None, Some(Message::FileSave),),
            t_icon_button(
                self.theme_mode,
                Icon::FolderOpen,
                None,
                Some(Message::FileLoad),
            ),
            t_icon_button(
                self.theme_mode,
                if self.theme_mode == crate::thememanager::ThemeMode::Dark {
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
        .spacing(10)
        .align_y(Alignment::Center);

        column![
            row![
                column![
                    t_heading("Taskscape", 40.0, palette.text_primary),
                    t_body(
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
