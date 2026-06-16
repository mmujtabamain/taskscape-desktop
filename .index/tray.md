# tray_src — `taskscape-tray` (menu-bar service)

Background service with **no Dock icon** (LSUIElement). **IPC server.** Owns the
menu-bar icon, the global hotkey, and the compact mini window. Mirrors the main
app's list; mini-window edits flow back to the main app. Manifest:
[tray_src/Cargo.toml](../tray_src/Cargo.toml) (uses `tray-icon`,
`global-hotkey`, and objc2 AppKit/QuartzCore on macOS).

State, `Message` enum, and Iced wiring live in
[src/app/mod.rs](../tray_src/src/app/mod.rs). Runs as a daemon with **zero
windows at startup** — a hidden bootstrap window installs the tray + hotkey,
then the service runs windowless until the mini window is opened.

## Entry & wiring

| File                                               | Purpose                                                                                                                                                                               |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/main.rs](../tray_src/src/main.rs)             | `main()`: `app::run()`                                                                                                                                                                |
| [src/app/mod.rs](../tray_src/src/app/mod.rs)       | **`TrayApp` state struct** + **`Message` enum** (~16 variants); `boot` (adopts theme + last-open list from disk), window-settings builders (mini / confirm / bootstrap), `title`, `theme`, `view_window`, `subscription`, `run` |
| [src/app/launch.rs](../tray_src/src/app/launch.rs) | `launch_main`: open/foreground the main app — walk up the nested bundle to `Taskscape.app` and `open` it (dev: sibling `taskscape` binary)                                            |

## Behavior modules

| File                                               | Purpose                                                                                                                                                                                                                                                              |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/app/update.rs](../tray_src/src/app/update.rs) | Central `update`: task mutations, window lifecycle (mini / quit-confirm / bootstrap), tray/hotkey/IPC events, quit flow, Esc handling, mini-window drag + close-on-blur. `TrayAnchor`, `MiniSpawn` (Tray-anchored vs Mouse), `toggle_mini_window`, `mini_window_position`, `mouse_window_position`, `open_quit_confirm`, `quit` (sends `Shutdown`, then `iced::exit`) |
| [src/app/tray.rs](../tray_src/src/app/tray.rs)     | Menu-bar icon + menu: `TrayCommand { ShowWindow{rect}, Quit }`, `install`, `subscription` (icon + menu event threads), `build_icon` (procedural glyph), `main_screen_scale` (NSScreen), `round_window` (CALayer rounded corners, no shadow), `focus_window` (activate accessory app + make key), `pin_over_spaces` (collectionBehavior: all Spaces + full-screen aux), `mouse_position_top_left` (NSEvent cursor, flipped)                          |
| [src/app/hotkey.rs](../tray_src/src/app/hotkey.rs) | Global hotkey (default Option/Alt+`): `HotkeyCommand::ToggleMini`, `install`, `apply(spec, enabled)`(re-register live),`to_hotkey`, `configured`, `subscription`→ see [ipc.md](ipc.md) for live rebind via`SetHotkey`                                                |
| [src/app/sync.rs](../tray_src/src/app/sync.rs)     | IPC server glue: `broadcast`, `persist_local` (write the list to disk when the main app is offline), `handle_ipc`, `apply_remote` (`Hello`/`AddTask`/`RemoveTask`/`ToggleTaskCompleted`/`SetHotkey`), `applying_remote` echo guard → see [ipc.md](ipc.md)                                                                                   |

## View

| File                                           | Purpose                                                                                                                                                  |
| ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/app/mini.rs](../tray_src/src/app/mini.rs) | Mini window UI: `mini_view` (drag-handle header + composer + scrollable task list + footer), `mini_task_row`, and `quit_confirm_view` (draggable borderless popover) |

## Other

| File                                             | Purpose                                                                         |
| ------------------------------------------------ | ------------------------------------------------------------------------------- |
| [macos/Info.plist](../tray_src/macos/Info.plist) | Tray bundle metadata; **`LSUIElement = true`** → background agent, no Dock icon |

## Notes

- The mini window is transparent + borderless; macOS-native rounding/shadow
  tweaks (`round_window`) and icon-anchored positioning
  (`mini_window_position` × `main_screen_scale`) make it look native. It also
  joins every Space and floats over full-screen apps (`pin_over_spaces`), not
  just the desktop Space it was created on (`Level::AlwaysOnTop` alone only
  affects z-order within a Space). All `#[cfg(target_os = "macos")]`-gated.
- The mini window behaves like a native popover: the tray click anchors it under
  the icon, the hotkey drops its top-left corner at the cursor
  (`mouse_window_position` × `mouse_position_top_left`), opening activates the
  accessory app so it takes keyboard focus (`focus_window`) and puts the cursor
  in the task input (`mini::MINI_INPUT_ID` via `operation::focus`), its header is
  a drag handle (`DragMini` → `window::drag`), and it auto-closes when it loses
  focus (`mini_focused` guard, mirroring `confirm_focused`).
- Tray/hotkey background threads forward OS events into the Iced `subscription`
  as `Message`s.
- Mini-window task edits call `broadcast()` → the main app applies and re-saves
  them (the main app owns the on-disk list while linked). When the main app is
  **offline**, the tray instead writes the list to disk itself (`persist_local`)
  so edits survive; `boot` loads the last-open list so a standalone tray has one
  to edit. The two never write at once — `persist_local` no-ops while linked.
