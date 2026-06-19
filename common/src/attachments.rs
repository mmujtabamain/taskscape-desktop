//! Filesystem helpers for task attachments.
//!
//! Where copied files live (`~/.taskscape/files/`), how to build an
//! [`Attachment`] from a picked file (linking the original or copying it in),
//! how to capture a screenshot, and how to open an attachment in the OS default
//! app. The [`Attachment`] model itself is in [`crate::models`].

use crate::models::{Attachment, AttachmentKind};
use crate::storage::app_data_dir;
use std::path::{Path, PathBuf};

const IMAGE_EXTS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff", "tif", "heic", "heif", "svg", "ico",
];

/// Directory holding copies of attached/owned files, created if missing.
pub fn files_dir() -> PathBuf {
    let dir = app_data_dir().join("files");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Whether a path looks like an image, judged by its file extension.
pub fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| IMAGE_EXTS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

fn file_name_of(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| String::from("file"))
}

/// Picks a not-yet-used `dir/<stem>.<ext>` path, suffixing `-2`, `-3`, … on
/// collision. An empty `ext` yields an extension-less name.
fn unique_path(dir: &Path, stem: &str, ext: &str) -> PathBuf {
    let named = |name: String| dir.join(name);
    let compose = |suffix: String| {
        if ext.is_empty() {
            named(format!("{stem}{suffix}"))
        } else {
            named(format!("{stem}{suffix}.{ext}"))
        }
    };
    let mut candidate = compose(String::new());
    let mut n = 2;
    while candidate.exists() {
        candidate = compose(format!("-{n}"));
        n += 1;
    }
    candidate
}

/// Copies `src` into the `files/` dir under a unique name; returns the new path.
pub fn copy_into_files(src: &Path) -> Result<PathBuf, String> {
    let dir = files_dir();
    let stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("");
    let dest = unique_path(&dir, stem, ext);
    std::fs::copy(src, &dest).map_err(|e| format!("Could not copy attachment: {e}"))?;
    Ok(dest)
}

/// Builds an attachment from a source file. Images are always copied into
/// `files/` (so they survive the original moving); other files are linked to
/// their original location unless `copy` is set.
pub fn attachment_from_path(src: &Path, copy: bool) -> Result<Attachment, String> {
    let image = is_image(src);
    let owned = image || copy;
    let path = if owned {
        copy_into_files(src)?
    } else {
        src.to_path_buf()
    };
    Ok(Attachment {
        name: file_name_of(src),
        path,
        kind: if image {
            AttachmentKind::Image
        } else {
            AttachmentKind::File
        },
        owned,
    })
}

/// Captures a full-screen screenshot into `files/` and returns it as an owned
/// image attachment. macOS only; an error elsewhere.
pub fn capture_screenshot() -> Result<Attachment, String> {
    #[cfg(target_os = "macos")]
    {
        let dest = unique_path(&files_dir(), "screenshot", "png");
        let status = std::process::Command::new("screencapture")
            .arg("-x")
            .arg(&dest)
            .status()
            .map_err(|e| format!("Could not run screencapture: {e}"))?;
        if !status.success() || !dest.exists() {
            return Err(String::from("Screenshot capture failed."));
        }
        Ok(Attachment {
            name: file_name_of(&dest),
            path: dest,
            kind: AttachmentKind::Image,
            owned: true,
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(String::from("Screenshots are only supported on macOS."))
    }
}

/// Opens a path in the OS default application (best-effort, non-blocking spawn).
pub fn open_path(path: &Path) {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(path).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(path)
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        let _ = path;
    }
}
