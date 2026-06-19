use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Whether an attachment is an image (renderable as a thumbnail) or a generic
/// file (shown as an icon chip).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentKind {
    Image,
    File,
}

/// A file attached to a task.
///
/// `path` is absolute: the user's original file when the attachment is a *link*
/// (`owned == false`), or a copy under [`crate::attachments::files_dir`] when it
/// is *owned* (`owned == true`). Images and screenshots are always copied (so
/// they survive the original moving), while other files link by default. Both
/// processes share the same `files/` dir, so an owned path is valid in either.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Display file name.
    pub name: String,
    /// Absolute path to the file (original when linked, `files/…` when owned).
    pub path: PathBuf,
    pub kind: AttachmentKind,
    /// Whether the file lives in our `files/` dir (a copy we made) rather than
    /// being a link to a user file elsewhere.
    #[serde(default)]
    pub owned: bool,
}

impl Attachment {
    pub fn is_image(&self) -> bool {
        matches!(self.kind, AttachmentKind::Image)
    }
}
