use crate::app::{AppElement, Message};
use crate::thememanager::{ThemeMode, pick_list_style};
use iced::Length;
use iced::widget::pick_list;
use std::borrow::Borrow;

pub fn t_dropdown<'a, T, L, V>(
    theme_mode: ThemeMode,
    options: L,
    selected: Option<V>,
    on_select: fn(T) -> Message,
    width: Length,
) -> AppElement<'a>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
{
    pick_list(options, selected, on_select)
        .width(width)
        .padding([12, 14])
        .text_size(16)
        .style(pick_list_style(theme_mode))
        .into()
}
