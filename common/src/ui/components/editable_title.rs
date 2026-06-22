//! The large, inline-editable list title (Raleway Bold display). Click to edit;
//! an accent underline appears while editing.

use crate::ui::theme::{ThemeMode, palette, with_alpha};
use crate::ui::tokens::text as text_size;
use crate::utils::fonts::raleway_bold;
use iced::widget::{Space, column, container, mouse_area, text, text_input};
use iced::{Border, Color, Element, Length};

pub const TITLE_INPUT_ID: &str = "title_input";

pub fn t_editable_title<'a, M: Clone + 'a>(
    theme_mode: ThemeMode,
    value: &'a str,
    is_editing: bool,
    on_input: impl Fn(String) -> M + 'a,
    on_toggle: M,
) -> Element<'a, M> {
    let p = palette(theme_mode);

    let underline_color = if is_editing { p.accent } else { Color::TRANSPARENT };

    let field_style = move |_t: &_, _status: text_input::Status| text_input::Style {
        background: Color::TRANSPARENT.into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: p.text_muted,
        value: p.text,
        selection: with_alpha(p.accent, 0.28),
    };

    let content: Element<'a, M> = if is_editing {
        text_input("Untitled", value)
            .id(TITLE_INPUT_ID)
            .font(raleway_bold())
            .size(text_size::DISPLAY)
            .width(Length::Fill)
            .padding([0, 0])
            .style(field_style)
            .on_input(on_input)
            .on_submit(on_toggle)
            .into()
    } else {
        container(
            mouse_area(
                text(if value.is_empty() { "Untitled" } else { value })
                    .font(raleway_bold())
                    .size(text_size::DISPLAY)
                    .color(p.text),
            )
            .on_double_click(on_toggle),
        )
        .width(Length::Fill)
        .clip(true)
        .into()
    };

    column![
        content,
        container(Space::new().height(Length::Fixed(2.0)))
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(underline_color.into()),
                ..container::Style::default()
            }),
    ]
    .into()
}
