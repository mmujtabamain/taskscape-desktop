use crate::utils::fonts::{inter_regular, poppins_semibold};
use iced::Color;
use iced::widget::text;

pub fn t_heading<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(poppins_semibold()).size(size).color(color)
}

pub fn t_body<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(inter_regular()).size(size).color(color)
}

pub fn t_caption<'a>(content: impl text::IntoFragment<'a>, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(content).font(inter_regular()).size(size).color(color)
}
