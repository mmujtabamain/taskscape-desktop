use crate::app::AppElement;
use crate::thememanager::{ThemeMode, panel_alt_container, tokens};
use crate::widgets::{body, heading};
use iced::Length;
use iced::widget::{column, container};

pub fn metric_card(theme_mode: ThemeMode, value: String, label: &'static str) -> AppElement<'static> {
    let palette = tokens(theme_mode);

    container(
        column![
            heading(value, 30.0, palette.text_primary),
            body(label, 14.0, palette.text_secondary),
        ]
        .spacing(4),
    )
    .width(Length::Fill)
    .padding(12)
    .style(panel_alt_container(theme_mode))
    .into()
}
