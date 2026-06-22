//! Geometry + type tokens. Centralizes the radii, spacing, and sizes that used to
//! be scattered as magic numbers across the screens.
//!
//! Sharpened, soft-cornered: **rounded corners everywhere, no pills, no sharp
//! edges**. Sharpness lives in the marks (type + icons) and in discipline (fill
//! over outline, restrained cards), not in the geometry.

/// Corner radii. Consistent rounding; nothing sharp, nothing fully circular.
pub mod radius {
    /// Chips, small controls.
    pub const SM: f32 = 8.0;
    /// Buttons, inputs, dropdowns, list rows.
    pub const MD: f32 = 10.0;
    /// Panels, cards.
    pub const LG: f32 = 12.0;
    /// Modal cards, the frosted mini-window shell.
    pub const XL: f32 = 16.0;
}

/// Spacing scale (px). Used for gaps and padding.
pub mod space {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 6.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 12.0;
    pub const XL: f32 = 16.0;
    pub const XXL: f32 = 24.0;
}

/// Type sizes (px) by role. Raleway carries display/heading/title; Montserrat
/// carries body/label/caption.
pub mod text {
    /// The editable list title — the one large, airy Raleway moment.
    pub const DISPLAY: f32 = 32.0;
    /// Screen / section headings.
    pub const HEADING: f32 = 22.0;
    /// Sub-section / modal titles.
    pub const TITLE: f32 = 17.0;
    /// Default body + control labels.
    pub const BODY: f32 = 14.0;
    /// Slightly smaller body.
    pub const SMALL: f32 = 13.0;
    /// Labels, chips.
    pub const LABEL: f32 = 12.0;
    /// Quietest metadata.
    pub const CAPTION: f32 = 11.0;
}

/// Hairline border width (only used where there is no fill).
pub const HAIRLINE_WIDTH: f32 = 1.0;
