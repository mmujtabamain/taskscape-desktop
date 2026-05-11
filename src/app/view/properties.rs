use crate::app::{AppElement, Message, Taskscape};
use crate::thememanager::{ThemeMode, panel_container, shell_container, tokens};
use crate::widgets::{info_card, radio_group, section_heading};
use iced::widget::{column, container, row, text};

const THEME_OPTIONS: [ThemeMode; 2] = [ThemeMode::Dark, ThemeMode::Light];

impl Taskscape {
    pub(crate) fn properties_view(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let properties = column![
            self.header(
                "WORKSPACE PROPERTIES",
                "Collections",
                "Manage appearance, persistence and workflow controls for TaskScape.",
            ),
            container(
                column![
                    section_heading(self.theme_mode, "Appearance"),
                    text("Global theme is shared across the dashboard, fields, buttons, sidebar and overlays.")
                        .size(15)
                        .style(palette.text_secondary),
                    radio_group(
                        self.theme_mode,
                        &THEME_OPTIONS,
                        self.theme_mode,
                        ThemeMode::label,
                        Message::SetThemeMode,
                    ),
                ]
                .spacing(18)
                .padding(22),
            )
            .style(panel_container(self.theme_mode)),
            row![
                info_card(
                    self.theme_mode,
                    "Persistence",
                    "Lists stay lightweight and local-first, with room for import and export flows.",
                ),
                info_card(
                    self.theme_mode,
                    "Custom widgets",
                    "Buttons, dropdowns, inputs, radio controls and sidebar tiles all share one design system.",
                ),
            ]
            .spacing(14),
            info_card(
                self.theme_mode,
                "Design language",
                "Warm editorial gradients, softened borders and a palette-led dark/light mode keep the desktop app aligned with the supplied reference images.",
            ),
        ]
        .spacing(18)
        .padding([22, 24, 30, 24]);

        container(properties)
            .width(iced::Length::Fill)
            .style(shell_container(self.theme_mode))
            .into()
    }
}
