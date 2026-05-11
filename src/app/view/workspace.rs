use crate::app::{AppElement, Taskscape};
use crate::models::Task;
use crate::thememanager::{empty_state_container, panel_alt_container, tokens};
use crate::widgets::small_chip;
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row, text};

impl Taskscape {
    pub(crate) fn workspace_panel<'a>(&'a self, tasks: &[&'a Task]) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);

        if tasks.is_empty() {
            container(
                column![
                    text("Your runway is clear").size(30).style(palette.text_primary),
                    text("Use the composer, import panel, or saved filters to build your workspace.")
                        .size(17)
                        .style(palette.text_secondary),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fixed(280.0))
            .center_x()
            .center_y()
            .style(empty_state_container(self.theme_mode))
            .into()
        } else {
            let list = tasks.iter().fold(column![].spacing(12), |column, task| {
                column.push(self.task_card(task))
            });

            container(list)
                .width(Length::Fill)
                .style(empty_state_container(self.theme_mode))
                .padding(14)
                .into()
        }
    }

    fn task_card<'a>(&'a self, task: &'a Task) -> AppElement<'a> {
        let palette = tokens(self.theme_mode);

        let chips = task.tags.iter().fold(row![].spacing(6), |row, tag| {
            row.push(small_chip(self.theme_mode, tag, false))
        });

        let meta = row![
            small_chip(self.theme_mode, task.priority.short_label(), true),
            small_chip(self.theme_mode, task.status.label(), false),
            match task.due_date.as_deref() {
                Some(date) => small_chip(self.theme_mode, date, false),
                None => small_chip(self.theme_mode, "No date", false),
            },
        ]
        .spacing(6)
        .align_items(Alignment::Center);

        container(
            column![
                row![
                    column![
                        text(&task.title).size(20).style(palette.text_primary),
                        text("Captured in the current list workspace")
                            .size(14)
                            .style(palette.text_secondary)
                    ]
                    .spacing(4),
                    Space::with_width(Length::Fill),
                    meta,
                ]
                .align_items(Alignment::Center),
                chips,
            ]
            .spacing(12),
        )
        .padding(16)
        .style(panel_alt_container(self.theme_mode))
        .into()
    }
}
