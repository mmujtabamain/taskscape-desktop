//! The compact "mini" window shown from the menu bar icon.
//!
//! It has no title bar (see `mini_window_settings`) and a deliberately small
//! surface: an add-a-task row and a scrollable list where each task can be
//! toggled complete or removed. It shares `tasks` with the main window, so any
//! change here is reflected there immediately.

use crate::app::{AppElement, Message, Taskscape};
use crate::models::Task;
use crate::thememanager::{ButtonKind, panel_alt_container, shell_container, tokens};
use crate::widgets::{t_body, t_button, t_heading, t_icon_button, t_input_box};
use iced::widget::{Space, checkbox, column, container, row, scrollable};
use iced::{Alignment, Length};
use lucide_icons::Icon;

impl Taskscape {
    pub(crate) fn mini_view(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        let title = row![
            t_heading(&self.file_name, 20.0, palette.text_primary),
            Space::new().width(Length::Fill),
            t_body(
                format!("{} tasks", self.total_count()),
                13.0,
                palette.text_muted,
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let composer = row![
            t_input_box(
                self.theme_mode,
                "Add a task…",
                &self.title_input,
                Message::TitleChanged,
                Length::Fill,
                Some(Message::AddTask),
            ),
            t_button(
                self.theme_mode,
                Some(Icon::CirclePlus),
                "Add",
                ButtonKind::Primary,
                Some(Message::AddTask),
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let content = column![title, composer, self.mini_task_list()]
            .spacing(12)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(12)
            .style(shell_container(self.theme_mode))
            .into()
    }

    fn mini_task_list(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let tasks = self.visible_tasks();

        if tasks.is_empty() {
            return container(t_body(
                "No tasks yet. Add one above.",
                14.0,
                palette.text_secondary,
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        let list = tasks
            .iter()
            .fold(column![].spacing(8), |column, (index, task)| {
                column.push(self.mini_task_row(*index, task))
            });

        scrollable(list).height(Length::Fill).into()
    }

    fn mini_task_row<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);

        container(
            row![
                checkbox(task.completed)
                    .on_toggle(move |completed| Message::ToggleTaskCompleted(index, completed))
                    .size(16),
                t_body(&task.title, 15.0, palette.text_primary),
                Space::new().width(Length::Fill),
                t_icon_button(
                    self.theme_mode,
                    Icon::Trash2,
                    None,
                    Some(Message::RemoveTask(index)),
                ),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        )
        .padding([8, 10])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }
}
