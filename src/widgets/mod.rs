pub mod lucide_icon;
pub mod t_button;
pub mod t_dropdown;
pub mod t_editable_title;
pub mod t_icon_button;
pub mod t_input_box;
pub mod t_metric_card;
pub mod t_small_chip;
pub mod t_typography;

use iced::widget::container;
pub use lucide_icon::lucide_icon;
pub use t_button::t_button;
pub use t_dropdown::t_dropdown;
pub use t_editable_title::t_editable_title;
pub use t_icon_button::t_icon_button;
pub use t_input_box::t_input_box;
pub use t_metric_card::t_metric_card;
pub use t_small_chip::t_small_chip;
pub use t_typography::{t_body, t_caption, t_heading};

/// Wraps an element with a debug outline showing its bounds without changing its position or size.
/// The outline is a thin red border with no background fill, making it ideal for layout debugging.
pub fn t_debug_outline<'a, Message: 'a>(
    element: impl Into<iced::Element<'a, Message>>,
) -> iced::Element<'a, Message> {
    container(element.into())
        .style(|_| container::Style {
            background: None,
            border: iced::Border {
                color: iced::Color::from_rgb(1.0, 0.0, 0.0),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

pub trait DebugWidget<'a, Message>: Sized {
    fn debug(self) -> iced::Element<'a, Message>;
    fn debug_colored(self, color: iced::Color) -> iced::Element<'a, Message>;
}

impl<'a, Message: 'a, T: Into<iced::Element<'a, Message>>> DebugWidget<'a, Message> for T {
    fn debug(self) -> iced::Element<'a, Message> {
        container(self.into())
            .style(|_| container::Style {
                background: Some(iced::Color::from_rgba(1.0, 0.0, 0.0, 0.2).into()),
                border: iced::Border {
                    color: iced::Color::from_rgb(1.0, 0.0, 0.0),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn debug_colored(self, color: iced::Color) -> iced::Element<'a, Message> {
        container(self.into())
            .style(move |_| container::Style {
                background: Some(iced::Color::from_rgba(color.r, color.g, color.b, 0.2).into()),
                border: iced::Border {
                    color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
