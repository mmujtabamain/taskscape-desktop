#[derive(Debug, Clone, Copy)]
pub enum ButtonKind {
    Primary,
    Secondary,
    Ghost,
    Icon,
    Sidebar(bool),
    Chip(bool),
}
