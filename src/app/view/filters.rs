use crate::app::{AppElement, Message, Taskscape};
use crate::models::{
    CompletionFilter, PriorityFilter, QuickDate, SortMode, StatusFilter,
};
use crate::thememanager::{ButtonKind, panel_container};
use crate::widgets::{
    app_input, filter_block, labeled_button, section_heading, segmented_group,
};
use iced::Alignment;
use iced::Length;
use iced::widget::{Space, column, container, row};

impl Taskscape {
    pub(crate) fn filters_panel(&self) -> AppElement<'_> {
        let save_row = row![
            Space::with_width(Length::Fill),
            labeled_button(
                self.theme_mode,
                "⌁",
                "Save",
                ButtonKind::Ghost,
                Some(Message::SaveFilters),
            ),
            labeled_button(
                self.theme_mode,
                "×",
                "Clear",
                ButtonKind::Ghost,
                Some(Message::ClearFilters),
            ),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let filters = column![
            save_row,
            row![
                filter_block(
                    self.theme_mode,
                    "COMPLETION",
                    segmented_group(
                        self.theme_mode,
                        &CompletionFilter::ALL,
                        self.completion_filter,
                        CompletionFilter::label,
                        Message::CompletionFilterChanged,
                    ),
                ),
                filter_block(
                    self.theme_mode,
                    "PRIORITY",
                    segmented_group(
                        self.theme_mode,
                        &PriorityFilter::ALL,
                        self.priority_filter,
                        PriorityFilter::label,
                        Message::PriorityFilterChanged,
                    ),
                ),
                filter_block(
                    self.theme_mode,
                    "STATUS",
                    segmented_group(
                        self.theme_mode,
                        &StatusFilter::ALL,
                        self.status_filter,
                        StatusFilter::label,
                        Message::StatusFilterChanged,
                    ),
                ),
            ]
            .spacing(20),
            row![
                filter_block(
                    self.theme_mode,
                    "QUICK DATE",
                    segmented_group(
                        self.theme_mode,
                        &QuickDate::ALL,
                        self.quick_date,
                        QuickDate::label,
                        Message::QuickDateChanged,
                    ),
                ),
                filter_block(
                    self.theme_mode,
                    "SORT BY",
                    segmented_group(
                        self.theme_mode,
                        &SortMode::ALL,
                        self.sort_mode,
                        SortMode::label,
                        Message::SortModeChanged,
                    ),
                ),
            ]
            .spacing(20),
            row![
                self.filter_field(
                    "SEARCH TEXT",
                    "Search todos and notes",
                    &self.filter_search,
                    Message::FilterSearchChanged,
                    Length::FillPortion(3),
                ),
                self.filter_field(
                    "SEARCH TAG",
                    "tag name",
                    &self.filter_tag,
                    Message::FilterTagChanged,
                    Length::FillPortion(3),
                ),
                self.filter_field(
                    "FROM",
                    "dd/mm/yyyy",
                    &self.filter_from,
                    Message::FilterFromChanged,
                    Length::FillPortion(2),
                ),
                self.filter_field(
                    "TO",
                    "dd/mm/yyyy",
                    &self.filter_to,
                    Message::FilterToChanged,
                    Length::FillPortion(2),
                ),
            ]
            .spacing(12),
        ]
        .spacing(18)
        .padding(14);

        container(filters)
            .style(panel_container(self.theme_mode))
            .into()
    }

    fn filter_field<'a>(
        &self,
        label: &'static str,
        placeholder: &'static str,
        value: &'a str,
        on_input: fn(String) -> Message,
        width: Length,
    ) -> AppElement<'a> {
        column![
            section_heading(self.theme_mode, label),
            app_input(self.theme_mode, placeholder, value, on_input, width, None),
        ]
        .spacing(8)
        .into()
    }
}
