//! The list sidebar — a compact collapsed rail that expands into a full panel —
//! plus the create-or-load empty-state prompt shown when no list is open.

use crate::app::{AppElement, Message, Taskscape};
use common::ui::tokens::{radius, space, text};
use common::ui::{
    ButtonKind, Icon, Interactive, Surface, SurfaceStyle, ThemeMode, icon, modal_backdrop,
    modal_card, palette, sidebar, surface, t_body, t_button, t_heading, t_icon_button,
    t_icon_button_ghost, t_input_box, text_input_style, with_alpha,
};
use common::utils::fonts::montserrat_regular;
use iced::widget::{Space, center, column, container, mouse_area, opaque, row, scrollable, text_input};
use iced::{Alignment, Length};

/// Id of the rename modal's text input, so it can be focused when the modal opens.
pub const RENAME_INPUT_ID: &str = "rename-modal-input";

const PANEL_WIDTH: f32 = 248.0;
const RAIL_WIDTH: f32 = 58.0;
/// Fixed height of every sidebar member so toggling collapsed↔expanded never
/// reflows. Also the rail cell's (square) side.
const ROW_H: f32 = 42.0;
const RAIL_CELL: f32 = ROW_H;
const RAIL_GAP: f32 = 8.0;

/// The interactive fill ramp for a rail cell / list row: an accent-tinted fill
/// when selected, a faint hover fill otherwise (fill-over-outline; no border).
fn cell_style(mode: ThemeMode, selected: bool) -> SurfaceStyle {
    let p = palette(mode);
    if selected {
        SurfaceStyle {
            rest: Surface::new(with_alpha(p.accent, 0.16), 0.0),
            hover: Surface::new(with_alpha(p.accent, 0.22), 0.0),
            pressed: Surface::new(with_alpha(p.accent, 0.26), 0.0),
            radius: radius::MD,
            ring: None,
        }
    } else {
        SurfaceStyle {
            rest: Surface::new(with_alpha(p.text, 0.0), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.06), 0.0),
            pressed: Surface::new(with_alpha(p.text, 0.10), 0.0),
            radius: radius::MD,
            ring: None,
        }
    }
}

fn rail_glyph(mode: ThemeMode, symbol: Icon) -> AppElement<'static> {
    icon(symbol, 18.0, palette(mode).text_dim).into()
}

fn rail_initial(mode: ThemeMode, initial: &str, is_current: bool) -> AppElement<'static> {
    let p = palette(mode);
    let color = if is_current { p.accent } else { p.text_dim };
    t_body(initial.to_owned(), 15.0, color).into()
}

