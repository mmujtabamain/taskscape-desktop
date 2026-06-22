mod header;
mod lists;
mod settings;
mod tasks;
mod titlebar;
mod workspace;

pub(crate) use lists::RENAME_INPUT_ID;

use crate::app::{AppElement, Taskscape};
use common::ui::tokens::{space, text};
use common::ui::{bar, frosted_shell, palette, t_body, t_caption, t_metric};
use iced::widget::{Space, column, container, row, stack};
use iced::{Alignment, Length};

impl Taskscape {
    pub(crate) fn view_root(&self) -> AppElement<'_> {
        let main_column = column![self.workspace_or_prompt(), self.status_bar()]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(space::LG)
            .padding(space::LG);

        let body = row![self.list_sidebar(), main_column]
            .spacing(0)
            .height(Length::Fill);

        // The custom title bar spans the full width above the sidebar + content;
        // the whole window is the one frosted-glass surface (`frosted_shell` over
        // the native vibrancy backdrop installed in `chrome::apply`).
        let root = column![self.title_bar(), body]
            .width(Length::Fill)
            .height(Length::Fill);

        let shell = container(root)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(frosted_shell(self.theme_mode));

        match self.rename_modal().or_else(|| self.clear_all_modal()) {
            Some(modal) => stack![shell, modal].into(),
            None => shell.into(),
        }
    }

    /// Settings when open, else the task workspace, else the empty-state prompt.
    fn workspace_or_prompt(&self) -> AppElement<'_> {
        if self.show_settings {
            self.settings_view()
        } else if self.current_list.is_some() {
            self.tasks_view()
        } else {
            self.empty_state_prompt()
        }
    }

    fn status_bar(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        container(
            row![
                t_body(&self.status_message, text::SMALL, p.text_dim),
                Space::new().width(Length::Fill),
                t_metric(self.theme_mode, self.total_count().to_string(), "Total"),
                t_metric(self.theme_mode, self.completed_count().to_string(), "Done"),
                t_metric(self.theme_mode, self.open_count().to_string(), "Left"),
                t_caption(self.theme_mode.label(), text::CAPTION, p.text_muted),
            ]
            .spacing(space::LG)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(32.0))
        .align_y(Alignment::Center)
        .padding([0, 12])
        .style(bar(self.theme_mode))
        .into()
    }
}
