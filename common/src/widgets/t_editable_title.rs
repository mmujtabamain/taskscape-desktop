use crate::thememanager::helpers::with_alpha;
use crate::thememanager::{ThemeMode, tokens};
use crate::utils::fonts::poppins_semibold;
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
    let palette = tokens(theme_mode);

    let underline_color = if is_editing {
        palette.accent
    } else {
        Color::TRANSPARENT
    };

    let field_style = move |_theme: &_, _status: text_input::Status| text_input::Style {
        background: Color::TRANSPARENT.into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: palette.text_muted,
        value: palette.text_primary,
        selection: with_alpha(palette.accent, 0.28),
    };

    let content: Element<'a, M> = if is_editing {
        text_input("Untitled", value)
            .id(TITLE_INPUT_ID)
            .font(poppins_semibold())
            .size(40.0)
            .width(Length::Fill)
            .padding([0, 0])
            .style(field_style)
            .on_input(on_input)
            .on_submit(on_toggle)
            .into()
    } else {
        // Clip the text to the full width so it doesn't overflow
        container(
            mouse_area(
                text(if value.is_empty() { "Untitled" } else { value })
                    .font(poppins_semibold())
                    .size(40.0)
                    .color(palette.text_primary),
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
