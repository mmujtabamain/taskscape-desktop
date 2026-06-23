//! The compact "mini" window shown from the menu-bar icon.
//!
//! A compact floating HUD (Spotlight-like): the window is transparent with rounded
//! corners (clipped via `tray::round_window`), and `mini_shell` paints the solid
//! opaque fill on top. Dense by design — a slim draggable header, a one-line
//! composer, the task list, and a thin footer.

use crate::app::{AppElement, AttachTarget, Message, TrayApp};
use common::models::Task;
use common::ui::tokens::{radius, space, text};
use common::ui::{
    ButtonKind, Icon, Interactive, Surface, SurfaceStyle, icon, mini_shell, palette, surface,
    surface_style, t_attachment_chip, t_body, t_caption, t_checkbox, t_icon_button,
    t_icon_button_ghost, text_input_style, with_alpha,
};
use common::utils::fonts::montserrat_regular;
use iced::widget::{Space, column, container, mouse_area, row, scrollable, text_input};
use iced::{Alignment, Length};

/// Stable id for the mini window's task input, so it can be focused on open.
pub(crate) const MINI_INPUT_ID: &str = "mini-task-input";

/// Corner radius of the frosted mini window (a floating HUD, rounder than the
/// main window's tight panels).
pub(crate) const MINI_RADIUS: f32 = radius::XL;

impl TrayApp {
    pub(crate) fn mini_view(&self) -> AppElement<'_> {
        let content = column![
            mouse_area(self.mini_header()).on_press(Message::DragMini),
            self.mini_composer(),
            self.mini_task_list(),
            self.mini_footer(),
        ]
        .spacing(space::SM)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(space::MD)
            .style(mini_shell(self.theme_mode))
            .into()
    }

    fn mini_header(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);
        let title = self.current_list.as_deref().unwrap_or("No list");

        let title_button = Interactive::new(
            row![
                t_body(title, text::BODY, p.text),
                icon(Icon::ChevronDown, text::BODY, p.text_muted),
            ]
            .spacing(space::XS)
            .align_y(Alignment::Center),
            surface_style(self.theme_mode, ButtonKind::Plain),
        )
        .padding([2, 4])
        .on_press(Message::ShowMainRequested);

        row![title_button, Space::new().width(Length::Fill)]
            .align_y(Alignment::Center)
            .spacing(space::SM)
            .into()
    }

    fn mini_composer(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        let input = text_input("Add a task…", &self.title_input)
            .id(MINI_INPUT_ID)
            .width(Length::Fill)
            .padding([10, 12])
            .size(15)
            .font(montserrat_regular())
            .on_input(Message::TitleChanged)
            .on_submit(Message::AddTask)
            .style(text_input_style(self.theme_mode));

        let input_row = row![
            input,
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Attach,
                Some(Message::AttachFile(AttachTarget::Composer)),
            ),
            t_icon_button_ghost(
                self.theme_mode,
                Icon::Camera,
                Some(Message::AttachScreenshot(AttachTarget::Composer)),
            ),
        ]
        .spacing(space::XS)
        .align_y(Alignment::Center);

        let mut composer = column![input_row].spacing(space::XS);

        if !self.staged_attachments.is_empty() {
            let chips = self.staged_attachments.iter().enumerate().fold(
                row![].spacing(space::XS),
                |chips, (index, attachment)| {
                    chips.push(t_attachment_chip(
                        self.theme_mode,
                        attachment,
                        Message::OpenAttachment(attachment.path.clone()),
                        Message::RemoveStagedAttachment(index),
                    ))
                },
            );
            composer = composer.push(chips);
        }

        composer
            .push(
                row![
                    icon(Icon::Enter, text::CAPTION, p.text_muted),
                    t_caption("Press Enter to add", text::CAPTION, p.text_muted),
                ]
                .spacing(space::XS)
                .align_y(Alignment::Center),
            )
            .into()
    }

    fn mini_task_list(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);
        let tasks = self.tasks.enumerated();

        let inner: AppElement<'_> = if tasks.is_empty() {
            container(
                column![
                    icon(Icon::Checklist, 26.0, p.text_muted),
                    t_caption("No tasks yet", text::SMALL, p.text_dim),
                ]
                .spacing(space::SM)
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
                .fold(column![].spacing(space::XS), |col, (index, task)| {
                    col.push(self.mini_task_row(*index, task))
                })
                .padding(iced::Padding::ZERO.right(8.0));
            scrollable(list).height(Length::Fill).into()
        };

        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(space::SM)
            .style(surface(self.theme_mode))
            .into()
    }

    fn mini_task_row<'a>(&'a self, index: usize, task: &'a Task) -> AppElement<'a> {
        let p = palette(self.theme_mode);
        let title_color = if task.completed { p.text_muted } else { p.text };

        let top = row![
            t_checkbox(
                self.theme_mode,
                task.completed,
                Message::ToggleTaskCompleted(index, !task.completed),
                16.0,
            ),
            t_body(&task.title, text::BODY, title_color).width(Length::Fill),
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
        .spacing(space::SM);

        let mut body = column![top].spacing(space::XS);

        if !task.attachments.is_empty() {
            let chips = task.attachments.iter().enumerate().fold(
                row![].spacing(space::XS),
                |chips, (att_index, attachment)| {
                    chips.push(t_attachment_chip(
                        self.theme_mode,
                        attachment,
                        Message::OpenAttachment(attachment.path.clone()),
                        Message::RemoveTaskAttachment { task: index, attachment: att_index },
                    ))
                },
            );
            body = body.push(chips);
        }

        let style = SurfaceStyle {
            rest: Surface::new(with_alpha(p.text, 0.0), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.06), 0.0),
            pressed: Surface::new(with_alpha(p.text, 0.06), 0.0),
            radius: radius::MD,
            ring: None,
        };

        Interactive::new(body, style)
            .width(Length::Fill)
            .padding([5, 8])
            .into()
    }

    fn mini_footer(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);
        let (dot, label) = if self.ipc_connected {
            (p.accent, "Linked")
        } else {
            (p.text_muted, "Standalone")
        };

        row![
            icon(Icon::Dot, 14.0, dot),
            t_caption(label, text::CAPTION, p.text_muted),
            t_caption(format!("{} tasks", self.tasks.total()), text::CAPTION, p.text_muted),
            Space::new().width(Length::Fill),
            t_icon_button(self.theme_mode, Icon::Power, None, Some(Message::QuitRequested)),
        ]
        .align_y(Alignment::Center)
        .spacing(space::SM)
        .into()
    }
}
