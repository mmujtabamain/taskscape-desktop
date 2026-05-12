use iced::Color;
use lucide_icons::Icon;

/// Renders a Lucide icon at the given size and colour.
/// Uses the library's own `From<Icon>` conversion which assigns the
/// correct `Font::with_name("lucide")` and uses the bundled TTF codepoints.
pub fn lucide_icon(icon: Icon, size: f32, color: Color) -> iced::widget::Text<'static> {
    let t: iced::widget::Text<'static> = icon.into();
    t.size(size).color(color)
}
