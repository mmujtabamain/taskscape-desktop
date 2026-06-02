use crate::app::{AppElement, Message, Taskscape};
use common::models::Task;
use common::thememanager::{empty_state_container, panel_alt_container, tokens};
use common::widgets::{t_body, t_caption, t_heading, t_icon_button};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, checkbox, column, container, row, scrollable};

impl Taskscape {
    pub(crate) fn task_list_panel(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);
        let tasks = self.visible_tasks();

        if tasks.is_empty() {
            container(
                column![
                    t_heading("No tasks yet", 22.0, palette.text_primary),
                    t_body(
                        "Add a task above to get started.",
                        14.0,
                        palette.text_muted,
                    ),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(empty_state_container(self.theme_mode))
            .into()
        } else {
            let list = tasks
                .iter()
                .fold(column![].spacing(6), |column, (index, task)| {
                    column.push(self.task_card(*index, task))
                });

            let list = scrollable(list).height(Length::Fill);

            container(list)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(empty_state_container(self.theme_mode))
                .padding(8)
                .into()
        }
    }

    fn task_card<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);

        container(
            row![
                checkbox(task.completed)
                    .on_toggle(move |completed| Message::ToggleTaskCompleted(index, completed))
                    .size(16),
                column![
                    t_body(&task.title, 15.0, palette.text_primary),
                    t_caption(
                        if task.completed { "Completed" } else { "Open" },
                        12.0,
                        palette.text_muted,
                    )
                ]
                .spacing(1),
                Space::new().width(Length::Fill),
                t_icon_button(
                    self.theme_mode,
                    lucide_icons::Icon::Trash2,
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
