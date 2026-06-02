//! The compact "mini" window shown from the menu-bar icon.
//!
//! A deliberately dense, single-purpose surface — tighter than the main window:
//! a slim header (brand + counts + quit), a one-line composer, a scrollable list
//! of compact task rows, and a thin footer showing the link status.

use crate::app::{AppElement, Message, TrayApp};
use common::models::Task;
use common::thememanager::{
    ButtonKind, mini_chip, mini_row, mini_shell_container, tokens,
};
use common::widgets::{lucide_icon, t_body, t_caption, t_icon_button_ghost, t_input_box};
use iced::widget::{Space, button, checkbox, column, container, row, scrollable};
use iced::{Alignment, Length};
use lucide_icons::Icon;

impl TrayApp {
    pub(crate) fn mini_view(&self) -> AppElement<'_> {
        let content = column![
            self.mini_header(),
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

    /// Slim header: brand glyph + title, a remaining-count pill, and quit.
    fn mini_header(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let remaining = self.tasks.open();

        let brand = row![
            lucide_icon(Icon::ListTodo, 16.0, palette.accent),
            t_body("Taskscape", 15.0, palette.text_primary),
        ]
        .spacing(7)
        .align_y(Alignment::Center);

        let count_pill = container(t_caption(
            format!("{remaining} left"),
            11.0,
            palette.accent,
        ))
        .padding([2, 7])
        .style(mini_chip(self.theme_mode));

        row![
            brand,
            Space::new().width(Length::Fill),
            count_pill,
            // Quit the background service.
            t_icon_button_ghost(self.theme_mode, Icon::Power, Some(Message::QuitRequested)),
        ]
        .align_y(Alignment::Center)
        .spacing(6)
        .into()
    }

    /// One-line composer: compact input + icon-only add button.
    fn mini_composer(&self) -> AppElement<'_> {
        row![
            t_input_box(
                self.theme_mode,
                "Add a task…",
                &self.title_input,
                Message::TitleChanged,
                Length::Fill,
                Some(Message::AddTask),
            ),
            // Icon-only add (no label) to stay compact.
            container(t_icon_button_ghost(
                self.theme_mode,
                Icon::Plus,
                Some(Message::AddTask),
            ))
            .style(common::thememanager::mini_chip(self.theme_mode)),
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
    }

    fn mini_task_list(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let tasks = self.tasks.enumerated();

        if tasks.is_empty() {
            return container(
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
            .into();
        }

        let list = tasks
            .iter()
            .fold(column![].spacing(5), |col, (index, task)| {
                col.push(self.mini_task_row(*index, task))
            });

        scrollable(list).height(Length::Fill).into()
    }

    fn mini_task_row<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);
        let title_color = if task.completed {
            palette.text_muted
        } else {
            palette.text_primary
        };

        // The title is a transparent button so clicking the row text toggles
        // completion (a quick, low-friction interaction in the mini window).
        let title_button = button(t_body(&task.title, 14.0, title_color).width(Length::Fill))
            .style(common::thememanager::button_style(
                self.theme_mode,
                ButtonKind::Plain,
            ))
            .width(Length::Fill)
            .padding(0)
            .on_press(Message::ToggleTaskCompleted(index, !task.completed));

        container(
            row![
                checkbox(task.completed)
                    .on_toggle(move |c| Message::ToggleTaskCompleted(index, c))
                    .size(15),
                title_button,
                t_icon_button_ghost(self.theme_mode, Icon::Trash2, Some(Message::RemoveTask(index))),
            ]
            .align_y(Alignment::Center)
            .spacing(8),
        )
        .padding([5, 8])
        .style(mini_row(self.theme_mode, task.completed))
        .into()
    }

    /// Thin footer: link status with a coloured dot.
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
}
