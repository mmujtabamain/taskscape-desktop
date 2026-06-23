//! Native macOS window chrome for the main task window.
//!
//! The main window keeps the system traffic lights and native resize/shadow, but
//! we **hand the title bar to ourselves**: the system bar is made transparent and
//! the content view fills the whole frame, so the Iced view draws under the
//! traffic lights and we render our own bar there (see `ui::titlebar`).
//!
//! Non-macOS builds are no-ops (the app targets macOS; the stub keeps it
//! compiling elsewhere).

/// Applies the custom title bar to the main window: the system title bar is made
/// transparent with its title hidden and the content view runs full height, so
/// Iced draws our own bar over the native traffic lights. Must run on the UI
/// thread — call inside `window::run`.
#[cfg(target_os = "macos")]
pub fn apply(handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::{NSView, NSWindowStyleMask, NSWindowTitleVisibility};

    let Ok(window_handle) = handle.window_handle() else {
        eprintln!("[chrome] no window handle");
        return;
    };
    let RawWindowHandle::AppKit(appkit) = window_handle.as_raw() else {
        eprintln!("[chrome] not an AppKit handle");
        return;
    };

    // SAFETY: on the UI thread (window::run guarantees it); the view pointer is
    // valid for the call.
    unsafe {
        let content: &NSView = appkit.ns_view.cast().as_ref();
        let Some(window) = content.window() else {
            eprintln!("[chrome] content view has no window");
            return;
        };

        // Hand the title bar to us. Make the system bar transparent, hide its
        // title, and let the content view run full height (under the traffic
        // lights). The traffic lights stay native; we draw the rest in Iced.
        window.setTitlebarAppearsTransparent(true);
        window.setTitleVisibility(NSWindowTitleVisibility::Hidden);
        window.setStyleMask(window.styleMask() | NSWindowStyleMask::FullSizeContentView);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn apply(_handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {}
