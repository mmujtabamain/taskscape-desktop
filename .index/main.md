# main_src — `taskscape` (main window)

The full task window. **IPC client** and **source of truth** for the list. Owns
the native menu, undo/redo, settings, and file import/export. Manifest:
[main_src/Cargo.toml](../main_src/Cargo.toml).

State, `Message` enum, and Iced wiring live in
[src/app/mod.rs](../main_src/src/app/mod.rs); `update` is split across sibling
modules by concern.

## Entry & wiring

| File                                                         | Purpose                                                                                                                                                                             |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/main.rs](../main_src/src/main.rs)                       | `main()`: `app::ensure_tray_running()` then `app::run()`                                                                                                                            |
| [src/app/mod.rs](../main_src/src/app/mod.rs)                 | **`Taskscape` state struct** (~22 fields) + **`Message` enum** (~45 variants); `boot`, `title`, `theme`, `view_window`, `subscription`, `run`; type aliases `AppElement`, `AppTask` |
| [src/app/launch.rs](../main_src/src/app/launch.rs)           | `ensure_tray_running`: spawn the tray binary if the socket isn't served (packaged: the nested `.app`; dev: sibling `taskscape-tray`)                                                |
| [src/app/native_menu.rs](../main_src/src/app/native_menu.rs) | Native menu bar (muda): `NativeMenuCommand`, `install_for_window`, `build_menu`, `subscription` (macOS ✓ / Windows ✓ / Linux ✗)                                                     |
| [src/app/chrome.rs](../main_src/src/app/chrome.rs)           | macOS window chrome: `apply` — transparent system title bar + full-size content view (keeps native traffic lights) and the frosted-glass `NSVisualEffectView` *behind* the transparent Iced surface. `#[cfg(macos)]`-gated, non-macOS stub. Applied on `WindowOpened` |

## update / state logic

| File                                                   | Purpose                                                                                                                                                                                                                                 |
| ------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/app/update.rs](../main_src/src/app/update.rs)     | Central `update`: routes every `Message` to an action + persist + IPC. Hosts keyboard shortcuts (Cmd+Z/⇧Z undo-redo, Cmd+E export, Cmd+O import, Cmd+N new, Cmd+L panel, Cmd+T theme, Esc), window/menu events, and live hotkey capture |
| [src/app/actions.rs](../main_src/src/app/actions.rs)   | Mutations + persistence + file dialogs: task add/remove/toggle/clear, undo/redo, list open/create/delete/rename/refresh, `persist_current`/`persist_settings`/`remember_open`, import/export dialog flow                                |
| [src/app/queries.rs](../main_src/src/app/queries.rs)   | Read-only views for the UI: `visible_tasks`, `open_count`, `completed_count`, `total_count`                                                                                                                                             |
| [src/app/snapshot.rs](../main_src/src/app/snapshot.rs) | `AppSnapshot` (task vector) for the undo/redo stacks                                                                                                                                                                                    |
| [src/app/sync.rs](../main_src/src/app/sync.rs)         | IPC glue: `broadcast`, `resync_tray` (full `Hello`), `send_hotkey_config`, `handle_ipc`, `apply_remote` (with `applying_remote` echo guard) → see [ipc.md](ipc.md)                                                                      |

## View — `src/app/ui/`

| File                                                       | Purpose                                                                                                                          |
| ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| [ui/mod.rs](../main_src/src/app/ui/mod.rs)             | Top-level composer: `view_root` (title bar + sidebar + content + status bar + modals), `workspace_or_prompt`, `status_bar`, `list_sidebar`; the whole window is one `frosted_shell` surface |
| [ui/titlebar.rs](../main_src/src/app/ui/titlebar.rs)   | Custom title bar (`title_bar`): draggable full-width strip (`Message::DragWindow` → `window::drag`) with a traffic-light gutter + centered wordmark, replacing the transparent system bar |
| [ui/header.rs](../main_src/src/app/ui/header.rs)       | Open-list top bar: title, count, control buttons (panel/import/export/theme/undo/redo)                                           |
| [ui/lists.rs](../main_src/src/app/ui/lists.rs)         | Sidebar rail (collapsed) / panel (expanded), list rows, rename + clear-all modals, empty-state prompt; exports `RENAME_INPUT_ID` |
| [ui/tasks.rs](../main_src/src/app/ui/tasks.rs)         | Task workspace: header + composer row + task list + actions row                                                                  |
| [ui/workspace.rs](../main_src/src/app/ui/workspace.rs) | Scrollable task list + flat per-task row (checkbox, title, status, attach/delete)                                                           |
| [ui/settings.rs](../main_src/src/app/ui/settings.rs)   | Settings page (replaces workspace): theme select, hotkey capture/reset, reopen-last + confirm-clear + hotkey-enabled toggles     |

## Other

| File                                             | Purpose                                                                              |
| ------------------------------------------------ | ------------------------------------------------------------------------------------ |
| [macos/Info.plist](../main_src/macos/Info.plist) | Main `.app` bundle metadata (bundle id `com.taskscape.app`, version, min OS, Hi-DPI) |

## Notes

- `update` returns `iced::Task<Message>` for async work (file pickers, window
  focus/minimize).
- After every list/task mutation the state persists immediately and (usually)
  broadcasts over IPC — keep both in sync when adding a mutation.
- Modals (rename, clear-all) are `stack!`ed over the shell with an opaque
  backdrop.
