use crate::thememanager::helpers::border;
use crate::thememanager::{ThemeMode, tokens};
use iced::widget::pick_list;
use iced::Theme;

pub fn pick_list_style(
    mode: ThemeMode,
) -> impl Fn(&Theme, pick_list::Status) -> pick_list::Style + Clone {
    move |_theme: &Theme, status| {
        let palette = tokens(mode);
        let mut style = pick_list::Style {
            text_color: palette.text_primary,
            placeholder_color: palette.text_muted,
            handle_color: palette.text_secondary,
            background: palette.panel_raised.into(),
            border: border(12.0, 1.0, palette.border),
        };

        if matches!(status, pick_list::Status::Hovered | pick_list::Status::Opened { .. }) {
            style.border.color = palette.border_strong;
        }

        style
    }
}
