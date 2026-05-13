use crate::app::AppElement;
use crate::thememanager::{ThemeMode, panel_alt_container, tokens};
use crate::widgets::{t_body, t_heading};
use iced::Length;
use iced::widget::{column, container};

pub fn t_metric_card(theme_mode: ThemeMode, value: String, label: &'static str) -> AppElement<'static> {
    let palette = tokens(theme_mode);

    container(
        column![
            t_heading(value, 30.0, palette.text_primary),
            t_body(label, 14.0, palette.text_secondary),
        ]
        .spacing(4),
    )
    .width(Length::Fill)
    .padding(12)
    .style(panel_alt_container(theme_mode))
    .into()
}
