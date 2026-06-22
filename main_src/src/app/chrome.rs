//! Native macOS window chrome for the main task window.
//!
//! The main window keeps the system traffic lights and native resize/shadow, but
//! we **hand the title bar to ourselves**: the system bar is made transparent and
//! the content view fills the whole frame, so the Iced view draws under the
//! traffic lights and we render our own bar there (see `ui::titlebar`).
//!
//! It is also **frosted glass** (matching the mini window): an
//! `NSVisualEffectView` is inserted *behind* the transparent Iced surface — the
//! blur renders **before** the content (behind it), never on top. The Iced
//! surface is opened transparent (`transparent: true`) and the shell paints only a
//! faint tint (`frosted_shell`), so the desktop shows through, blurred.
//!
//! Non-macOS builds are no-ops (the app targets macOS; the stub keeps it
//! compiling elsewhere).

/// Applies the transparent title bar + frosted backdrop to the main window.
/// Must run on the UI thread — call inside `window::run`.
#[cfg(target_os = "macos")]
pub fn apply(handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {
    use iced::window::raw_window_handle::RawWindowHandle;
    use objc2_app_kit::{
        NSAutoresizingMaskOptions, NSColor, NSView, NSVisualEffectBlendingMode,
        NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
        NSWindowStyleMask, NSWindowTitleVisibility,
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
    // valid for the call. The effect view is owned by the content view once added.
    unsafe {
        let content: &NSView = appkit.ns_view.cast().as_ref();
        let Some(window) = content.window() else {
            return;
        };

        // 1) Hand the title bar to us. Make the system bar transparent, hide its
        //    title, and let the content view run full height (under the traffic
        //    lights). The traffic lights stay native; we draw the rest in Iced.
        window.setTitlebarAppearsTransparent(true);
        window.setTitleVisibility(NSWindowTitleVisibility::Hidden);
        window.setStyleMask(window.styleMask() | NSWindowStyleMask::FullSizeContentView);

        // 2) Make the window non-opaque so the blur shows through the transparent
        //    Iced surface. Keep the native shadow and rounded corners.
        window.setOpaque(false);
        window.setBackgroundColor(Some(&NSColor::clearColor()));

        // 3) The frosted backdrop. The Iced view renders into its *own* backing
        //    layer, so a subview added to it still composites on top (covering the
        //    content), and re-parenting the Iced view crashes winit (it must stay
        //    the window's content view). Instead, add the vibrancy view to the
        //    content view's *superview* (the window frame), positioned behind the
        //    content view — a sibling, so the blur sits behind without disturbing
        //    winit's view. It autoresizes to fill the frame.
        let Some(frame_view) = content.superview() else {
            return;
        };
        let effect = NSVisualEffectView::initWithFrame(
            mtm.alloc::<NSVisualEffectView>(),
            frame_view.bounds(),
        );
        effect.setMaterial(NSVisualEffectMaterial::HUDWindow);
        effect.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
        effect.setState(NSVisualEffectState::Active);
        effect.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        frame_view.addSubview_positioned_relativeTo(&effect, NSWindowOrderingMode::Below, None);
    }
}

#[cfg(not(target_os = "macos"))]
pub fn apply(_handle: &dyn iced::window::raw_window_handle::HasWindowHandle) {}
