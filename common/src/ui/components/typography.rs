//! Text helpers. Raleway carries headings/display; Montserrat carries body/labels.

use crate::utils::fonts::{montserrat_regular, raleway_bold, raleway_semibold};
use iced::Color;
use iced::widget::text;

/// Section / screen heading (Raleway SemiBold).
pub fn t_heading<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(raleway_semibold()).size(size).color(color)
}

/// The largest, airiest display text (Raleway Bold) — list titles.
pub fn t_display<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(raleway_bold()).size(size).color(color)
}

/// Body and control text (Montserrat).
pub fn t_body<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(montserrat_regular()).size(size).color(color)
}

/// Quiet metadata (Montserrat, smaller).
pub fn t_caption<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(montserrat_regular()).size(size).color(color)
}
