//! The list sidebar — a compact collapsed rail that expands into a full panel —
//! plus the create-or-load empty-state prompt shown when no list is open.

use crate::app::{AppElement, Message, Taskscape};
use common::thememanager::{
    ButtonKind, ThemeMode, empty_state_container, modal_backdrop, modal_card, sidebar_container,
    text_input_style, tokens,
};
use common::widgets::{
    lucide_icon, t_body, t_button, t_heading, t_icon_button, t_icon_button_ghost, t_input_box,
};
use iced::widget::{
    Space, button, center, column, container, mouse_area, opaque, row, scrollable, text,
    text_input,
};
use iced::{Alignment, Length};
use lucide_icons::Icon;

/// Id of the rename modal's text input, so it can be focused when the modal opens.
pub const RENAME_INPUT_ID: &str = "rename-modal-input";

const PANEL_WIDTH: f32 = 248.0;
const RAIL_WIDTH: f32 = 58.0;
/// Fixed height of every sidebar member — the rail cells *and* the panel's
/// header / new-list / list rows — so toggling collapsed↔expanded causes no
/// vertical reflow. Sized to comfortably contain the tallest member (the text
/// input, ≈42px) without clipping. Also the rail cell's width (square).
const ROW_H: f32 = 42.0;
const RAIL_CELL: f32 = ROW_H;
/// Vertical gap between sidebar members, identical in both states.
const RAIL_GAP: f32 = 8.0;
/// Horizontal padding inside a panel list row.
const ROW_PAD_X: u16 = 8;

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

/// Button style for a rail cell. The non-selected look is kept *identical* to
/// the expanded panel's `ButtonKind::Icon` buttons (same fill, border, radius,
/// and hover) so collapsed and expanded share the exact same shades; the
/// selected cell adds an accent tint.
fn rail_cell_button_style(
    mode: ThemeMode,
    selected: bool,
) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style + Clone {
    use common::thememanager::helpers::{border, mix, with_alpha};
    move |_theme, status| {
        let palette = tokens(mode);
        let (bg, border_color) = if selected {
            (with_alpha(palette.accent, 0.16), with_alpha(palette.accent, 0.55))
        } else {
            // Matches ButtonKind::Icon exactly.
            (palette.panel_raised, palette.border)
        };
        let mut style = iced::widget::button::Style {
            background: Some(bg.into()),
            text_color: palette.text_primary,
            border: border(12.0, 1.0, border_color),
            ..Default::default()
        };
        if matches!(status, iced::widget::button::Status::Hovered) && !selected {
            // Same hover as ButtonKind::Icon.
            style.background = Some(mix(palette.panel_raised, palette.panel_alt, 0.55).into());
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
        // Toggle to expand the panel, a "+" to start a new list, then one chip
        // per list — all the same fixed square cell, centered in the rail.
        let mut stack = column![
            self.rail_cell(
                rail_glyph(self.theme_mode, Icon::PanelLeftOpen),
                false,
                Message::ToggleListPanel,
            ),
            self.rail_cell(
                rail_glyph(self.theme_mode, Icon::Plus),
                false,
                // Expands the panel where the name input lives (future: focus +
                // animate the create flow).
                Message::ToggleListPanel,
            ),
        ]
        .spacing(RAIL_GAP)
        .width(Length::Fill)
        .align_x(Alignment::Center);

        for entry in &self.lists {
            let is_current = self.current_list.as_deref() == Some(entry.name.as_str());
            let initial = entry
                .name
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_default();
            stack = stack.push(self.rail_cell(
                rail_initial(self.theme_mode, &initial, is_current),
                is_current,
                Message::OpenList(entry.name.clone()),
            ));
        }

        container(stack)
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

        // Fixed-height header so it lines up exactly with the rail's first cell.
        let header = container(
            row![
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
            .spacing(6),
        )
        .height(Length::Fixed(ROW_H))
        .align_y(Alignment::Center);

        // "New list" name input + create button — fixed height (matches the
        // rail's "+" cell position).
        let new_row = container(
            row![
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
            .align_y(Alignment::Center),
        )
        .height(Length::Fixed(ROW_H))
        .align_y(Alignment::Center);

        let rows = self
            .lists
            .iter()
            .fold(column![].spacing(RAIL_GAP), |col, entry| {
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
                .spacing(RAIL_GAP)
                .height(Length::Fill),
        )
        .width(Length::Fixed(PANEL_WIDTH))
        .height(Length::Fill)
        .padding([10, 10])
        .style(sidebar_container(self.theme_mode))
        .into()
    }

    /// A single panel row: the whole row is one button that opens the list, with
    /// nested ghost rename/delete actions. Renaming happens in a separate modal
    /// (see `rename_modal`), so the row layout is fixed and never reflows.
    fn list_row<'a>(&'a self, entry: &'a common::storage::ListEntry) -> AppElement<'a> {
        let is_current = self.current_list.as_deref() == Some(entry.name.as_str());

        let name_color = if is_current {
            tokens(self.theme_mode).accent
        } else {
            tokens(self.theme_mode).text_primary
        };

        let content = row![
            t_body(&entry.name, 14.0, name_color).width(Length::Fill),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Pencil,
                Some(Message::StartRenameList(entry.name.clone())),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Trash2,
                Some(Message::DeleteList(entry.name.clone())),
            ),
        ]
        // Fill the row's fixed height and centre everything vertically so the
        // name lines up with the action icons.
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(4);

        // The nested ghost actions capture their own clicks (iced updates the
        // child first and bails if it captured), so they don't trigger "open".
        button(content)
            .width(Length::Fill)
            .height(Length::Fixed(ROW_H))
            .padding([0, ROW_PAD_X])
            .style(rail_cell_button_style(self.theme_mode, is_current))
            .on_press(Message::OpenList(entry.name.clone()))
            .into()
    }

    /// The rename modal: a dimmed backdrop (click to cancel) with a centered card
    /// holding the rename input and Cancel / Rename actions. Returned only when a
    /// rename is in progress; stacked over the main UI by `view_root`.
    pub(crate) fn rename_modal(&self) -> Option<AppElement<'_>> {
        let (old_name, new_name) = self.renaming.as_ref()?;
        let palette = tokens(self.theme_mode);

        let card = container(
            column![
                t_heading("Rename list", 20.0, palette.text_primary),
                t_body(
                    format!("Renaming \"{old_name}\""),
                    13.0,
                    palette.text_muted,
                ),
                text_input("List name…", new_name)
                    .id(RENAME_INPUT_ID)
                    .width(Length::Fill)
                    .padding([12, 14])
                    .size(16)
                    .on_input(Message::RenameInputChanged)
                    .on_submit(Message::CommitRenameList)
                    .style(text_input_style(self.theme_mode)),
                row![
                    Space::new().width(Length::Fill),
                    t_button(
                        self.theme_mode,
                        None,
                        "Cancel",
                        ButtonKind::Ghost,
                        Some(Message::CancelRenameList),
                    ),
                    t_button(
                        self.theme_mode,
                        Some(Icon::Check),
                        "Rename",
                        ButtonKind::Primary,
                        Some(Message::CommitRenameList),
                    ),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            ]
            .spacing(14),
        )
        .width(Length::Fixed(380.0))
        .padding(20)
        .style(modal_card(self.theme_mode));

        // Backdrop fills the window, dims the UI, and cancels on click; the card
        // is centered and swallows its own clicks (it is interactive content).
        let backdrop = mouse_area(
            container(center(card))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(modal_backdrop(self.theme_mode)),
        )
        .on_press(Message::CancelRenameList);

        Some(opaque(backdrop))
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
