use crate::app::AppElement;
use crate::thememanager::{ThemeMode, tokens};
use iced::widget::text;

pub fn section_heading<'a>(theme_mode: ThemeMode, label: &'a str) -> AppElement<'a> {
    text(label).size(12).style(tokens(theme_mode).text_muted).into()
}
