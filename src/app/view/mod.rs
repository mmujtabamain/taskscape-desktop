pub mod header;
pub mod tasks;
pub mod workspace;

use crate::app::{AppElement, Message, Taskscape};
use crate::thememanager::{panel_alt_container, shell_container, tokens};
use crate::widgets::{t_body, t_caption};
use iced::widget::{column, container, mouse_area, row};
use iced::{Alignment, Length};

impl Taskscape {
    pub(crate) fn view_root(&self) -> AppElement<'_> {
        let content = container(
            column![self.tasks_view(), self.status_bar()]
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(14)
                .padding(14),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(shell_container(self.theme_mode));

        // If any editing is active, wrap with mouse_area to detect clicks outside
        if self.is_any_editing() {
            mouse_area(content)
                .on_press(Message::CancelAllEditing)
                .into()
        } else {
            content.into()
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
            .spacing(12),
        )
        .padding([10, 14])
        .style(panel_alt_container(self.theme_mode))
        .into()
    }
}
