//! macOS menu bar (status bar) icon.
//!
//! Adds an `NSStatusItem` to the system menu bar, similar to OneDrive's icon.
//! Left-clicking the icon reveals the main window.
//!
//! Windows and Linux are intentionally left unimplemented for now: their tray
//! integrations need a platform event loop / GTK handle that we are not wiring
//! up yet. The public API stays the same on every platform so the rest of the
//! app does not need `cfg` guards — the non-macOS builds simply no-op.

use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;

#[cfg(target_os = "macos")]
use std::cell::{Cell, RefCell};

#[cfg(target_os = "macos")]
use tray_icon::{
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent,
};

/// Commands produced by interacting with the menu bar icon.
#[derive(Debug, Clone, Copy)]
pub enum TrayCommand {
    /// Bring the main window to the foreground.
    ShowWindow,
}

#[cfg(target_os = "macos")]
thread_local! {
    // The `TrayIcon` must stay alive for the icon to remain in the menu bar.
    static TRAY_HANDLE: RefCell<Option<TrayIcon>> = const { RefCell::new(None) };
    static TRAY_INSTALLED: Cell<bool> = const { Cell::new(false) };
}

/// Installs the menu bar icon. Must be called on the main (UI) thread.
#[cfg(target_os = "macos")]
pub fn install() -> Result<(), String> {
    if TRAY_INSTALLED.with(Cell::get) {
        return Ok(());
    }

    let icon = build_icon()?;

    let tray = TrayIconBuilder::new()
        .with_tooltip("Taskscape")
        // Template mode lets macOS tint the icon for light/dark menu bars.
        .with_icon_as_template(true)
        .with_icon(icon)
        .build()
        .map_err(|e| format!("failed to create tray icon: {e}"))?;

    TRAY_HANDLE.with(|slot| *slot.borrow_mut() = Some(tray));
    TRAY_INSTALLED.with(|flag| flag.set(true));
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn install() -> Result<(), String> {
    // TODO: Windows (Shell_NotifyIcon) and Linux (StatusNotifierItem/GTK).
    Err(String::from(
        "Menu bar icon is only implemented on macOS for now.",
    ))
}

/// Subscription that delivers menu bar icon interactions as [`TrayCommand`]s.
#[cfg(target_os = "macos")]
pub fn subscription() -> Subscription<TrayCommand> {
    Subscription::run(tray_event_stream)
}

#[cfg(not(target_os = "macos"))]
pub fn subscription() -> Subscription<TrayCommand> {
    Subscription::none()
}

/// Forwards tray events from the global receiver into an async stream without
/// blocking the executor, mirroring the native menu plumbing.
#[cfg(target_os = "macos")]
fn tray_event_stream() -> impl iced::futures::Stream<Item = TrayCommand> {
    let (tx, rx) = mpsc::channel::<TrayCommand>(64);

    if let Err(error) = std::thread::Builder::new()
        .name("taskscape-tray-events".into())
        .spawn(move || {
            let mut tx = tx;
            let receiver = TrayIconEvent::receiver();

            while let Ok(event) = receiver.recv() {
                if let Some(cmd) = map_event_to_command(&event) {
                    if iced::futures::executor::block_on(tx.send(cmd)).is_err() {
                        // The subscription stream was dropped.
                        break;
                    }
                }
            }
        })
    {
        eprintln!("Failed to spawn tray event thread: {error}");
    }

    rx
}

#[cfg(target_os = "macos")]
fn map_event_to_command(event: &TrayIconEvent) -> Option<TrayCommand> {
    match event {
        // Fire once, on release, for a left click on the icon.
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => Some(TrayCommand::ShowWindow),
        _ => None,
    }
}

/// Procedurally builds a simple "task list" glyph (three rows, each a small
/// box and a line) as a template icon, so we don't ship a binary asset.
#[cfg(target_os = "macos")]
fn build_icon() -> Result<Icon, String> {
    const SIZE: usize = 32;
    let mut rgba = vec![0u8; SIZE * SIZE * 4];

    let mut fill = |x: usize, y: usize| {
        if x < SIZE && y < SIZE {
            let i = (y * SIZE + x) * 4;
            // Color is ignored in template mode; only alpha defines the shape.
            rgba[i + 3] = 255;
        }
    };

    // Three rows: checkbox (filled square) + a line beside it.
    for &top in &[6usize, 15, 24] {
        // 6x6 box on the left.
        for y in top..top + 6 {
            for x in 4..10 {
                fill(x, y);
            }
        }
        // 2px line, vertically centered against the box.
        for y in top + 2..top + 4 {
            for x in 12..28 {
                fill(x, y);
            }
        }
    }

    Icon::from_rgba(rgba, SIZE as u32, SIZE as u32)
        .map_err(|e| format!("failed to build tray icon: {e}"))
}
