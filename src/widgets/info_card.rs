use crate::app::AppElement;
use crate::thememanager::{ThemeMode, panel_container, tokens};
use iced::Length;
use iced::widget::{column, container, text};

pub fn info_card<'a>(theme_mode: ThemeMode, title: &'a str, body: &'a str) -> AppElement<'a> {
    let palette = tokens(theme_mode);

    container(
        column![
            text(title).size(22).style(palette.text_primary),
            text(body).size(15).style(palette.text_secondary),
        ]
        .spacing(8),
    )
    .width(Length::Fill)
    .padding(20)
    .style(panel_container(theme_mode))
    .into()
}
