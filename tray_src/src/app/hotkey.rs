//! OS-global keyboard shortcut for toggling the mini window.
//!
//! Uses the `global-hotkey` crate (same maintainer/event model as `tray-icon`)
//! to register a system-wide hotkey that fires even when Taskscape is not the
//! focused application. The binding is Option/Alt+` on every platform.
//!
//! This mirrors `tray.rs`: the manager must be kept alive for the hotkey to stay
//! registered, so we stash it in thread-local storage, and a subscription
//! forwards press events into the iced runtime.

use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;

use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use std::cell::{Cell, RefCell};

/// Commands produced by the global hotkey.
#[derive(Debug, Clone, Copy)]
pub enum HotkeyCommand {
    /// Toggle the compact mini window.
    ToggleMini,
}

/// The mini-window toggle: Option/Alt+` (the backtick / `~` key).
fn mini_toggle_hotkey() -> HotKey {
    HotKey::new(Some(Modifiers::ALT), Code::Backquote)
}

thread_local! {
    // The manager must stay alive for the hotkey to remain registered.
    static HOTKEY_MANAGER: RefCell<Option<GlobalHotKeyManager>> = const { RefCell::new(None) };
    static HOTKEY_INSTALLED: Cell<bool> = const { Cell::new(false) };
}

/// Registers the global hotkey. Must be called on the main (UI) thread.
pub fn install() -> Result<(), String> {
    if HOTKEY_INSTALLED.with(Cell::get) {
        return Ok(());
    }

    let manager = GlobalHotKeyManager::new()
        .map_err(|e| format!("failed to create hotkey manager: {e}"))?;

    manager
        .register(mini_toggle_hotkey())
        .map_err(|e| format!("failed to register hotkey: {e}"))?;

    HOTKEY_MANAGER.with(|slot| *slot.borrow_mut() = Some(manager));
    HOTKEY_INSTALLED.with(|flag| flag.set(true));
    Ok(())
}

/// Subscription that delivers global hotkey presses as [`HotkeyCommand`]s.
pub fn subscription() -> Subscription<HotkeyCommand> {
    Subscription::run(hotkey_event_stream)
}

/// Forwards hotkey events from the global receiver into an async stream without
/// blocking the executor, mirroring the tray plumbing.
fn hotkey_event_stream() -> impl iced::futures::Stream<Item = HotkeyCommand> {
    let (tx, rx) = mpsc::channel::<HotkeyCommand>(64);

    if let Err(error) = std::thread::Builder::new()
        .name("taskscape-hotkey-events".into())
        .spawn(move || {
            let mut tx = tx;
            let receiver = GlobalHotKeyEvent::receiver();
            // Recomputed (not read from thread-local state) because this runs on
            // a separate thread; the id is deterministic from mods + key.
            let mini_id = mini_toggle_hotkey().id();

            while let Ok(event) = receiver.recv() {
                // Fire once, on press, for our registered hotkey only.
                if event.state == HotKeyState::Pressed && event.id == mini_id {
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
