use crate::app::{AppElement, Message, Taskscape};
use crate::models::Task;
use crate::thememanager::{empty_state_container, panel_alt_container, tokens};
use crate::widgets::{body, heading};
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
                    heading("No tasks yet", 30.0, palette.text_primary),
                    body(
                        "Create a task or load a CSV todo file.",
                        17.0,
                        palette.text_secondary,
                    ),
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fixed(280.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(empty_state_container(self.theme_mode))
            .into()
        } else {
            let list = tasks
                .iter()
                .fold(column![].spacing(12), |column, (index, task)| {
                    column.push(self.task_card(*index, task))
                });

            let list = scrollable(list).height(Length::Fill);

            container(list)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(empty_state_container(self.theme_mode))
                .padding(14)
                .into()
        }
    }

    fn task_card<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);

        container(
            column![
                row![
                    checkbox(task.completed)
                        .on_toggle(move |completed| Message::ToggleTaskCompleted(index, completed))
                        .size(18),
                    column![
                        heading(&task.title, 20.0, palette.text_primary),
                        body(
                            if task.completed { "Completed" } else { "Open" },
                            14.0,
                            palette.text_secondary,
                        )
                    ]
                    .spacing(4),
                    Space::new().width(Length::Fill)
                ]
                .align_y(Alignment::Center)
                .spacing(12),
            ]
            .spacing(12),
        )
        .padding(16)
        .style(panel_alt_container(self.theme_mode))
        .into()
    }
}
