use crate::app::{AppElement, Message};
use crate::thememanager::{ThemeMode, radio_style};
use iced::Alignment;
use iced::widget::{radio, row};

pub fn radio_group<'a, T>(
    theme_mode: ThemeMode,
    options: &'a [T],
    selected: T,
    label: fn(T) -> &'static str,
    on_select: fn(T) -> Message,
) -> AppElement<'a>
where
    T: Copy + Eq + 'a,
{
    options
        .iter()
        .copied()
        .fold(row![].spacing(24).align_items(Alignment::Center), |row, option| {
            row.push(
                radio(label(option), option, Some(selected), on_select)
                    .text_size(15)
                    .spacing(12)
                    .style(radio_style(theme_mode)),
            )
        })
        .into()
}
