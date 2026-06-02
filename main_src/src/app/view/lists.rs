//! The list sidebar (browse / create / rename / delete / import / export) and
//! the create-or-load empty-state prompt shown when no list is open.

use crate::app::{AppElement, Message, Taskscape};
use common::thememanager::{ButtonKind, empty_state_container, panel_alt_container, tokens};
use common::widgets::{t_body, t_button, t_caption, t_heading, t_icon_button, t_input_box};
use iced::widget::{Space, column, container, row, scrollable};
use iced::{Alignment, Length};
use lucide_icons::Icon;

const PANEL_WIDTH: f32 = 280.0;

impl Taskscape {
    /// The left sidebar listing all task lists with row actions.
    pub(crate) fn list_panel(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let header = row![
            t_heading("Lists", 22.0, palette.text_primary),
            Space::new().width(Length::Fill),
            t_icon_button(self.theme_mode, Icon::Import, None, Some(Message::ImportList)),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        // "New list" name input + create button.
        let new_row = row![
            t_input_box(
                self.theme_mode,
                "New list name…",
                &self.new_list_name,
                Message::NewListNameChanged,
                Length::Fill,
                Some(Message::CreateList),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::ListPlus),
                "Add",
                ButtonKind::Primary,
                Some(Message::CreateList),
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let rows = self.lists.iter().fold(column![].spacing(8), |col, entry| {
            col.push(self.list_row(entry))
        });

        let body: AppElement<'_> = if self.lists.is_empty() {
            container(t_body(
                "No lists yet. Create one above.",
                14.0,
                palette.text_secondary,
            ))
            .width(Length::Fill)
            .padding([12, 4])
            .into()
        } else {
            scrollable(rows).height(Length::Fill).into()
        };

        container(
            column![header, new_row, body]
                .spacing(12)
                .height(Length::Fill),
        )
        .width(Length::Fixed(PANEL_WIDTH))
        .height(Length::Fill)
        .padding(14)
        .style(panel_alt_container(self.theme_mode))
        .into()
    }

    /// A single sidebar row: either the normal open/rename/delete row, or an
    /// inline rename editor when this list is being renamed.
    fn list_row<'a>(&'a self, entry: &'a common::storage::ListEntry) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);
        let is_current = self.current_list.as_deref() == Some(entry.name.as_str());

        // Inline rename editor for the list being renamed.
        if let Some((old_name, new_name)) = &self.renaming {
            if old_name == &entry.name {
                let editor = row![
                    t_input_box(
                        self.theme_mode,
                        "List name…",
                        new_name,
                        Message::RenameInputChanged,
                        Length::Fill,
                        Some(Message::CommitRenameList),
                    ),
                    t_icon_button(
                        self.theme_mode,
                        Icon::Check,
                        None,
                        Some(Message::CommitRenameList),
                    ),
                    t_icon_button(
                        self.theme_mode,
                        Icon::X,
                        None,
                        Some(Message::CancelRenameList),
                    ),
                ]
                .spacing(6)
                .align_y(Alignment::Center);

                return container(editor)
                    .padding([8, 10])
                    .style(panel_alt_container(self.theme_mode))
                    .into();
            }
        }

        let name_color = if is_current {
            palette.accent_text
        } else {
            palette.text_primary
        };

        let label = column![
            t_body(&entry.name, 15.0, name_color),
            t_caption(
                format!("{} tasks", entry.task_count),
                12.0,
                palette.text_muted,
            ),
        ]
        .spacing(2);

        // Clicking the label area opens the list.
        let open_button = iced::widget::button(label)
            .style(common::thememanager::button_style(
                self.theme_mode,
                ButtonKind::Ghost,
            ))
            .width(Length::Fill)
            .padding([4, 6])
            .on_press(Message::OpenList(entry.name.clone()));

        let actions = row![
            t_icon_button(
                self.theme_mode,
                Icon::Pencil,
                None,
                Some(Message::StartRenameList(entry.name.clone())),
            ),
            t_icon_button(
                self.theme_mode,
                Icon::Trash2,
                None,
                Some(Message::DeleteList(entry.name.clone())),
            ),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        container(
            row![open_button, actions]
                .align_y(Alignment::Center)
                .spacing(6),
        )
        .padding([6, 8])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }

    /// The create-or-load prompt shown in the main area when no list is open.
    pub(crate) fn empty_state_prompt(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let create = row![
            t_input_box(
                self.theme_mode,
                "Name your first list…",
                &self.new_list_name,
                Message::NewListNameChanged,
                Length::Fixed(280.0),
                Some(Message::CreateList),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::ListPlus),
                "Create list",
                ButtonKind::Primary,
                Some(Message::CreateList),
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        let load = row![
            t_button(
                self.theme_mode,
                Some(Icon::PanelLeft),
                "Browse lists",
                ButtonKind::Ghost,
                Some(Message::ToggleListPanel),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::Import),
                "Import from JSON",
                ButtonKind::Ghost,
                Some(Message::ImportList),
            ),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        container(
            column![
                t_heading("Welcome to Taskscape", 30.0, palette.text_primary),
                t_body(
                    "Create a new task list to get started, or load an existing one.",
                    16.0,
                    palette.text_secondary,
                ),
                Space::new().height(Length::Fixed(8.0)),
                create,
                load,
            ]
            .spacing(14)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(24)
        .style(empty_state_container(self.theme_mode))
        .into()
    }
}
