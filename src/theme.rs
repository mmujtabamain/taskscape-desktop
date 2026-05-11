use iced::gradient;
use iced::theme;
use iced::widget::overlay::menu;
use iced::widget::{button, container, pick_list, radio, text_input};
use iced::{Border, Color, Shadow, Theme, Vector};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark mode",
            Self::Light => "Light mode",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AppPalette {
    pub background_top: Color,
    pub background_bottom: Color,
    pub sidebar: Color,
    pub sidebar_active: Color,
    pub panel: Color,
    pub panel_alt: Color,
    pub panel_raised: Color,
    pub border: Color,
    pub border_strong: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub accent_text: Color,
    pub shadow: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonKind {
    Primary,
    Secondary,
    Ghost,
    Icon,
    Sidebar(bool),
    Chip(bool),
}

pub fn app_theme(mode: ThemeMode) -> Theme {
    let palette = match mode {
        ThemeMode::Dark => theme::Palette {
            background: color(0x23, 0x18, 0x14),
            text: color(0xF6, 0xE9, 0xD6),
            primary: color(0xFF, 0x7C, 0x5D),
            success: color(0x81, 0xB2, 0x7D),
            danger: color(0xE2, 0x7F, 0x67),
        },
        ThemeMode::Light => theme::Palette {
            background: color(0xF6, 0xED, 0xDE),
            text: color(0x38, 0x20, 0x17),
            primary: color(0xF2, 0x6E, 0x53),
            success: color(0x7E, 0xA2, 0x74),
            danger: color(0xD1, 0x6B, 0x59),
        },
    };

    Theme::custom(format!("Taskscape {}", mode.label()), palette)
}

pub fn tokens(mode: ThemeMode) -> AppPalette {
    match mode {
        ThemeMode::Dark => AppPalette {
            background_top: color(0x4A, 0x30, 0x1D),
            background_bottom: color(0x2B, 0x1C, 0x16),
            sidebar: color(0x35, 0x2A, 0x24),
            sidebar_active: color(0x5A, 0x31, 0x23),
            panel: color(0x2D, 0x1F, 0x18),
            panel_alt: color(0x3C, 0x2D, 0x26),
            panel_raised: color(0x2B, 0x21, 0x1B),
            border: with_alpha(color(0xC5, 0x9A, 0x74), 0.18),
            border_strong: with_alpha(color(0xF2, 0xA1, 0x75), 0.5),
            text_primary: color(0xF5, 0xE7, 0xD4),
            text_secondary: color(0xC9, 0xB1, 0x99),
            text_muted: color(0x9B, 0x7F, 0x6C),
            accent: color(0xFF, 0x7C, 0x5D),
            accent_soft: color(0x6A, 0x3C, 0x2E),
            accent_text: color(0x21, 0x11, 0x0E),
            shadow: with_alpha(color(0x08, 0x05, 0x04), 0.35),
        },
        ThemeMode::Light => AppPalette {
            background_top: color(0xF6, 0xE9, 0xD8),
            background_bottom: color(0xF1, 0xE5, 0xD4),
            sidebar: color(0xF4, 0xEE, 0xE2),
            sidebar_active: color(0xF6, 0xC9, 0xBA),
            panel: color(0xF3, 0xE5, 0xD4),
            panel_alt: color(0xF7, 0xF0, 0xE3),
            panel_raised: color(0xFB, 0xF7, 0xEF),
            border: with_alpha(color(0x8E, 0x63, 0x4E), 0.18),
            border_strong: with_alpha(color(0xE6, 0x8B, 0x6B), 0.42),
            text_primary: color(0x31, 0x1D, 0x15),
            text_secondary: color(0x7A, 0x61, 0x53),
            text_muted: color(0x9B, 0x84, 0x75),
            accent: color(0xF2, 0x6E, 0x53),
            accent_soft: color(0xF7, 0xD8, 0xCC),
            accent_text: color(0xFE, 0xF8, 0xF1),
            shadow: with_alpha(color(0x6B, 0x4D, 0x3C), 0.10),
        },
    }
}

pub fn shell_container(mode: ThemeMode) -> theme::Container {
    let palette = tokens(mode);

    container::Appearance {
        text_color: Some(palette.text_primary),
        background: Some(background_gradient(mode).into()),
        ..container::Appearance::default()
    }
    .into()
}

pub fn sidebar_container(mode: ThemeMode) -> theme::Container {
    let palette = tokens(mode);

    container::Appearance {
        text_color: Some(palette.text_primary),
        background: Some(palette.sidebar.into()),
        border: border(0.0, 1.0, palette.border),
        ..container::Appearance::default()
    }
    .into()
}

pub fn panel_container(mode: ThemeMode) -> theme::Container {
    let palette = tokens(mode);

    container::Appearance {
        text_color: Some(palette.text_primary),
        background: Some(palette.panel.into()),
        border: border(18.0, 1.0, palette.border),
        shadow: shadow(palette.shadow, 0.0, 20.0),
    }
    .into()
}

pub fn panel_alt_container(mode: ThemeMode) -> theme::Container {
    let palette = tokens(mode);

    container::Appearance {
        text_color: Some(palette.text_primary),
        background: Some(palette.panel_alt.into()),
        border: border(16.0, 1.0, palette.border),
        ..container::Appearance::default()
    }
    .into()
}

pub fn panel_raised_container(mode: ThemeMode) -> theme::Container {
    let palette = tokens(mode);

    container::Appearance {
        text_color: Some(palette.text_primary),
        background: Some(palette.panel_raised.into()),
        border: border(14.0, 1.0, palette.border),
        ..container::Appearance::default()
    }
    .into()
}

pub fn empty_state_container(mode: ThemeMode) -> theme::Container {
    let palette = tokens(mode);

    container::Appearance {
        text_color: Some(palette.text_primary),
        background: Some(with_alpha(palette.panel_raised, 0.58).into()),
        border: border(18.0, 1.0, with_alpha(palette.border_strong, 0.55)),
        ..container::Appearance::default()
    }
    .into()
}

pub fn button_style(mode: ThemeMode, kind: ButtonKind) -> theme::Button {
    theme::Button::custom(AppButtonStyle { mode, kind })
}

pub fn text_input_style(mode: ThemeMode) -> theme::TextInput {
    theme::TextInput::Custom(Box::new(AppInputStyle { mode }))
}

pub fn pick_list_style(mode: ThemeMode) -> theme::PickList {
    theme::PickList::Custom(
        Rc::new(AppPickListStyle { mode }),
        Rc::new(AppMenuStyle { mode }),
    )
}

pub fn radio_style(mode: ThemeMode) -> theme::Radio {
    theme::Radio::Custom(Box::new(AppRadioStyle { mode }))
}

fn background_gradient(mode: ThemeMode) -> gradient::Linear {
    let palette = tokens(mode);

    gradient::Linear::new(0.25)
        .add_stop(0.0, palette.background_top)
        .add_stop(0.42, mix(palette.background_top, palette.background_bottom, 0.35))
        .add_stop(1.0, palette.background_bottom)
}

fn border(radius: f32, width: f32, color: Color) -> Border {
    Border {
        radius: radius.into(),
        width,
        color,
    }
}

fn shadow(color: Color, offset_y: f32, blur_radius: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::new(0.0, offset_y),
        blur_radius,
    }
}

fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

fn mix(from: Color, to: Color, amount: f32) -> Color {
    let clamped = amount.clamp(0.0, 1.0);

    Color {
        r: from.r + (to.r - from.r) * clamped,
        g: from.g + (to.g - from.g) * clamped,
        b: from.b + (to.b - from.b) * clamped,
        a: from.a + (to.a - from.a) * clamped,
    }
}

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
        appearance.background = mix(tokens(self.mode).panel_raised, tokens(self.mode).accent_soft, 0.35).into();
        appearance
    }
}