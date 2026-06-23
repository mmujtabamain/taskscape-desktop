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

/// Gives the window backing `handle` genuinely rounded corners with `radius`
/// logical points.
///
/// A borderless transparent `NSWindow` otherwise shows square edges: the window
/// frame is rectangular and its drop shadow is square. The robust fix (per the
/// macOS forums) is to clip the *content view's CALayer* to a rounded rect so the
/// GPU surface itself is rounded, make the window non-opaque with a clear
/// background, and drop the square shadow. Must run on the UI thread (call inside
/// `window::run`).
#[cfg(target_os = "macos")]
pub fn round_window(
    handle: &dyn iced::window::raw_window_handle::HasWindowHandle,
    radius: f64,
) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::{NSColor, NSView};

    let Ok(window_handle) = handle.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = window_handle.as_raw() else {
        return;
    };

    // SAFETY: we're on the UI thread (window::run guarantees it) and the view
    // pointer is valid for the lifetime of this call.
    unsafe {
        let view: &NSView = appkit.ns_view.cast().as_ref();

        // Clip the view's backing layer to a rounded rect — this rounds the
        // actual rendered surface, killing the square corners.
        view.setWantsLayer(true);
        if let Some(layer) = view.layer() {
            layer.setCornerRadius(radius);
            layer.setMasksToBounds(true);
        }

        if let Some(window) = view.window() {
            window.setOpaque(false);
            window.setBackgroundColor(Some(&NSColor::clearColor()));
            window.setHasShadow(false);
            window.invalidateShadow();
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn round_window(
    _handle: &dyn iced::window::raw_window_handle::HasWindowHandle,
    _radius: f64,
) {
}

/// Puts a native `NSVisualEffectView` (system blur) *behind* the Iced content of a
/// transparent window, giving the Spotlight-style frosted glass. The Iced surface
/// stays transparent (only `glass_shell`'s faint tint sits on top), so the desktop
/// behind the window shows through, blurred. Rounded to `radius` to match the
/// window clip. Must run on the UI thread (call inside `window::run`).
///
/// The Iced view renders into its *own* backing layer, so a view merely added
/// "below" it as a subview still composites on top (covering the content), and
/// re-parenting the Iced view crashes winit (it must stay the window's content
/// view). Instead the vibrancy view is added to the content view's *superview*
/// (the window frame), positioned behind the content view — a sibling, which puts
/// the blur behind without disturbing winit.
#[cfg(target_os = "macos")]
pub fn frost_window(handle: &dyn iced::window::raw_window_handle::HasWindowHandle, radius: f64) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::{
        NSAutoresizingMaskOptions, NSView, NSVisualEffectBlendingMode, NSVisualEffectMaterial,
        NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
    };
    use objc2_foundation::MainThreadMarker;

    let Ok(window_handle) = handle.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = window_handle.as_raw() else {
        return;
    };
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };

    // SAFETY: on the UI thread (window::run guarantees it); the view pointer is
    // valid for the call. The effect view is owned by the frame view once added.
    unsafe {
        let started = std::time::Instant::now();
        let content: &NSView = appkit.ns_view.cast().as_ref();
        let Some(frame_view) = content.superview() else {
            eprintln!("[frost] content view has no superview (window frame)");
            return;
        };

        // Mark the Iced content layer non-opaque, otherwise Core Animation
        // composites it as solid and the blur behind never shows through the tint.
        content.setWantsLayer(true);
        match content.layer() {
            Some(layer) => {
                eprintln!(
                    "[frost] content layer isOpaque before = {}, sublayers = {}",
                    layer.isOpaque(),
                    layer.sublayers().map(|s| s.count()).unwrap_or(0),
                );
                layer.setOpaque(false);
                // wgpu's CAMetalLayer is a *sublayer* and is opaque by default, so
                // Core Animation composites it solid (black) regardless of the
                // surface's PostMultiplied alpha. Make every sublayer non-opaque so
                // the vibrancy behind actually shows through.
                if let Some(subs) = layer.sublayers() {
                    for i in 0..subs.count() {
                        let sub = subs.objectAtIndex(i);
                        eprintln!("[frost]   sublayer {i} isOpaque before = {}", sub.isOpaque());
                        sub.setOpaque(false);
                    }
                }
            }
            None => eprintln!("[frost] content view has NO layer"),
        }

        let fb = frame_view.bounds();
        eprintln!("[frost] frame = {:.0}x{:.0}", fb.size.width, fb.size.height);

        let effect = NSVisualEffectView::initWithFrame(mtm.alloc::<NSVisualEffectView>(), fb);
        effect.setMaterial(NSVisualEffectMaterial::HUDWindow);
        effect.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
        effect.setState(NSVisualEffectState::Active);
        effect.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        // Clip the blur to the rounded rect so its corners match the window (the
        // Iced content view is rounded separately by `round_window`).
        effect.setWantsLayer(true);
        if let Some(layer) = effect.layer() {
            layer.setCornerRadius(radius);
            layer.setMasksToBounds(true);
        }

        frame_view.addSubview_positioned_relativeTo(&effect, NSWindowOrderingMode::Below, None);
        eprintln!("[frost] frosted backdrop installed in {:?}", started.elapsed());
    }
}

