//! The main window's custom title bar.
//!
//! The native system title bar is made transparent (see `app::chrome`), leaving
//! only the traffic-light buttons; this slim, draggable bar takes its place. It
//! sits over the frosted-glass backdrop, carries a quiet centered wordmark, and
//! reserves a gutter on the left so it never overlaps the traffic lights.

use crate::app::{AppElement, Message, Taskscape};
use common::ui::tokens::text;
use common::ui::{palette, t_heading};
use iced::widget::{Space, container, mouse_area, row};
use iced::{Alignment, Length};

/// Title-bar height. Taller than the 28pt system bar so the wordmark and the
/// traffic lights breathe, and the drag target is comfortable.
pub(crate) const TITLE_BAR_H: f32 = 40.0;

/// Width reserved on the left for the native traffic-light buttons. Mirrored on
/// the right so the wordmark stays centered in the window.
const TRAFFIC_LIGHT_GUTTER: f32 = 78.0;

impl Taskscape {
    pub(crate) fn title_bar(&self) -> AppElement<'_> {
        let p = palette(self.theme_mode);

        let wordmark = container(t_heading("Taskscape", text::SMALL, p.text_dim))
            .width(Length::Fill)
            .center_x(Length::Fill);

        let bar = row![
            Space::new().width(Length::Fixed(TRAFFIC_LIGHT_GUTTER)),
            wordmark,
            Space::new().width(Length::Fixed(TRAFFIC_LIGHT_GUTTER)),
        ]
        .align_y(Alignment::Center);

        // The whole bar is the drag handle: the system bar is transparent, so it
        // no longer moves the window for us.
        mouse_area(
            container(bar)
                .width(Length::Fill)
                .height(Length::Fixed(TITLE_BAR_H))
                .align_y(Alignment::Center),
        )
        .on_press(Message::DragWindow)
        .into()
    }
}
