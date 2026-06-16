//! The compact "mini" window shown from the menu-bar icon, plus the small
//! quit-confirmation popover rendered in its own window.
//!
//! The mini surface is deliberately dense — tighter than the main window: a slim
//! header (clickable list title + quit), a one-line composer, a bordered panel
//! holding the scrollable task list, and a thin footer showing the link status.

use crate::app::{AppElement, Message, TrayApp};
use common::models::Task;
use common::thememanager::{
    ButtonKind, button_style, empty_state_container, mini_shell_container, panel_alt_container,
    text_input_style, tokens,
};
use common::widgets::{lucide_icon, t_body, t_button, t_caption, t_heading, t_icon_button};
use iced::widget::{Space, button, checkbox, column, container, mouse_area, row, scrollable, text_input};
use iced::{Alignment, Length};
use lucide_icons::Icon;

/// Stable id for the mini window's task input, so it can be focused on open.
pub(crate) const MINI_INPUT_ID: &str = "mini-task-input";

impl TrayApp {
    pub(crate) fn mini_view(&self) -> AppElement<'_> {
        // The window is borderless, so the header doubles as a drag handle:
        // dragging its empty space moves the window, while the nested title and
        // quit buttons still capture their own clicks.
        let content = column![
            mouse_area(self.mini_header()).on_press(Message::DragMini),
            self.mini_composer(),
            self.mini_task_list(),
            self.mini_footer(),
        ]
        .spacing(8)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(mini_shell_container(self.theme_mode))
            .into()
    }

    /// Slim header: the open list's name (clicking it opens the main app's list
    /// sidebar), plus the quit button.
    fn mini_header(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let title = self.current_list.as_deref().unwrap_or("No list");

        // The title is a borderless button: click to bring the main app forward
        // and open its list sidebar (launching it if it is closed).
        let title_button = button(
            row![
                t_body(title, 15.0, palette.text_primary),
                lucide_icon(Icon::ChevronDown, 14.0, palette.text_muted),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
        )
        .style(button_style(self.theme_mode, ButtonKind::Plain))
        .padding([2, 4])
        .on_press(Message::ShowMainRequested);

        row![
            title_button,
            Space::new().width(Length::Fill),
            t_icon_button(self.theme_mode, Icon::Power, None, Some(Message::QuitRequested)),
        ]
        .align_y(Alignment::Center)
        .spacing(6)
        .into()
    }

    /// One-line composer: compact input + an add button matching the main app's
    /// icon buttons.
    fn mini_composer(&self) -> AppElement<'_> {
        // Built inline (rather than via `t_input_box`) so it can carry an id and
        // be focused when the window opens — mirroring the main app's rename input.
        let input = text_input("Add a task…", &self.title_input)
            .id(MINI_INPUT_ID)
            .width(Length::Fill)
            .padding([12, 14])
            .size(16)
            .on_input(Message::TitleChanged)
            .on_submit(Message::AddTask)
            .style(text_input_style(self.theme_mode));

        row![
            input,
            t_icon_button(self.theme_mode, Icon::Plus, None, Some(Message::AddTask)),
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
    }

    /// The task list, wrapped in a bordered panel.
    fn mini_task_list(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let tasks = self.tasks.enumerated();

        let inner: AppElement<'_> = if tasks.is_empty() {
            container(
                column![
                    lucide_icon(Icon::ListTodo, 26.0, palette.text_muted),
                    t_caption("No tasks yet", 13.0, palette.text_secondary),
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        } else {
            let list = tasks
                .iter()
                .fold(column![].spacing(5), |col, (index, task)| {
                    col.push(self.mini_task_row(*index, task))
                })
                // Reserve a gutter on the right so the scrollbar doesn't overlap
                // the rows' trash buttons.
                .padding(iced::Padding::ZERO.right(10.0));
            scrollable(list).height(Length::Fill).into()
        };

        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(7)
            .style(empty_state_container(self.theme_mode))
            .into()
    }

    fn mini_task_row<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);

        container(
            row![
                // Completion toggles only via the checkbox.
                checkbox(task.completed)
                    .on_toggle(move |c| Message::ToggleTaskCompleted(index, c))
                    .size(16),
                t_body(&task.title, 14.0, palette.text_primary).width(Length::Fill),
                t_icon_button(
                    self.theme_mode,
                    Icon::Trash2,
                    None,
                    Some(Message::RemoveTask(index)),
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(8),
        )
        .padding([6, 9])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }

    /// Thin footer: link status with a coloured dot + total task count.
    fn mini_footer(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let (dot, label) = if self.ipc_connected {
            (palette.accent, "Linked")
        } else {
            (palette.text_muted, "Standalone")
        };

        row![
            lucide_icon(Icon::Dot, 16.0, dot),
            t_caption(label, 11.0, palette.text_muted),
            Space::new().width(Length::Fill),
            t_caption(format!("{} tasks", self.tasks.total()), 11.0, palette.text_muted),
        ]
        .align_y(Alignment::Center)
        .spacing(2)
        .into()
    }

    /// The contents of the standalone "Quit Taskscape?" confirmation popover
    /// window.
    pub(crate) fn quit_confirm_view(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let card = column![
            t_heading("Quit Taskscape?", 17.0, palette.text_primary),
            t_caption(
                "The menu-bar icon and mini window will close.",
                12.0,
                palette.text_secondary,
            ),
            Space::new().height(Length::Fill),
            row![
                Space::new().width(Length::Fill),
                t_button(
                    self.theme_mode,
                    None,
                    "Cancel",
                    ButtonKind::Ghost,
                    Some(Message::CancelQuit),
                ),
                t_button(
                    self.theme_mode,
                    Some(Icon::Power),
                    "Quit",
                    ButtonKind::Primary,
                    Some(Message::ConfirmQuit),
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(8);

        let panel = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(14)
            .style(mini_shell_container(self.theme_mode));

        // The window is borderless (no title bar), so make the whole card a drag
        // handle. The nested buttons capture their own clicks (Cancel/Quit still
        // work); dragging anywhere else moves the window.
        mouse_area(panel)
            .on_press(Message::DragConfirm)
            .into()
    }
}
