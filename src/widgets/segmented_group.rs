use crate::app::{AppElement, Message};
use crate::thememanager::{ButtonKind, ThemeMode, button_style, tokens};
use iced::Alignment;
use iced::widget::{button, row, text};

fn segmented_button(
    theme_mode: ThemeMode,
    label: &'static str,
    selected: bool,
    message: Message,
) -> AppElement<'static> {
    let palette = tokens(theme_mode);

    button(text(label).size(14).style(if selected {
        palette.accent_text
    } else {
        palette.text_secondary
    }))
    .padding([7, 12])
    .style(button_style(theme_mode, ButtonKind::Chip(selected)))
    .on_press(message)
    .into()
}

pub fn segmented_group<'a, T>(
    theme_mode: ThemeMode,
    options: &'a [T],
    selected: T,
    label: fn(T) -> &'static str,
    on_select: fn(T) -> Message,
) -> AppElement<'a>
where
    T: Copy + PartialEq + 'a,
{
    options
        .iter()
        .copied()
        .fold(row![].spacing(8).align_items(Alignment::Center), |row, option| {
            row.push(segmented_button(
                theme_mode,
                label(option),
                option == selected,
                on_select(option),
            ))
        })
        .into()
}
