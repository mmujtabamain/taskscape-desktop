use crate::thememanager::helpers::mix;
use crate::thememanager::{ThemeMode, tokens};
use iced::Theme;
use iced::theme;
use iced::widget::radio;

struct AppRadioStyle {
    mode: ThemeMode,
}

impl radio::StyleSheet for AppRadioStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style, is_selected: bool) -> radio::Appearance {
        let palette = tokens(self.mode);

        radio::Appearance {
            background: if is_selected {
                palette.accent_soft.into()
            } else {
                palette.panel_raised.into()
            },
            dot_color: palette.accent,
            border_width: 1.0,
            border_color: if is_selected {
                palette.accent
            } else {
                palette.border_strong
            },
            text_color: Some(palette.text_primary),
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        let mut appearance = self.active(style, is_selected);
        appearance.background =
            mix(tokens(self.mode).panel_raised, tokens(self.mode).accent_soft, 0.35).into();
        appearance
    }
}

pub fn radio_style(mode: ThemeMode) -> theme::Radio {
    theme::Radio::Custom(Box::new(AppRadioStyle { mode }))
}
