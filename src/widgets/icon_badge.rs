use crate::app::AppElement;
use crate::thememanager::{ThemeMode, panel_alt_container, panel_raised_container, tokens};
use iced::Length;
use iced::widget::{container, text};

pub fn icon_badge<'a>(theme_mode: ThemeMode, symbol: &'a str, accent: bool) -> AppElement<'a> {
    let palette = tokens(theme_mode);

    container(
        text(symbol)
            .size(18)
            .style(if accent { palette.accent_text } else { palette.text_primary }),
    )
    .width(Length::Fixed(42.0))
    .height(Length::Fixed(42.0))
    .center_x()
    .center_y()
    .style(if accent {
        panel_alt_container(theme_mode)
    } else {
        panel_raised_container(theme_mode)
    })
    .into()
}
