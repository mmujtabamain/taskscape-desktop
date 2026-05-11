use crate::thememanager::helpers::{background_gradient, border, shadow, with_alpha};
use crate::thememanager::{ThemeMode, tokens};
use iced::theme;
use iced::widget::container;

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
