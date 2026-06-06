//! OS-global keyboard shortcut for toggling the mini window.
//!
//! Uses the `global-hotkey` crate (same maintainer/event model as `tray-icon`)
//! to register a system-wide hotkey that fires even when Taskscape is not the
//! focused application. The binding is read from the saved config (defaulting to
//! Option/Alt+`) and can be changed live from the main app's settings via
//! [`apply`].
//!
//! This mirrors `tray.rs`: the manager must be kept alive for the hotkey to stay
//! registered, so we stash it in thread-local storage, and a subscription
//! forwards press events into the iced runtime.

use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;

use common::hotkey::HotkeySpec;
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::HotKey,
};
use std::cell::RefCell;
use std::str::FromStr;

/// Commands produced by the global hotkey.
#[derive(Debug, Clone, Copy)]
pub enum HotkeyCommand {
    /// Toggle the compact mini window.
    ToggleMini,
}

thread_local! {
    // The manager must stay alive for the hotkey to remain registered.
    static HOTKEY_MANAGER: RefCell<Option<GlobalHotKeyManager>> = const { RefCell::new(None) };
    // The currently-registered hotkey, so we can unregister it before swapping in
    // a new one. `None` means nothing is registered (manager absent or disabled).
    static CURRENT_HOTKEY: RefCell<Option<HotKey>> = const { RefCell::new(None) };
}

/// Builds a `global-hotkey` `HotKey` from our serializable spec by composing the
/// `"Mod+Mod+Key"` string the crate's parser understands (its key names match the
/// W3C `code` names we store).
fn to_hotkey(spec: &HotkeySpec) -> Result<HotKey, String> {
    let mut parts: Vec<&str> = Vec::new();
    if spec.alt {
        parts.push("Alt");
    }
    if spec.ctrl {
        parts.push("Control");
    }
    if spec.shift {
        parts.push("Shift");
    }
    if spec.meta {
        parts.push("Super");
    }
    parts.push(&spec.code);
    HotKey::from_str(&parts.join("+")).map_err(|e| format!("invalid hotkey: {e}"))
}

/// Reads the saved hotkey binding + enabled flag from the config, falling back to
/// the built-in default.
fn configured() -> (HotkeySpec, bool) {
    let config = common::storage::load_config();
    let spec = config.hotkey.unwrap_or_else(HotkeySpec::default_mini_toggle);
    (spec, config.hotkey_enabled)
}

/// Registers the configured global hotkey. Must be called on the main (UI) thread.
pub fn install() -> Result<(), String> {
    HOTKEY_MANAGER.with(|slot| {
        if slot.borrow().is_none() {
            let manager = GlobalHotKeyManager::new()
                .map_err(|e| format!("failed to create hotkey manager: {e}"))?;
            *slot.borrow_mut() = Some(manager);
        }
        Ok::<(), String>(())
    })?;

    let (spec, enabled) = configured();
    apply(Some(spec), enabled)
}

/// Swaps the registered hotkey to `spec` (or unregisters when `enabled` is false
/// or `spec` is `None`). Must run on the same (UI) thread that owns the manager,
/// i.e. the `update` loop. Best-effort and idempotent.
pub fn apply(spec: Option<HotkeySpec>, enabled: bool) -> Result<(), String> {
    HOTKEY_MANAGER.with(|slot| {
        let guard = slot.borrow();
        let Some(manager) = guard.as_ref() else {
            // No manager yet (install() hasn't run): nothing to do.
            return Ok(());
        };

        // Drop whatever is currently bound.
        CURRENT_HOTKEY.with(|cur| {
            if let Some(existing) = cur.borrow_mut().take() {
                let _ = manager.unregister(existing);
            }
        });

        if !enabled {
            return Ok(());
        }

        let spec = spec.unwrap_or_else(HotkeySpec::default_mini_toggle);
        let hotkey = to_hotkey(&spec)?;
        manager
            .register(hotkey)
            .map_err(|e| format!("failed to register hotkey: {e}"))?;
        CURRENT_HOTKEY.with(|cur| *cur.borrow_mut() = Some(hotkey));
        Ok(())
    })
}

/// Subscription that delivers global hotkey presses as [`HotkeyCommand`]s.
pub fn subscription() -> Subscription<HotkeyCommand> {
    Subscription::run(hotkey_event_stream)
}

/// Forwards hotkey events from the global receiver into an async stream without
/// blocking the executor, mirroring the tray plumbing. We only ever register one
/// hotkey at a time in this process, so *any* press the receiver sees is ours —
/// no id filtering, which keeps this correct across live re-registration.
fn hotkey_event_stream() -> impl iced::futures::Stream<Item = HotkeyCommand> {
    let (tx, rx) = mpsc::channel::<HotkeyCommand>(64);

    if let Err(error) = std::thread::Builder::new()
        .name("taskscape-hotkey-events".into())
        .spawn(move || {
            let mut tx = tx;
            let receiver = GlobalHotKeyEvent::receiver();

            while let Ok(event) = receiver.recv() {
                if event.state == HotKeyState::Pressed {
                    if iced::futures::executor::block_on(tx.send(HotkeyCommand::ToggleMini)).is_err()
                    {
                        // The subscription stream was dropped.
                        break;
                    }
                }
            }
        })
    {
        eprintln!("Failed to spawn hotkey event thread: {error}");
    }

    rx
}
