#[derive(Debug, Clone, Copy)]
pub enum ButtonKind {
    Primary,
    Ghost,
    Icon,
    /// Transparent, borderless button — used when the button sits inside an
    /// already-styled container (e.g. a list row) and should not paint its own
    /// background. Only a subtle hover tint is applied.
    Plain,
}
