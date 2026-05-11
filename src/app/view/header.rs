use crate::app::{AppElement, Message, Taskscape};
use crate::thememanager::{panel_alt_container, tokens};
use crate::widgets::icon_button;
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row, text};

impl Taskscape {
    pub(crate) fn header(
        &self,
        eyebrow: &'static str,
        title: &'static str,
        subtitle_prefix: &'static str,
    ) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let summary = format!(
            "{} {} of {} todos",
            subtitle_prefix,
            self.visible_count(),
            self.tasks.len()
        );

        let controls = row![
            icon_button(
                self.theme_mode,
                if self.theme_mode == crate::thememanager::ThemeMode::Dark {
                    "☼"
                } else {
                    "☾"
                },
                None,
                Some(Message::ToggleTheme),
            ),
            icon_button(self.theme_mode, "↺", Some(0), None),
            icon_button(self.theme_mode, "↻", Some(0), None),
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
                .style(panel_alt_container(self.theme_mode)),
        ]
        .spacing(18)
        .into()
    }
}
