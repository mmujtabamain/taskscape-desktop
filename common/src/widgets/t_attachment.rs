use crate::models::Attachment;
use crate::thememanager::{ButtonKind, ThemeMode, button_style, helpers, tokens};
use crate::widgets::{lucide_icon, t_caption};
use iced::widget::image::Handle as ImageHandle;
use iced::widget::{button, container, image, row};
use iced::{Alignment, ContentFit, Element, Length};
use lucide_icons::Icon;

/// Side of an image attachment's square thumbnail, in logical points.
const THUMB: f32 = 18.0;

/// A compact attachment chip: a thumbnail (images) or paperclip icon (files),
/// the file name, and a remove (×) button. Pressing the chip body fires
/// `on_open`; pressing the × fires `on_remove`.
pub fn t_attachment_chip<'a, M: Clone + 'static>(
    theme_mode: ThemeMode,
    attachment: &'a Attachment,
    on_open: M,
    on_remove: M,
) -> Element<'a, M> {
    let palette = tokens(theme_mode);

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
        lucide_icon(Icon::Paperclip, 12.0, palette.text_secondary).into()
    };

    let open = button(
        row![leading, t_caption(&attachment.name, 12.0, palette.text_secondary)]
            .spacing(6)
            .align_y(Alignment::Center),
    )
    .padding([2, 4])
    .style(button_style(theme_mode, ButtonKind::Plain))
    .on_press(on_open);

    let remove = button(lucide_icon(Icon::X, 11.0, palette.text_muted))
        .padding([2, 4])
        .style(button_style(theme_mode, ButtonKind::Plain))
        .on_press(on_remove);

    container(
        row![open, remove]
            .spacing(0)
            .align_y(Alignment::Center),
    )
    .padding([1, 2])
    .style(move |_theme| {
        container::Style::default()
            .background(palette.panel_raised)
            .border(helpers::border(8.0, 1.0, palette.border))
    })
    .into()
}
