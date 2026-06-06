//! The settings page, shown in place of the task workspace (toggled from the
//! sidebar gear). Preferences apply and persist immediately; the mini-window
//! hotkey is captured live and pushed to the tray service.

use crate::app::{AppElement, Message, Taskscape};
use common::thememanager::helpers::border;
use common::thememanager::{
    ButtonKind, ThemeMode, empty_state_container, panel_alt_container, tokens,
};
use common::widgets::{t_body, t_button, t_caption, t_dropdown, t_heading};
use iced::widget::{column, container, row, scrollable, toggler};
use iced::{Alignment, Length};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn settings_view(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let header = row![
            column![
                t_heading("Settings", 26.0, palette.text_primary),
                t_body(
                    "Changes are saved automatically.",
                    13.0,
                    palette.text_muted,
                ),
            ]
            .spacing(2)
            .width(Length::Fill),
            t_button(
                self.theme_mode,
                Some(Icon::Check),
                "Done",
                ButtonKind::Primary,
                Some(Message::CloseSettings),
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(12);

        let sections = column![
            self.settings_section(
                "Appearance",
                column![self.theme_setting()].spacing(8).into(),
            ),
            self.settings_section(
                "Mini window",
                column![self.hotkey_setting(), self.hotkey_enabled_setting()]
                    .spacing(8)
                    .into(),
            ),
            self.settings_section(
                "General",
                column![self.reopen_setting(), self.confirm_clear_setting()]
                    .spacing(8)
                    .into(),
            ),
        ]
        .spacing(18);

        let body = container(scrollable(sections).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(14)
            .style(empty_state_container(self.theme_mode));

        column![header, body]
            .spacing(12)
            .height(Length::Fill)
            .into()
    }

    /// A titled group of setting rows.
    fn settings_section<'a>(&self, title: &'a str, rows: AppElement<'a>) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);
        column![
            t_caption(title.to_uppercase(), 11.0, palette.text_muted),
            rows,
        ]
        .spacing(8)
        .into()
    }

    /// One setting: a label + description on the left, its control on the right.
    fn setting_row<'a>(
        &self,
        title: &'a str,
        description: &'a str,
        control: AppElement<'a>,
    ) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);
        container(
            row![
                column![
                    t_body(title, 15.0, palette.text_primary),
                    t_caption(description, 12.0, palette.text_muted),
                ]
                .spacing(2)
                .width(Length::Fill),
                control,
            ]
            .align_y(Alignment::Center)
            .spacing(12),
        )
        .width(Length::Fill)
        .padding([12, 14])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }

    fn theme_setting(&self) -> AppElement<'_> {
        let control = t_dropdown(
            self.theme_mode,
            ThemeMode::ALL,
            Some(self.theme_mode),
            Message::SetTheme,
            Length::Fixed(150.0),
        );
        self.setting_row("Theme", "Switch between light and dark.", control)
    }

    fn hotkey_setting(&self) -> AppElement<'_> {
        self.setting_row(
            "Show mini window",
            "Global shortcut to toggle the mini window from anywhere.",
            self.hotkey_control(),
        )
    }

    /// The hotkey control: while recording, a prompt + Cancel; otherwise the
    /// current binding as a chip plus Change / Reset.
    fn hotkey_control(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        if self.recording_hotkey {
            return row![
                t_body("Press keys…  (Esc to cancel)", 14.0, palette.accent),
                t_button(
                    self.theme_mode,
                    None,
                    "Cancel",
                    ButtonKind::Ghost,
                    Some(Message::CancelRecordHotkey),
                ),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .into();
        }

        let chip = container(t_body(self.hotkey.label(), 15.0, palette.text_primary))
            .padding([6, 12])
            .style(move |_theme: &iced::Theme| {
                iced::widget::container::Style::default()
                    .background(palette.panel_raised)
                    .border(border(8.0, 1.0, palette.border))
            });

        row![
            chip,
            t_button(
                self.theme_mode,
                Some(Icon::Keyboard),
                "Change",
                ButtonKind::Icon,
                Some(Message::StartRecordHotkey),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::RotateCcw),
                "Reset",
                ButtonKind::Ghost,
                Some(Message::ResetHotkey),
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    }

    fn hotkey_enabled_setting(&self) -> AppElement<'_> {
        let control = toggler(self.hotkey_enabled)
            .size(22)
            .on_toggle(Message::SetHotkeyEnabled);
        self.setting_row(
            "Enable shortcut",
            "Turn the global mini-window shortcut on or off.",
            control.into(),
        )
    }

    fn reopen_setting(&self) -> AppElement<'_> {
        let control = toggler(self.reopen_last_list)
            .size(22)
            .on_toggle(Message::SetReopenLastList);
        self.setting_row(
            "Reopen last list",
            "Open the list you were last using when Taskscape starts.",
            control.into(),
        )
    }

    fn confirm_clear_setting(&self) -> AppElement<'_> {
        let control = toggler(self.confirm_clear_all)
            .size(22)
            .on_toggle(Message::SetConfirmClearAll);
        self.setting_row(
            "Confirm “Clear all”",
            "Ask before removing every task in a list.",
            control.into(),
        )
    }
}
