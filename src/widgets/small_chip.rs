use crate::app::AppElement;
use crate::thememanager::{ThemeMode, panel_alt_container, panel_raised_container, tokens};
use iced::widget::{container, text};

pub fn small_chip<'a>(theme_mode: ThemeMode, label: &'a str, accent: bool) -> AppElement<'a> {
    let palette = tokens(theme_mode);

    container(
        text(label)
            .size(13)
            .style(if accent { palette.accent_text } else { palette.text_secondary }),
    )
    .padding([6, 10])
    .style(if accent {
        panel_alt_container(theme_mode)
    } else {
        panel_raised_container(theme_mode)
    })
    .into()
}
