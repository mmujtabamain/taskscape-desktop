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
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

/// Stable id of the "Quit" context-menu item.
#[cfg(target_os = "macos")]
const QUIT_ITEM_ID: &str = "taskscape.tray.quit";

/// Commands produced by interacting with the menu bar icon.
#[derive(Debug, Clone, Copy)]
pub enum TrayCommand {
    /// Toggle the mini window. Carries the icon's screen rect (in physical
    /// pixels) so the mini window can be anchored beneath it.
    ShowWindow {
        /// Top-left x of the menu bar icon, in physical pixels.
        icon_x: f64,
        /// Top-left y of the menu bar icon, in physical pixels.
        icon_y: f64,
        /// Width of the menu bar icon, in physical pixels.
        icon_width: f64,
        /// Height of the menu bar icon, in physical pixels.
        icon_height: f64,
    },
    /// The user chose "Quit" from the tray's right-click menu.
    Quit,
}

#[cfg(target_os = "macos")]
thread_local! {
    // The `TrayIcon` must stay alive for the icon to remain in the menu bar.
    static TRAY_HANDLE: RefCell<Option<TrayIcon>> = const { RefCell::new(None) };
    // The `Menu` must also stay alive for the context menu to keep working.
    static TRAY_MENU: RefCell<Option<Menu>> = const { RefCell::new(None) };
    static TRAY_INSTALLED: Cell<bool> = const { Cell::new(false) };
}

/// Installs the menu bar icon and its right-click context menu (with Quit).
/// Must be called on the main (UI) thread.
#[cfg(target_os = "macos")]
pub fn install() -> Result<(), String> {
    if TRAY_INSTALLED.with(Cell::get) {
        return Ok(());
    }

    let icon = build_icon()?;

    // Right-click context menu: a header label, a separator, then Quit. Left
    // click still toggles the mini window (`with_menu_on_left_click(false)`).
    let menu = Menu::new();
    let header = MenuItem::new("Taskscape", false, None);
    let quit = MenuItem::with_id(QUIT_ITEM_ID, "Quit Taskscape", true, None);
    menu.append(&header)
        .and_then(|_| menu.append(&PredefinedMenuItem::separator()))
        .and_then(|_| menu.append(&quit))
        .map_err(|e| format!("failed to build tray menu: {e}"))?;

    let tray = TrayIconBuilder::new()
        .with_tooltip("Taskscape")
        // Template mode lets macOS tint the icon for light/dark menu bars.
        .with_icon_as_template(true)
        .with_icon(icon)
        .with_menu(Box::new(menu.clone()))
        // Keep left-click for toggling the window; the menu is right-click only.
        .with_menu_on_left_click(false)
        .build()
        .map_err(|e| format!("failed to create tray icon: {e}"))?;

    TRAY_HANDLE.with(|slot| *slot.borrow_mut() = Some(tray));
    TRAY_MENU.with(|slot| *slot.borrow_mut() = Some(menu));
    TRAY_INSTALLED.with(|flag| flag.set(true));
    Ok(())
}

/// The main display's backing scale factor (2.0 on Retina, 1.0 otherwise).
///
/// The menu bar icon rect from `tray-icon` is in physical pixels, but iced
/// positions windows in logical points; dividing by this converts between them.
#[cfg(target_os = "macos")]
pub fn main_screen_scale() -> f64 {
    use objc2_app_kit::NSScreen;
    use objc2_foundation::MainThreadMarker;

    // Must run on the main thread; the tray handler does.
    MainThreadMarker::new()
        .and_then(NSScreen::mainScreen)
        .map(|screen| screen.backingScaleFactor() as f64)
        .filter(|s| *s > 0.0)
        .unwrap_or(1.0)
}

#[cfg(not(target_os = "macos"))]
pub fn main_screen_scale() -> f64 {
    1.0
}

/// Disables the drop shadow of the window backing `handle`. macOS draws the
/// shadow on the (square) window frame, which appears as a square outline behind
/// the mini window's transparent rounded corners; turning it off removes that.
/// Must run on the UI thread (call inside `window::run`).
#[cfg(target_os = "macos")]
pub fn disable_window_shadow(handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::NSView;

    let Ok(window_handle) = handle.window_handle() else {
        return;
    };
    if let RawWindowHandle::AppKit(appkit) = window_handle.as_raw() {
        // SAFETY: we're on the UI thread (window::run guarantees it) and the view
        // pointer is valid for the lifetime of this call.
        unsafe {
            let view: &NSView = appkit.ns_view.cast().as_ref();
            if let Some(window) = view.window() {
                window.setHasShadow(false);
                // Nudge AppKit to recompute the shadow region immediately.
                window.invalidateShadow();
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn disable_window_shadow(
    _handle: &dyn iced::window::raw_window_handle::HasWindowHandle,
) {
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

/// Forwards icon clicks and context-menu selections from their global receivers
/// into one async stream, without blocking the executor.
#[cfg(target_os = "macos")]
fn tray_event_stream() -> impl iced::futures::Stream<Item = TrayCommand> {
    let (tx, rx) = mpsc::channel::<TrayCommand>(64);

    // Icon click events.
    {
        let mut tx = tx.clone();
        if let Err(error) = std::thread::Builder::new()
            .name("taskscape-tray-events".into())
            .spawn(move || {
                let receiver = TrayIconEvent::receiver();
                while let Ok(event) = receiver.recv() {
                    if let Some(cmd) = map_event_to_command(&event) {
                        if iced::futures::executor::block_on(tx.send(cmd)).is_err() {
                            break; // subscription dropped
                        }
                    }
                }
            })
        {
            eprintln!("Failed to spawn tray event thread: {error}");
        }
    }

    // Context-menu selections (Quit).
    {
        let mut tx = tx;
        if let Err(error) = std::thread::Builder::new()
            .name("taskscape-tray-menu".into())
            .spawn(move || {
                let receiver = MenuEvent::receiver();
                while let Ok(event) = receiver.recv() {
                    if event.id.0 == QUIT_ITEM_ID {
                        if iced::futures::executor::block_on(tx.send(TrayCommand::Quit)).is_err() {
                            break; // subscription dropped
                        }
                    }
                }
            })
        {
            eprintln!("Failed to spawn tray menu thread: {error}");
        }
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
            rect,
            ..
        } => Some(TrayCommand::ShowWindow {
            icon_x: rect.position.x,
            icon_y: rect.position.y,
            icon_width: rect.size.width as f64,
            icon_height: rect.size.height as f64,
        }),
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
