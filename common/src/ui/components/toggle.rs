//! A toggle built on [`Interactive`]: a rounded-rect track (not a pill) whose fill
//! animates on hover/press; the knob sits left (off) or right (on).

use crate::ui::components::interactive::{Interactive, Style, Surface};
use crate::ui::theme::{ThemeMode, border, palette, with_alpha};
use crate::ui::tokens::radius;
use iced::widget::{Space, container, row};
use iced::{Alignment, Element, Length};

const TRACK_W: f32 = 38.0;
const TRACK_H: f32 = 22.0;
const KNOB: f32 = 16.0;

pub fn t_toggle<'a, M: Clone + 'a>(theme_mode: ThemeMode, on: bool, on_toggle: M) -> Element<'a, M> {
    let p = palette(theme_mode);

    let knob_fill = if on { p.on_accent } else { p.text_dim };
    let knob = container(Space::new().width(Length::Fixed(KNOB)).height(Length::Fixed(KNOB)))
        .style(move |_t| {
            container::Style::default()
                .background(knob_fill)
                .border(border(5.0, 0.0, knob_fill))
        });

    let inner = if on {
        row![Space::new().width(Length::Fill), knob]
    } else {
        row![knob, Space::new().width(Length::Fill)]
    }
    .align_y(Alignment::Center);

    let track = container(inner)
        .width(Length::Fixed(TRACK_W))
        .height(Length::Fixed(TRACK_H))
        .padding([3, 3]);

    let style = if on {
        Style {
            rest: Surface::new(p.accent, 0.0),
            hover: Surface::new(p.accent_hover, 0.0),
            pressed: Surface::new(p.accent, 0.0),
            radius: radius::SM,
            ring: None,
        }
    } else {
        Style {
            rest: Surface::new(with_alpha(p.text, 0.12), 0.0),
            hover: Surface::new(with_alpha(p.text, 0.18), 0.0),
            pressed: Surface::new(with_alpha(p.text, 0.18), 0.0),
            radius: radius::SM,
            ring: None,
        }
    };

    Interactive::new(track, style).on_press(on_toggle).into()
}
