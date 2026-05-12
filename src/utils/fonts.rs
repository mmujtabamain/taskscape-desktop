use iced::font::{Family, Stretch, Style, Weight};
use iced::Font;

pub const INTER_REGULAR_BYTES: &[u8] =
    include_bytes!("../../assets/fonts/Inter/static/Inter_24pt-Regular.ttf");
pub const POPPINS_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../../assets/fonts/Poppins/Poppins-SemiBold.ttf");

pub const fn poppins_semibold() -> Font {
    Font {
        family: Family::Name("Poppins"),
        weight: Weight::Semibold,
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

pub const fn inter_regular() -> Font {
    Font {
        family: Family::Name("Inter"),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}
