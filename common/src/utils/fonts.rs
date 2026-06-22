//! Embedded fonts + `Font` builders.
//!
//! The redesign uses **Raleway** for display/headings and **Montserrat** for
//! body/UI, with a sharp-cornered **Material Symbols Sharp** icon subset. The
//! fontsource static weights carry no typographic family, so each weight is its
//! own family name — the builders below select by that exact name.
//!
//! Inter + Poppins remain only until the old `widgets`/`thememanager` modules are
//! removed; the new `common::ui` layer does not use them.

use iced::Font;
use iced::font::{Family, Stretch, Style, Weight};

// --- New design system: Raleway (display) + Montserrat (body/UI) ---------------

pub const MONTSERRAT_REGULAR_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Montserrat/Montserrat-Regular.ttf");
pub const MONTSERRAT_MEDIUM_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Montserrat/Montserrat-Medium.ttf");
pub const MONTSERRAT_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Montserrat/Montserrat-SemiBold.ttf");
pub const RALEWAY_MEDIUM_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Medium.ttf");
pub const RALEWAY_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-SemiBold.ttf");
pub const RALEWAY_BOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Raleway/Raleway-Bold.ttf");

/// 28-glyph subset of Material Symbols Sharp (see
/// `assets/fonts/MaterialSymbols/used-icons.txt` + `regen-subset.sh`).
pub const ICON_FONT_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/MaterialSymbols/MaterialSymbolsSharp-subset.ttf");

/// Every font byte blob the apps must register at startup.
pub const REGISTERED_FONT_BYTES: &[&[u8]] = &[
    MONTSERRAT_REGULAR_BYTES,
    MONTSERRAT_MEDIUM_BYTES,
    MONTSERRAT_SEMIBOLD_BYTES,
    RALEWAY_MEDIUM_BYTES,
    RALEWAY_SEMIBOLD_BYTES,
    RALEWAY_BOLD_BYTES,
    ICON_FONT_BYTES,
];

const fn face(name: &'static str, weight: Weight) -> Font {
    Font {
        family: Family::Name(name),
        weight,
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

// Body / UI — Montserrat.
pub const fn montserrat_regular() -> Font {
    face("Montserrat", Weight::Normal)
}
pub const fn montserrat_medium() -> Font {
    face("Montserrat Medium", Weight::Medium)
}
pub const fn montserrat_semibold() -> Font {
    face("Montserrat SemiBold", Weight::Semibold)
}

// Display / headings — Raleway.
pub const fn raleway_medium() -> Font {
    face("Raleway Medium", Weight::Medium)
}
pub const fn raleway_semibold() -> Font {
    face("Raleway SemiBold", Weight::Semibold)
}
pub const fn raleway_bold() -> Font {
    face("Raleway", Weight::Bold)
}

/// The Material Symbols Sharp icon face.
pub const fn icon_font() -> Font {
    face("Material Symbols Sharp", Weight::Normal)
}

// --- Retired (Inter + Poppins): kept until old widgets/thememanager are removed --

pub const INTER_REGULAR_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Inter/static/Inter_24pt-Regular.ttf");
pub const POPPINS_SEMIBOLD_BYTES: &[u8] =
    include_bytes!("../../../assets/fonts/Poppins/Poppins-SemiBold.ttf");

pub const fn poppins_semibold() -> Font {
    face("Poppins", Weight::Semibold)
}

pub const fn inter_regular() -> Font {
    face("Inter", Weight::Normal)
}
