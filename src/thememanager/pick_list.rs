use crate::thememanager::helpers::border;
use crate::thememanager::{ThemeMode, tokens};
use iced::theme;
use iced::widget::overlay::menu;
use iced::widget::pick_list;
use iced::Theme;
use std::rc::Rc;

struct AppPickListStyle {
    mode: ThemeMode,
}

impl pick_list::StyleSheet for AppPickListStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        let palette = tokens(self.mode);

        pick_list::Appearance {
            text_color: palette.text_primary,
            background: palette.panel_raised.into(),
            placeholder_color: palette.text_muted,
            handle_color: palette.text_secondary,
            border: border(12.0, 1.0, palette.border),
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let mut appearance = self.active(style);
        appearance.border.color = tokens(self.mode).border_strong;
        appearance
    }
}

struct AppMenuStyle {
    mode: ThemeMode,
}

impl menu::StyleSheet for AppMenuStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        let palette = tokens(self.mode);

        menu::Appearance {
            text_color: palette.text_primary,
            background: palette.panel.into(),
            border: border(12.0, 1.0, palette.border),
            selected_text_color: palette.accent_text,
            selected_background: palette.accent.into(),
        }
    }
}

pub fn pick_list_style(mode: ThemeMode) -> theme::PickList {
    theme::PickList::Custom(
        Rc::new(AppPickListStyle { mode }),
        Rc::new(AppMenuStyle { mode }),
    )
}
