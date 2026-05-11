use crate::thememanager::{ThemeMode, tokens};
use iced::gradient;
use iced::{Border, Color, Shadow, Vector};

pub fn background_gradient(mode: ThemeMode) -> gradient::Linear {
    let palette = tokens(mode);

    gradient::Linear::new(0.25)
        .add_stop(0.0, palette.background_top)
        .add_stop(0.42, mix(palette.background_top, palette.background_bottom, 0.35))
        .add_stop(1.0, palette.background_bottom)
}

pub fn border(radius: f32, width: f32, color: Color) -> Border {
    Border {
        radius: radius.into(),
        width,
        color,
    }
}

pub fn shadow(color: Color, offset_y: f32, blur_radius: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::new(0.0, offset_y),
        blur_radius,
    }
}

pub fn color(red: u8, green: u8, blue: u8) -> Color {
    Color::from_rgb8(red, green, blue)
}

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

pub fn mix(from: Color, to: Color, amount: f32) -> Color {
    let clamped = amount.clamp(0.0, 1.0);

    Color {
        r: from.r + (to.r - from.r) * clamped,
        g: from.g + (to.g - from.g) * clamped,
        b: from.b + (to.b - from.b) * clamped,
        a: from.a + (to.a - from.a) * clamped,
    }
}
