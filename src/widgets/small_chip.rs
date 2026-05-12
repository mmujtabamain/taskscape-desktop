use crate::app::AppElement;
use crate::thememanager::{ThemeMode, tokens};
use crate::widgets::caption;
use iced::widget::container;

pub fn small_chip<'a>(theme_mode: ThemeMode, label: &'a str, accent: bool) -> AppElement<'a> {
    let palette = tokens(theme_mode);
    let style = if accent {
        iced::widget::container::Style::default()
            .color(palette.text_primary)
            .background(palette.panel_alt)
            .border(crate::thememanager::helpers::border(16.0, 1.0, palette.border))
    } else {
        iced::widget::container::Style::default()
            .color(palette.text_primary)
            .background(palette.panel_raised)
            .border(crate::thememanager::helpers::border(14.0, 1.0, palette.border))
    };

    container(
        caption(
            label,
            13.0,
            if accent {
                palette.accent_text
            } else {
                palette.text_secondary
            },
        ),
    )
    .padding([6, 10])
    .style(move |_theme| style)
    .into()
}
