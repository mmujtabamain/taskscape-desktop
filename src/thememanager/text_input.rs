use crate::thememanager::helpers::{border, with_alpha};
use crate::thememanager::{ThemeMode, tokens};
use iced::theme;
use iced::widget::text_input;
use iced::{Color, Theme};

struct AppInputStyle {
    mode: ThemeMode,
}

impl text_input::StyleSheet for AppInputStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = tokens(self.mode);

        text_input::Appearance {
            background: palette.panel_raised.into(),
            border: border(12.0, 1.0, palette.border),
            icon_color: palette.text_secondary,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.border.color = tokens(self.mode).border_strong;
        appearance
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.border.color = tokens(self.mode).accent;
        appearance
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        tokens(self.mode).text_muted
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        tokens(self.mode).text_primary
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        tokens(self.mode).text_muted
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        with_alpha(tokens(self.mode).accent, 0.28)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = self.active(style);
        appearance.background = with_alpha(tokens(self.mode).panel_raised, 0.5).into();
        appearance
    }
}

pub fn text_input_style(mode: ThemeMode) -> theme::TextInput {
    theme::TextInput::Custom(Box::new(AppInputStyle { mode }))
}
