use crate::thememanager::button_kind::ButtonKind;
use crate::thememanager::helpers::{border, mix, shadow, with_alpha};
use crate::thememanager::{ThemeMode, tokens};
use iced::widget::button;
use iced::{Color, Theme};

fn radius_for_kind(kind: ButtonKind) -> f32 {
    match kind {
        ButtonKind::Primary | ButtonKind::Ghost => 14.0,
        ButtonKind::Icon | ButtonKind::Plain => 12.0,
    }
}

fn active_style(mode: ThemeMode, kind: ButtonKind) -> button::Style {
    let palette = tokens(mode);

    match kind {
        ButtonKind::Primary => button::Style {
            background: Some(palette.accent.into()),
            text_color: palette.accent_text,
            border: border(14.0, 0.0, palette.accent),
            shadow: shadow(palette.shadow, 0.0, 14.0),
            ..button::Style::default()
        },
        ButtonKind::Ghost => button::Style {
            background: Some(with_alpha(palette.panel_raised, 0.35).into()),
            text_color: palette.text_primary,
            border: border(14.0, 1.0, palette.border),
            ..button::Style::default()
        },
        ButtonKind::Icon => button::Style {
            background: Some(palette.panel_raised.into()),
            text_color: palette.text_primary,
            border: border(12.0, 1.0, palette.border),
            ..button::Style::default()
        },
        ButtonKind::Plain => button::Style {
            background: None,
            text_color: palette.text_primary,
            border: border(12.0, 0.0, Color::TRANSPARENT),
            ..button::Style::default()
        },
    }
}

pub fn button_style(
    mode: ThemeMode,
    kind: ButtonKind,
) -> impl Fn(&Theme, button::Status) -> button::Style + Clone {
    move |_theme: &Theme, status| {
        let palette = tokens(mode);
        let mut style = active_style(mode, kind);

        match status {
            button::Status::Active => style,
            button::Status::Hovered => {
                match kind {
                    ButtonKind::Primary => {
                        style.background = Some(mix(palette.accent, Color::WHITE, 0.08).into());
                    }
                    ButtonKind::Ghost | ButtonKind::Icon => {
                        style.background =
                            Some(mix(palette.panel_raised, palette.panel_alt, 0.55).into());
                        style.border.color = palette.border_strong;
                    }
                    // Borderless: only a faint fill so the row container shows
                    // through, no border to avoid a redundant box.
                    ButtonKind::Plain => {
                        style.background = Some(with_alpha(palette.text_primary, 0.06).into());
                    }
                }

                style
            }
            button::Status::Pressed => {
                style.shadow = iced::Shadow::default();
                match kind {
                    // Borderless: deepen the hover fill on press, still no border.
                    ButtonKind::Plain => {
                        style.background = Some(with_alpha(palette.text_primary, 0.12).into());
                    }
                    _ => style.border.color = palette.border_strong,
                }
                style
            }
            button::Status::Disabled => button::Style {
                background: style.background.map(|background| match background {
                    iced::Background::Color(color) => with_alpha(color, 0.65).into(),
                    other => other,
                }),
                text_color: with_alpha(style.text_color, 0.55),
                border: border(
                    radius_for_kind(kind),
                    style.border.width,
                    with_alpha(style.border.color, 0.4),
                ),
                shadow: iced::Shadow::default(),
                ..style
            },
        }
    }
}