impl Taskscape {
    pub(crate) fn list_sidebar(&self) -> AppElement<'_> {
        if self.show_list_panel {
            self.list_panel()
        } else {
            self.list_rail()
        }
    }

    fn list_rail(&self) -> AppElement<'_> {
        let mut stack = column![
            self.rail_cell(
                rail_glyph(self.theme_mode, Icon::PanelOpen),
                false,
                Message::ToggleListPanel,
            ),
            self.rail_cell(
                rail_glyph(self.theme_mode, Icon::Add),
                false,
                Message::ToggleListPanel,
            ),
        ]
        .spacing(RAIL_GAP)
        .width(Length::Fill)
        .height(Length::Fill)
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

        stack = stack
            .push(Space::new().height(Length::Fill))
            .push(self.rail_cell(
                rail_glyph(self.theme_mode, Icon::Settings),
                self.show_settings,
                Message::ToggleSettings,
            ));

        container(stack)
            .width(Length::Fixed(RAIL_WIDTH))
            .height(Length::Fill)
            .padding([10, 0])
            .style(sidebar(self.theme_mode))
            .into()
    }

    fn rail_cell<'a>(
        &self,
        content: AppElement<'a>,
        selected: bool,
        on_press: Message,
    ) -> AppElement<'a> {
        Interactive::new(
            container(content).center_x(Length::Fill).center_y(Length::Fill),
            cell_style(self.theme_mode, selected),
        )
        .width(Length::Fixed(RAIL_CELL))
        .height(Length::Fixed(RAIL_CELL))
        .on_press(on_press)
        .into()
    }

    fn list_panel(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        let header = container(
            row![
                t_heading("Lists", text::TITLE, p.text),
                Space::new().width(Length::Fill),
                t_icon_button(self.theme_mode, Icon::Import, None, Some(Message::ImportList)),
                t_icon_button(self.theme_mode, Icon::Settings, None, Some(Message::ToggleSettings)),
                t_icon_button(
                    self.theme_mode,
                    Icon::PanelClose,
                    None,
                    Some(Message::ToggleListPanel),
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(space::SM),
        )
        .height(Length::Fixed(ROW_H))
        .align_y(Alignment::Center);

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
                t_icon_button(self.theme_mode, Icon::Add, None, Some(Message::CreateList)),
            ]
            .spacing(space::SM)
            .align_y(Alignment::Center),
        )
        .height(Length::Fixed(ROW_H))
        .align_y(Alignment::Center);

        let rows = self
            .lists
            .iter()
            .fold(column![].spacing(RAIL_GAP), |col, entry| col.push(self.list_row(entry)));

        let body: AppElement<'_> = if self.lists.is_empty() {
            container(t_body("No lists yet. Create one above.", text::SMALL, p.text_muted))
                .width(Length::Fill)
                .padding([10, 4])
                .into()
        } else {
            scrollable(rows).height(Length::Fill).into()
        };

        container(column![header, new_row, body].spacing(RAIL_GAP).height(Length::Fill))
            .width(Length::Fixed(PANEL_WIDTH))
            .height(Length::Fill)
            .padding([10, 10])
            .style(sidebar(self.theme_mode))
            .into()
    }

    fn list_row<'a>(&'a self, entry: &'a common::storage::ListEntry) -> AppElement<'a> {
        let p = palette(self.theme_mode);
        let is_current = self.current_list.as_deref() == Some(entry.name.as_str());
        let name_color = if is_current { p.accent } else { p.text };

        let content = row![
            container(t_body(&entry.name, text::BODY, name_color))
                .width(Length::Fill)
                .padding([0, 6]),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Edit,
                Some(Message::StartRenameList(entry.name.clone())),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Delete,
                Some(Message::DeleteList(entry.name.clone())),
            ),
        ]
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(space::XS);

        Interactive::new(content, cell_style(self.theme_mode, is_current))
            .width(Length::Fill)
            .height(Length::Fixed(ROW_H))
            .padding([0, 8])
            .on_press(Message::OpenList(entry.name.clone()))
            .into()
    }

    pub(crate) fn rename_modal(&self) -> Option<AppElement<'_>> {
        let (old_name, new_name) = self.renaming.as_ref()?;
        let p = palette(self.theme_mode);

        let field = text_input("List name…", new_name)
            .id(RENAME_INPUT_ID)
            .width(Length::Fill)
            .padding([12, 14])
            .size(15)
            .font(montserrat_regular())
            .on_input(Message::RenameInputChanged)
            .on_submit(Message::CommitRenameList)
            .style(text_input_style(self.theme_mode));

        let card = container(
            column![
                t_heading("Rename list", text::TITLE, p.text),
                t_body(format!("Renaming \"{old_name}\""), text::SMALL, p.text_muted),
                field,
                row![
                    Space::new().width(Length::Fill),
                    t_button(self.theme_mode, None, "Cancel", ButtonKind::Ghost, Some(Message::CancelRenameList)),
                    t_button(
                        self.theme_mode,
                        Some(Icon::Check),
                        "Rename",
                        ButtonKind::Primary,
                        Some(Message::CommitRenameList),
                    ),
                ]
                .spacing(space::MD)
                .align_y(Alignment::Center),
            ]
            .spacing(space::XL),
        )
        .width(Length::Fixed(380.0))
        .padding(space::XXL)
        .style(modal_card(self.theme_mode));

        let backdrop = mouse_area(
            container(center(card))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(modal_backdrop(self.theme_mode)),
        )
        .on_press(Message::CancelRenameList);

        Some(opaque(backdrop))
    }

    pub(crate) fn clear_all_modal(&self) -> Option<AppElement<'_>> {
        if !self.confirming_clear_all {
            return None;
        }
        let p = palette(self.theme_mode);
        let list = self.current_list.as_deref().unwrap_or("this list");

        let card = container(
            column![
                t_heading("Clear all tasks?", text::TITLE, p.text),
                t_body(
                    format!("This removes every task in \"{list}\". You can undo it afterwards."),
                    text::SMALL,
                    p.text_muted,
                ),
                row![
                    Space::new().width(Length::Fill),
                    t_button(self.theme_mode, None, "Cancel", ButtonKind::Ghost, Some(Message::CancelClearAll)),
                    t_button(
                        self.theme_mode,
                        Some(Icon::Delete),
                        "Clear all",
                        ButtonKind::Primary,
                        Some(Message::ClearAll),
                    ),
                ]
                .spacing(space::MD)
                .align_y(Alignment::Center),
            ]
            .spacing(space::XL),
        )
        .width(Length::Fixed(380.0))
        .padding(space::XXL)
        .style(modal_card(self.theme_mode));

        let backdrop = mouse_area(
            container(center(card))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(modal_backdrop(self.theme_mode)),
        )
        .on_press(Message::CancelClearAll);

        Some(opaque(backdrop))
    }

    pub(crate) fn empty_state_prompt(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

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
                Some(Icon::ListAdd),
                "Create",
                ButtonKind::Primary,
                Some(Message::CreateList),
            ),
        ]
        .spacing(space::MD)
        .align_y(Alignment::Center);

        let load = row![
            t_button(
                self.theme_mode,
                Some(Icon::PanelOpen),
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
        .spacing(space::MD)
        .align_y(Alignment::Center);

        container(
            column![
                t_heading("Welcome to Taskscape", text::HEADING, p.text),
                t_body("Create a new task list, or load an existing one.", text::BODY, p.text_dim),
                Space::new().height(Length::Fixed(6.0)),
                create,
                load,
            ]
            .spacing(space::LG)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(space::XXL)
        .style(surface(self.theme_mode))
        .into()
    }
}
