//! Motion presets for the redesign.
//!
//! All eased motion uses the ease-out-quint curve (no bounce/elastic). Per-widget
//! animations live in the widgets' own `tree::State` (see `components::interactive`)
//! and drive their own redraws; app-state moments use a `window::frames()` tick.
//! Every duration runs through [`gated`] so the Settings "Reduce motion" toggle can
//! collapse it to instant.

use iced::animation::{Animation, Easing};
use iced::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub use iced::time::Instant;

/// Process-wide "reduce motion" flag. Set from config/Settings so widgets don't
/// each need it threaded through their signatures; read by `interactive` and any
/// app-state tween.
static REDUCE_MOTION: AtomicBool = AtomicBool::new(false);

pub fn set_reduce_motion(reduce: bool) {
    REDUCE_MOTION.store(reduce, Ordering::Relaxed);
}

pub fn reduce_motion() -> bool {
    REDUCE_MOTION.load(Ordering::Relaxed)
}

/// The one easing curve for the whole system.
pub const EASING: Easing = Easing::EaseOutQuint;

/// The snappiest feedback (press in/out).
pub const PRESS: Duration = Duration::from_millis(90);
/// Hover / focus and most state changes.
pub const QUICK: Duration = Duration::from_millis(120);
/// Default for most transitions (hover, reveal, open/close).
pub const BASE: Duration = Duration::from_millis(180);
/// Larger moments (theme cross-fade).
pub const SLOW: Duration = Duration::from_millis(250);

/// Collapse a duration to instant when reduced motion is on.
pub fn gated(duration: Duration, reduce_motion: bool) -> Duration {
    if reduce_motion {
        Duration::ZERO
    } else {
        duration
    }
}

/// A `0.0 → 1.0` progress animation at the given duration, eased and reduce-motion
/// aware. The building block for hover/press/focus and reveal tweens.
pub fn progress(duration: Duration, reduce_motion: bool) -> Animation<f32> {
    Animation::new(0.0)
        .easing(EASING)
        .duration(gated(duration, reduce_motion))
}
