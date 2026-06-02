use crate::thememanager::{ThemeMode, panel_alt_container, tokens};
use crate::widgets::t_caption;
use iced::widget::{container, row};
use iced::{Alignment, Element, Length};

pub fn t_metric_card<M: 'static>(
    theme_mode: ThemeMode,
    value: String,
    label: &'static str,
) -> Element<'static, M> {
    let palette = tokens(theme_mode);

    container(
        row![t_caption(value + " " + label, 12.0, palette.text_muted),]
            .align_y(Alignment::Center)
            .spacing(12),
    )
    .width(Length::Fill)
    .padding(12)
    .style(panel_alt_container(theme_mode))
    .into()
}
