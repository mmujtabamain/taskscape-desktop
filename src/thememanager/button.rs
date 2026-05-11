use crate::thememanager::button_kind::ButtonKind;
use crate::thememanager::helpers::{border, mix, shadow, with_alpha};
use crate::thememanager::{ThemeMode, tokens};
use iced::theme;
use iced::widget::button;
use iced::{Color, Theme, Vector};

struct AppButtonStyle {
    mode: ThemeMode,
    kind: ButtonKind,
}

impl button::StyleSheet for AppButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let palette = tokens(self.mode);

        match self.kind {
            ButtonKind::Primary => button::Appearance {
                background: Some(palette.accent.into()),
                text_color: palette.accent_text,
                border: border(14.0, 0.0, palette.accent),
                shadow: shadow(palette.shadow, 0.0, 14.0),
                shadow_offset: Vector::new(0.0, 2.0),
            },
            ButtonKind::Secondary => button::Appearance {
                background: Some(palette.panel_raised.into()),
                text_color: palette.text_primary,
                border: border(14.0, 1.0, palette.border),
                ..button::Appearance::default()
            },
            ButtonKind::Ghost => button::Appearance {
                background: Some(with_alpha(palette.panel_raised, 0.35).into()),
                text_color: palette.text_primary,
                border: border(14.0, 1.0, palette.border),
                ..button::Appearance::default()
            },
            ButtonKind::Icon => button::Appearance {
                background: Some(palette.panel_raised.into()),
                text_color: palette.text_primary,
                border: border(12.0, 1.0, palette.border),
                ..button::Appearance::default()
            },
            ButtonKind::Sidebar(is_active) => button::Appearance {
                background: Some(
                    if is_active {
                        palette.sidebar_active
                    } else {
                        with_alpha(palette.panel_raised, 0.2)
                    }
                    .into(),
                ),
                text_color: if is_active {
                    palette.text_primary
                } else {
                    palette.text_secondary
                },
                border: border(
                    18.0,
                    1.0,
                    if is_active {
                        palette.border_strong
                    } else {
                        palette.border
                    },
                ),
                ..button::Appearance::default()
            },
            ButtonKind::Chip(is_selected) => button::Appearance {
                background: Some(
                    if is_selected {
                        palette.accent
                    } else {
                        with_alpha(palette.panel_raised, 0.4)
                    }
                    .into(),
                ),
                text_color: if is_selected {
                    palette.accent_text
                } else {
                    palette.text_secondary
                },
                border: border(
                    999.0,
                    1.0,
                    if is_selected {
                        palette.accent
                    } else {
                        palette.border
                    },
                ),
                ..button::Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let palette = tokens(self.mode);
        let mut appearance = self.active(style);

        match self.kind {
            ButtonKind::Primary => {
                appearance.background = Some(mix(palette.accent, Color::WHITE, 0.08).into());
            }
            ButtonKind::Secondary | ButtonKind::Ghost | ButtonKind::Icon => {
                appearance.background = Some(mix(palette.panel_raised, palette.panel_alt, 0.55).into());
                appearance.border.color = palette.border_strong;
            }
            ButtonKind::Sidebar(is_active) => {
                appearance.background = Some(
                    if is_active {
                        mix(palette.sidebar_active, palette.accent_soft, 0.28)
                    } else {
                        mix(palette.panel_raised, palette.sidebar_active, 0.32)
                    }
                    .into(),
                );
                appearance.text_color = palette.text_primary;
            }
            ButtonKind::Chip(is_selected) => {
                appearance.background = Some(
                    if is_selected {
                        mix(palette.accent, Color::WHITE, 0.08)
                    } else {
                        mix(palette.panel_raised, palette.accent_soft, 0.25)
                    }
                    .into(),
                );
            }
        }

        appearance
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = self.active(style);
        appearance.shadow_offset = Vector::default();
        appearance
    }
}

pub fn button_style(mode: ThemeMode, kind: ButtonKind) -> theme::Button {
    theme::Button::custom(AppButtonStyle { mode, kind })
}
