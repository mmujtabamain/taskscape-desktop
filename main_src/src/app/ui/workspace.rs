use crate::app::{AppElement, AttachTarget, Message, Taskscape};
use common::models::Task;
use common::ui::tokens::{radius, space, text};
use common::ui::{
    Icon, Interactive, Surface, SurfaceStyle, palette, surface, t_attachment_chip, t_body,
    t_caption, t_checkbox, t_heading, t_icon_button_ghost, with_alpha,
};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row, scrollable};

impl Taskscape {
    pub(crate) fn task_list_panel(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);
        let tasks = self.visible_tasks();

        if tasks.is_empty() {
            container(
                column![
                    t_heading("No tasks yet", text::HEADING, p.text),
                    t_body("Add a task above to get started.", text::BODY, p.text_muted),
                ]
                .spacing(space::SM)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(surface(self.theme_mode))
            .into()
        } else {
            let list = tasks.iter().fold(column![].spacing(space::XS), |column, (index, task)| {
                column.push(self.task_card(*index, task))
            });

            container(scrollable(list).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(surface(self.theme_mode))
                .padding(space::MD)
                .into()
        }
    }

    fn task_card<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let p = palette(self.theme_mode);

        let title_color = if task.completed { p.text_muted } else { p.text };
        let title_block = column![
            t_body(&task.title, text::BODY, title_color),
            t_caption(
                if task.completed { "Completed" } else { "Open" },
                text::LABEL,
                p.text_muted,
            )
        ]
        .spacing(1);

        let info: AppElement<'a> = if task.attachments.is_empty() {
            title_block.into()
        } else {
            let chips = task.attachments.iter().enumerate().fold(
                row![].spacing(space::SM),
                |chips, (att_index, attachment)| {
                    chips.push(t_attachment_chip(
                        self.theme_mode,
                        attachment,
                        Message::OpenAttachment(attachment.path.clone()),
                        Message::RemoveTaskAttachment { task: index, attachment: att_index },
                    ))
                },
            );
            column![title_block, chips].spacing(space::SM).into()
        };

        let body = row![
            t_checkbox(
                self.theme_mode,
                task.completed,
                Message::ToggleTaskCompleted(index, !task.completed),
                18.0,
            ),
            info,
            Space::new().width(Length::Fill),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Attach,
                Some(Message::AttachFile(AttachTarget::Task(index))),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Camera,
                Some(Message::AttachScreenshot(AttachTarget::Task(index))),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Delete,
                Some(Message::RemoveTask(index)),
            ),
        ]
        .align_y(Alignment::Center)
        .spacing(space::LG);

        // Flat row: no fill at rest, a faint fill on hover (not a stacked card).
        let style = SurfaceStyle {
            rest: Surface::new(with_alpha(p.text, 0.0), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.05), 0.0),
            pressed: Surface::new(with_alpha(p.text, 0.05), 0.0),
            radius: radius::MD,
            ring: None,
        };

        Interactive::new(body, style)
            .width(Length::Fill)
            .padding([8, 10])
            .into()
    }
}
