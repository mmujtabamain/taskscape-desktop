use crate::app::{AppElement, Message};
use crate::thememanager::{ThemeMode, text_input_style};
use iced::Length;
use iced::widget::text_input;

pub fn t_input_box<'a>(
    theme_mode: ThemeMode,
    placeholder: &'a str,
    value: &'a str,
    on_input: fn(String) -> Message,
    width: Length,
    on_submit: Option<Message>,
) -> AppElement<'a> {
    let mut field = text_input(placeholder, value)
        .width(width)
        .padding([12, 14])
        .size(16)
        .on_input(on_input)
        .style(text_input_style(theme_mode));

    if let Some(message) = on_submit {
        field = field.on_submit(message);
    }

    field.into()
}
