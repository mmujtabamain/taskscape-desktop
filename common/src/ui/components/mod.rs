//! The animated `t_*` component toolkit. Built on `interactive` (the custom
//! animated `Widget`) and styled through `theme` + `tokens`.

pub mod button;
pub mod checkbox;
pub mod chip;
pub mod containers;
pub mod dropdown;
pub mod editable_title;
pub mod icon;
pub mod icon_button;
pub mod input;
pub mod interactive;
pub mod metric;
pub mod toggle;
pub mod typography;

pub use button::{ButtonKind, surface_style, t_button};
pub use checkbox::t_checkbox;
pub use containers::{
    bar, divider, frosted_shell, glass_shell, modal_backdrop, modal_card, raised, shell, sidebar,
    surface,
};
pub use chip::{t_attachment_chip, t_small_chip};
pub use dropdown::{pick_list_style, t_dropdown};
pub use editable_title::{TITLE_INPUT_ID, t_editable_title};
pub use icon::{Icon, icon};
pub use icon_button::{t_icon_button, t_icon_button_ghost};
pub use input::{t_input_box, text_input_style};
pub use interactive::{Interactive, Style as SurfaceStyle, Surface};
pub use metric::t_metric;
pub use toggle::t_toggle;
pub use typography::{t_body, t_caption, t_display, t_heading};
