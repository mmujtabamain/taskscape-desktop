use crate::thememanager::{ThemeMode, pick_list_style};
use iced::Element;
use iced::Length;
use iced::widget::pick_list;
use std::borrow::Borrow;

pub fn t_dropdown<'a, M, T, L, V>(
    theme_mode: ThemeMode,
    options: L,
    selected: Option<V>,
    on_select: impl Fn(T) -> M + 'a,
    width: Length,
) -> Element<'a, M>
where
    M: Clone + 'a,
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