#[cfg(not(target_os = "macos"))]
pub fn frost_window(
    _handle: &dyn iced::window::raw_window_handle::HasWindowHandle,
    _radius: f64,
) {
}

/// Pulls the window backing `handle` to the foreground and makes it the key
/// window.
///
/// The tray runs as a background **accessory** app (LSUIElement), so ordering a
/// window front is not enough — the *app* itself must be activated or the window
/// never becomes key and can't accept keyboard input. We activate the app and
/// then make the window key. Must run on the UI thread (call inside `window::run`).
#[cfg(target_os = "macos")]
pub fn focus_window(handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::{NSApplication, NSView};
    use objc2_foundation::MainThreadMarker;

    let Ok(window_handle) = handle.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = window_handle.as_raw() else {
        return;
    };

    // SAFETY: we're on the UI thread (window::run guarantees it) and the view
    // pointer is valid for the lifetime of this call.
    unsafe {
        if let Some(mtm) = MainThreadMarker::new() {
            // `activateIgnoringOtherApps` is soft-deprecated in favour of
            // `activate()`, but the replacement is cooperative and can silently
            // no-op; a hotkey-summoned popover must reliably steal focus.
            #[allow(deprecated)]
            NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
        }

        let view: &NSView = appkit.ns_view.cast().as_ref();
        if let Some(window) = view.window() {
            window.makeKeyAndOrderFront(None);
            // Force it to the front of the current Space even if app activation
            // was gentle (e.g. over a full-screen app).
            window.orderFrontRegardless();
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn focus_window(_handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {}

/// Makes the window backing `handle` behave like a system popover: it appears on
/// **every** Space — including the separate Space a full-screen app occupies —
/// and floats above the frontmost window there, instead of staying pinned to the
/// desktop Space it was created on.
///
/// A plain window only shows on its origin Space, so the mini window was visible
/// on the desktop but not when a full-screen app was frontmost. `canJoinAllSpaces`
/// puts it on all Spaces and `fullScreenAuxiliary` lets it join a full-screen
/// Space, but at the default floating level it still drew *behind* the
/// full-screen app's window — so we also raise it to the pop-up-menu level (the
/// level native menus/popovers use). Must run on the UI thread (`window::run`).
#[cfg(target_os = "macos")]
pub fn pin_over_spaces(handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::{NSPopUpMenuWindowLevel, NSView, NSWindowCollectionBehavior};

    let Ok(window_handle) = handle.window_handle() else {
        return;
    };
    let RawWindowHandle::AppKit(appkit) = window_handle.as_raw() else {
        return;
    };

    // SAFETY: we're on the UI thread (window::run guarantees it) and the view
    // pointer is valid for the lifetime of this call.
    unsafe {
        let view: &NSView = appkit.ns_view.cast().as_ref();
        if let Some(window) = view.window() {
            window.setCollectionBehavior(
                NSWindowCollectionBehavior::CanJoinAllSpaces
                    | NSWindowCollectionBehavior::FullScreenAuxiliary,
            );
            window.setLevel(NSPopUpMenuWindowLevel);
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn pin_over_spaces(_handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {}

/// The current mouse location in logical points, with a **top-left** origin
/// (matching how iced positions windows). `None` if it can't be read.
///
/// macOS reports the global mouse location with a bottom-left origin, so we flip
/// it against the main screen's height.
#[cfg(target_os = "macos")]
pub fn mouse_position_top_left() -> Option<(f64, f64)> {
    use objc2_app_kit::{NSEvent, NSScreen};
    use objc2_foundation::MainThreadMarker;

    let mtm = MainThreadMarker::new()?;
    let height = NSScreen::mainScreen(mtm)?.frame().size.height;
    let point = NSEvent::mouseLocation();
    Some((point.x, height - point.y))
}

#[cfg(not(target_os = "macos"))]
pub fn mouse_position_top_left() -> Option<(f64, f64)> {
    None
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
