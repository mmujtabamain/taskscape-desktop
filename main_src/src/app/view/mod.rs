mod header;
mod lists;
mod tasks;
mod workspace;

pub(crate) use lists::RENAME_INPUT_ID;

use crate::app::{AppElement, Taskscape};
use common::thememanager::{panel_alt_container, shell_container, tokens};
use common::widgets::{t_body, t_caption};
use iced::widget::{column, container, row, stack};
use iced::{Alignment, Length};

impl Taskscape {
    pub(crate) fn view_root(&self) -> AppElement<'_> {
        // The main column is either the task workspace (a list is open) or the
        // create/load empty-state prompt (none open).
        let main_column = column![self.workspace_or_prompt(), self.status_bar()]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .padding(12);

        // The sidebar is always present — a compact rail when collapsed, the
        // full panel when expanded.
        let body = row![self.list_sidebar(), main_column]
            .spacing(0)
            .height(Length::Fill);

        let shell = container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(shell_container(self.theme_mode));

        // Overlay the rename modal on top when a rename is in progress.
        match self.rename_modal() {
            Some(modal) => stack![shell, modal].into(),
            None => shell.into(),
        }
    }

    /// The task workspace when a list is open, else the empty-state prompt.
    fn workspace_or_prompt(&self) -> AppElement<'_> {
        if self.current_list.is_some() {
            self.tasks_view()
        } else {
            self.empty_state_prompt()
        }
    }

    fn status_bar(&self) -> AppElement<'_> {
        let palette = tokens(self.theme_mode);

        macro_rules! metric_text {
            ($value:expr, $label:expr) => {
                format!("{} {}", $value, $label)
            };
        }

        container(
            row![
                t_body(&self.status_message, 14.0, palette.text_secondary),
                iced::widget::Space::new().width(Length::Fill),
                t_caption(
                    metric_text!(&self.total_count(), "Total"),
                    12.0,
                    palette.text_muted
                )
                .align_x(Alignment::End),
                t_caption(
                    metric_text!(&self.completed_count(), "Completed"),
                    12.0,
                    palette.text_muted
                )
                .align_x(Alignment::End),
                t_caption(
                    metric_text!(&self.open_count(), "Remaining"),
                    12.0,
                    palette.text_muted
                )
                .align_x(Alignment::End),
                t_caption(self.theme_mode.label(), 12.0, palette.text_muted)
                    .align_x(Alignment::End),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(30.0))
        .align_y(Alignment::Center)
        .padding([0, 12])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }
}
