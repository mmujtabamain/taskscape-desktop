use crate::app::AppElement;
use crate::thememanager::ThemeMode;
use crate::widgets::section_heading;
use iced::Length;
use iced::widget::column;

pub fn filter_block<'a>(
    theme_mode: ThemeMode,
    title: &'a str,
    content: AppElement<'a>,
) -> AppElement<'a> {
    column![section_heading(theme_mode, title), content]
        .spacing(10)
        .width(Length::Fill)
        .into()
}
