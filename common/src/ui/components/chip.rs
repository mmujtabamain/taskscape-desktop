//! Chips — a static badge and an interactive attachment chip.

use crate::models::Attachment;
use crate::ui::components::icon::{Icon, icon};
use crate::ui::components::icon_button::t_icon_button_ghost;
use crate::ui::components::interactive::{Interactive, Style, Surface};
use crate::ui::components::typography::t_caption;
use crate::ui::theme::{ThemeMode, mix, palette};
use crate::ui::tokens::{radius, text};
use iced::widget::image::Handle as ImageHandle;
use iced::widget::{container, image, row};
use iced::{Alignment, ContentFit, Element, Length};

/// Side of an image attachment's square thumbnail, in logical points.
const THUMB: f32 = 18.0;

/// A small, static badge (no interaction).
pub fn t_small_chip<'a, M: 'a>(theme_mode: ThemeMode, label: &'a str, accent: bool) -> Element<'a, M> {
    let p = palette(theme_mode);
    let (fill, ink) = if accent {
        (p.accent, p.on_accent)
    } else {
        (p.raised, p.text_dim)
    };

    container(t_caption(label, text::LABEL, ink))
        .padding([5, 9])
        .style(move |_t| {
            container::Style::default()
                .background(fill)
                .border(crate::ui::theme::border(radius::SM, 0.0, fill))
        })
        .into()
}

/// A compact attachment chip: thumbnail (images) or paperclip icon (files), the
/// name, and a remove (×) button. The body fires `on_open`; the × fires `on_remove`.
pub fn t_attachment_chip<'a, M: Clone + 'static>(
    theme_mode: ThemeMode,
    attachment: &'a Attachment,
    on_open: M,
    on_remove: M,
) -> Element<'a, M> {
    let p = palette(theme_mode);

    let leading: Element<'a, M> = if attachment.is_image() {
        container(
            image(ImageHandle::from_path(&attachment.path))
                .width(Length::Fixed(THUMB))
                .height(Length::Fixed(THUMB))
                .content_fit(ContentFit::Cover),
        )
        .width(Length::Fixed(THUMB))
        .height(Length::Fixed(THUMB))
        .clip(true)
        .into()
    } else {
        icon(Icon::Attach, text::LABEL, p.text_dim).into()
    };

    let body = row![
        leading,
        t_caption(&attachment.name, text::LABEL, p.text_dim),
        t_icon_button_ghost(theme_mode, Icon::Close, Some(on_remove)),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let style = Style {
        rest: Surface::new(p.raised, 0.0),
        hover: Surface::new(mix(p.raised, p.text, 0.05), 0.0),
        pressed: Surface::new(mix(p.raised, p.text, 0.08), 0.0),
        radius: radius::SM,
        ring: None,
    };

    Interactive::new(body, style)
        .padding([2, 4])
        .on_press(on_open)
        .into()
}
