//! The list sidebar — a compact collapsed rail that expands into a full panel —
//! plus the create-or-load empty-state prompt shown when no list is open.

use crate::app::{AppElement, Message, Taskscape};
use common::thememanager::{
    ButtonKind, ThemeMode, button_style, empty_state_container, list_row_container,
    sidebar_container, tokens,
};
use common::widgets::{lucide_icon, t_body, t_button, t_caption, t_heading, t_icon_button, t_input_box};
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Length};
use lucide_icons::Icon;

const PANEL_WIDTH: f32 = 248.0;
const RAIL_WIDTH: f32 = 52.0;
/// Square cell size for items in the collapsed rail (toggle + list chips).
const RAIL_CELL: f32 = 36.0;
/// Vertical gap between rail cells.
const RAIL_GAP: f32 = 6.0;

/// A lucide glyph sized for a rail cell, in the secondary text colour.
fn rail_glyph(mode: ThemeMode, icon: Icon) -> AppElement<'static> {
    lucide_icon(icon, 17.0, tokens(mode).text_secondary).into()
}

/// A list's initial, accent-coloured when it is the current list.
fn rail_initial(mode: ThemeMode, initial: &str, is_current: bool) -> AppElement<'static> {
    let palette = tokens(mode);
    let color = if is_current {
        palette.accent
    } else {
        palette.text_secondary
    };
    text(initial.to_owned()).size(15.0).color(color).into()
}

