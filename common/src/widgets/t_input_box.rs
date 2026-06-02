use crate::thememanager::{ThemeMode, text_input_style};
use iced::Element;
use iced::Length;
use iced::widget::text_input;

pub fn t_input_box<'a, M: Clone + 'a>(
    theme_mode: ThemeMode,
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> M + 'a,
    width: Length,
    on_submit: Option<M>,
) -> Element<'a, M> {
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
