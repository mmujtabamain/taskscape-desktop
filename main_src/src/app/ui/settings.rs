//! The settings page, shown in place of the task workspace (toggled from the
//! sidebar gear). Preferences apply and persist immediately; the mini-window
//! hotkey is captured live and pushed to the tray service.

use crate::app::{AppElement, Message, Taskscape};
use common::ui::tokens::{radius, space, text};
use common::ui::{
    ButtonKind, Icon, ThemeMode, border, palette, surface, t_body, t_button, t_caption, t_dropdown,
    t_heading, t_toggle,
};
use iced::widget::{column, container, row, scrollable};
use iced::{Alignment, Length};

impl Taskscape {
    pub(crate) fn settings_view(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        let header = row![
            column![
                t_heading("Settings", text::HEADING, p.text),
                t_body("Changes are saved automatically.", text::SMALL, p.text_muted),
            ]
            .spacing(space::XS)
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
        .spacing(space::LG);

        let sections = column![
            self.settings_section("Appearance", column![self.theme_setting(), self.motion_setting()].spacing(space::MD).into()),
            self.settings_section(
                "Mini window",
                column![self.hotkey_setting(), self.hotkey_enabled_setting()].spacing(space::MD).into(),
            ),
            self.settings_section(
                "General",
                column![self.reopen_setting(), self.confirm_clear_setting()].spacing(space::MD).into(),
            ),
        ]
        .spacing(space::XL + 2.0);

        let body = container(scrollable(sections).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(space::XL)
            .style(surface(self.theme_mode));

        column![header, body].spacing(space::LG).height(Length::Fill).into()
    }

    fn settings_section<'a>(&self, title: &'a str, rows: AppElement<'a>) -> AppElement<'a> {
        let p = palette(self.theme_mode);
        column![t_caption(title.to_uppercase(), text::CAPTION, p.text_muted), rows]
            .spacing(space::MD)
            .into()
    }

    fn setting_row<'a>(
        &self,
        title: &'a str,
        description: &'a str,
        control: AppElement<'a>,
    ) -> AppElement<'a> {
        let p = palette(self.theme_mode);
        container(
            row![
                column![
                    t_body(title, text::BODY, p.text),
                    t_caption(description, text::LABEL, p.text_muted),
                ]
                .spacing(space::XS)
                .width(Length::Fill),
                control,
            ]
            .align_y(Alignment::Center)
            .spacing(space::LG),
        )
        .width(Length::Fill)
        .padding([12, 14])
        .style(common::ui::raised(self.theme_mode))
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

    fn motion_setting(&self) -> AppElement<'_> {
        let control = t_toggle(
            self.theme_mode,
            self.reduce_motion,
            Message::SetReduceMotion(!self.reduce_motion),
        );
        self.setting_row(
            "Reduce motion",
            "Collapse animations to instant transitions.",
            control,
        )
    }

    fn hotkey_setting(&self) -> AppElement<'_> {
        self.setting_row(
            "Show mini window",
            "Global shortcut to toggle the mini window from anywhere.",
            self.hotkey_control(),
        )
    }

    fn hotkey_control(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        if self.recording_hotkey {
            return row![
                t_body("Press keys…  (Esc to cancel)", text::BODY, p.accent),
                t_button(
                    self.theme_mode,
                    None,
                    "Cancel",
                    ButtonKind::Ghost,
                    Some(Message::CancelRecordHotkey),
                ),
            ]
            .spacing(space::LG)
            .align_y(Alignment::Center)
            .into();
        }

        let chip = container(t_body(self.hotkey.label(), text::BODY, p.text))
            .padding([6, 12])
            .style(move |_theme: &iced::Theme| {
                iced::widget::container::Style::default()
                    .background(p.raised)
                    .border(border(radius::SM, 0.0, p.raised))
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
                Some(Icon::Reset),
                "Reset",
                ButtonKind::Ghost,
                Some(Message::ResetHotkey),
            ),
        ]
        .spacing(space::MD)
        .align_y(Alignment::Center)
        .into()
    }

    fn hotkey_enabled_setting(&self) -> AppElement<'_> {
        let control = t_toggle(
            self.theme_mode,
            self.hotkey_enabled,
            Message::SetHotkeyEnabled(!self.hotkey_enabled),
        );
        self.setting_row(
            "Enable shortcut",
            "Turn the global mini-window shortcut on or off.",
            control,
        )
    }

    fn reopen_setting(&self) -> AppElement<'_> {
        let control = t_toggle(
            self.theme_mode,
            self.reopen_last_list,
            Message::SetReopenLastList(!self.reopen_last_list),
        );
        self.setting_row(
            "Reopen last list",
            "Open the list you were last using when Taskscape starts.",
            control,
        )
    }

    fn confirm_clear_setting(&self) -> AppElement<'_> {
        let control = t_toggle(
            self.theme_mode,
            self.confirm_clear_all,
            Message::SetConfirmClearAll(!self.confirm_clear_all),
        );
        self.setting_row(
            "Confirm “Clear all”",
            "Ask before removing every task in a list.",
            control,
        )
    }
}