/// Button style for a rail cell: an accent-tinted fill when selected, a subtle
/// raised fill otherwise, with a matching border. Drawn on the button directly
/// so the cell stays a fixed square.
fn rail_cell_button_style(
    mode: ThemeMode,
    selected: bool,
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style + Clone {
    use common::thememanager::helpers::{border, with_alpha};
    move |_theme, status| {
        let palette = tokens(mode);
        let (bg, border_color) = if selected {
            (with_alpha(palette.accent, 0.16), with_alpha(palette.accent, 0.55))
        } else {
            (with_alpha(palette.panel_raised, 0.5), palette.border)
        };
        let mut style = iced::widget::button::Style {
            background: Some(bg.into()),
            text_color: palette.text_primary,
            border: border(10.0, 1.0, border_color),
            ..Default::default()
        };
        if matches!(status, iced::widget::button::Status::Hovered) && !selected {
            style.border.color = palette.border_strong;
        }
        style
    }
}

impl Taskscape {
    /// The sidebar: a narrow rail when collapsed, the full panel when expanded.
    pub(crate) fn list_sidebar(&self) -> AppElement<'_> {
        if self.show_list_panel {
            self.list_panel()
        } else {
            self.list_rail()
        }
    }

    /// Collapsed rail: a toggle button on top, then one compact chip per list
    /// (its initial). Clicking the toggle expands; clicking a chip opens that
    /// list (and expands so the user sees the result).
    fn list_rail(&self) -> AppElement<'_> {
        // Toggle to expand the panel, then one chip per list — all the same
        // square cell so they form a clean, centered vertical stack.
        let toggle = self.rail_cell(
            rail_glyph(self.theme_mode, Icon::PanelLeftOpen),
            false,
            Message::ToggleListPanel,
        );

        let chips = self.lists.iter().fold(
            column![].spacing(RAIL_GAP).align_x(Alignment::Center),
            |col, entry| {
                let is_current = self.current_list.as_deref() == Some(entry.name.as_str());
                let initial = entry
                    .name
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default();
                col.push(self.rail_cell(
                    rail_initial(self.theme_mode, &initial, is_current),
                    is_current,
                    Message::OpenList(entry.name.clone()),
                ))
            },
        );

        container(
            column![toggle, Space::new().height(Length::Fixed(2.0)), chips]
                .spacing(RAIL_GAP)
                .align_x(Alignment::Center),
        )
        .width(Length::Fixed(RAIL_WIDTH))
        .height(Length::Fill)
        .padding([10, 0])
        .style(sidebar_container(self.theme_mode))
        .into()
    }

    /// One uniform, centered, clickable square cell in the collapsed rail.
    fn rail_cell<'a>(
        &self,
        content: AppElement<'a>,
        selected: bool,
        on_press: Message,
    ) -> AppElement<'a> {
        // The styled box is the button itself, pinned to a fixed square so cells
        // don't stretch to fill the column's height.
        button(
            container(content)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fixed(RAIL_CELL))
        .height(Length::Fixed(RAIL_CELL))
        .padding(0)
        .style(rail_cell_button_style(self.theme_mode, selected))
        .on_press(on_press)
        .into()
    }

    /// Expanded panel listing all task lists with row actions.
    fn list_panel(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let header = row![
            t_heading("Lists", 18.0, palette.text_primary),
            Space::new().width(Length::Fill),
            t_icon_button(self.theme_mode, Icon::Import, None, Some(Message::ImportList)),
            t_icon_button(
                self.theme_mode,
                Icon::PanelLeftClose,
                None,
                Some(Message::ToggleListPanel),
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(6);

        // "New list" name input + create button.
        let new_row = row![
            t_input_box(
                self.theme_mode,
                "New list…",
                &self.new_list_name,
                Message::NewListNameChanged,
                Length::Fill,
                Some(Message::CreateList),
            ),
            t_icon_button(self.theme_mode, Icon::Plus, None, Some(Message::CreateList)),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let rows = self.lists.iter().fold(column![].spacing(5), |col, entry| {
            col.push(self.list_row(entry))
        });

        let body: AppElement<'_> = if self.lists.is_empty() {
            container(t_body(
                "No lists yet. Create one above.",
                13.0,
                palette.text_muted,
            ))
            .width(Length::Fill)
            .padding([10, 4])
            .into()
        } else {
            scrollable(rows).height(Length::Fill).into()
        };

        container(
            column![header, new_row, body]
                .spacing(10)
                .height(Length::Fill),
        )
        .width(Length::Fixed(PANEL_WIDTH))
        .height(Length::Fill)
        .padding(10)
        .style(sidebar_container(self.theme_mode))
        .into()
    }

    /// A single panel row: open/rename/delete, or an inline rename editor.
    fn list_row<'a>(&'a self, entry: &'a common::storage::ListEntry) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);
        let is_current = self.current_list.as_deref() == Some(entry.name.as_str());

        // Inline rename editor for the list being renamed.
        if let Some((old_name, new_name)) = &self.renaming {
            if old_name == &entry.name {
                let editor = row![
                    t_input_box(
                        self.theme_mode,
                        "Name…",
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
                    t_icon_button(self.theme_mode, Icon::X, None, Some(Message::CancelRenameList)),
                ]
                .spacing(5)
                .align_y(Alignment::Center);

                return container(editor)
                    .padding([5, 6])
                    .style(list_row_container(self.theme_mode, true))
                    .into();
            }
        }

        let name_color = if is_current {
            palette.accent
        } else {
            palette.text_primary
        };

        let label = column![
            t_body(&entry.name, 14.0, name_color),
            t_caption(format!("{} tasks", entry.task_count), 11.0, palette.text_muted),
        ]
        .spacing(1);

        // Clicking the label area opens the list. Transparent so only the row
        // container provides the background (no redundant box behind the title).
        let open_button = button(label)
            .style(button_style(self.theme_mode, ButtonKind::Plain))
            .width(Length::Fill)
            .padding([3, 5])
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
        .spacing(2)
        .align_y(Alignment::Center);

        container(
            row![open_button, actions]
                .align_y(Alignment::Center)
                .spacing(4),
        )
        .padding([4, 6])
        .style(list_row_container(self.theme_mode, is_current))
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
                Length::Fixed(240.0),
                Some(Message::CreateList),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::ListPlus),
                "Create",
                ButtonKind::Primary,
                Some(Message::CreateList),
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let load = row![
            t_button(
                self.theme_mode,
                Some(Icon::PanelLeftOpen),
                "Browse lists",
                ButtonKind::Ghost,
                Some(Message::ToggleListPanel),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::Import),
                "Import JSON",
                ButtonKind::Ghost,
                Some(Message::ImportList),
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        container(
            column![
                t_heading("Welcome to Taskscape", 26.0, palette.text_primary),
                t_body(
                    "Create a new task list, or load an existing one.",
                    14.0,
                    palette.text_secondary,
                ),
                Space::new().height(Length::Fixed(6.0)),
                create,
                load,
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(20)
        .style(empty_state_container(self.theme_mode))
        .into()
    }
}
