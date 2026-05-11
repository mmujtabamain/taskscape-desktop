use crate::app::AppElement;
use crate::thememanager::{ThemeMode, panel_alt_container, tokens};
use iced::Length;
use iced::widget::{column, container, text};

pub fn metric_card(theme_mode: ThemeMode, value: String, label: &'static str) -> AppElement<'static> {
    let palette = tokens(theme_mode);

    container(
        column![
            text(value).size(30).style(palette.text_primary),
            text(label).size(14).style(palette.text_secondary),
        ]
        .spacing(4),
    )
    .width(Length::Fill)
    .padding(12)
    .style(panel_alt_container(theme_mode))
    .into()
}
