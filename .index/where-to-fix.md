# Where to fix it

"I want to change X" → the file(s) to open. Skim this before grepping. Paths are
relative to the repo root. Deeper file detail: [common.md](common.md),
[main.md](main.md), [tray.md](tray.md).

> Rule of thumb: **data/model/IPC/theme** changes live in `common/`; **main
> window** behavior in `main_src/`; **tray, hotkey, mini window** in `tray_src/`.
> A shared contract (IPC message, storage field, widget signature) almost always
> means editing `common/` **and** both `*/sync.rs` or both call sites.

## Tasks & lists (the data)

| Goal                                                          | Edit                                                                                                     |
| ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Change the `Task` shape (fields, defaults)                    | [common/src/models/task.rs](../common/src/models/task.rs) (+ persistence in storage.rs, + IPC if synced) |
| Attachment model / image-vs-file kind                         | [common/src/models/attachment.rs](../common/src/models/attachment.rs)                                    |
| Attachment files: copy/link/screenshot/open helpers           | [common/src/attachments.rs](../common/src/attachments.rs) (writes to `~/.taskscape/files/`)              |
| Task collection ops (add/remove/toggle/clear/attach/counts)   | [common/src/tasklist.rs](../common/src/tasklist.rs)                                                      |
| How/where lists, attachments & config are saved on disk       | [common/src/storage.rs](../common/src/storage.rs) (`~/.taskscape/`)                                      |
| Main-window task actions (add/clear/undo/redo, import/export) | [main_src/src/app/actions.rs](../main_src/src/app/actions.rs)                                            |
| Undo/redo snapshot contents                                   | [main_src/src/app/snapshot.rs](../main_src/src/app/snapshot.rs)                                          |
| Counts shown in the UI                                        | [main_src/src/app/queries.rs](../main_src/src/app/queries.rs)                                            |

## Main window UI

| Goal                                                          | Edit                                                                                                                |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Overall layout (sidebar + content + status bar + modals)      | [main_src/src/app/view/mod.rs](../main_src/src/app/view/mod.rs)                                                     |
| Top header of an open list (title, controls)                  | [main_src/src/app/view/header.rs](../main_src/src/app/view/header.rs)                                               |
| Sidebar list rail/panel, rename/clear-all modals, empty state | [main_src/src/app/view/lists.rs](../main_src/src/app/view/lists.rs)                                                 |
| Task composer + actions row (incl. attach/screenshot buttons) | [main_src/src/app/view/tasks.rs](../main_src/src/app/view/tasks.rs)                                                 |
| Task cards / scroll list (incl. per-task attach buttons+chips) | [main_src/src/app/view/workspace.rs](../main_src/src/app/view/workspace.rs)                                        |
| Attach-file/screenshot logic (both apps)                      | `app/update.rs` + main's [actions.rs](../main_src/src/app/actions.rs) (`attach_to_target`, `launch_file_attach_dialog`); chip widget [t_attachment.rs](../common/src/widgets/t_attachment.rs) |
| Settings page                                                 | [main_src/src/app/view/settings.rs](../main_src/src/app/view/settings.rs)                                           |
| Add a new `Message` / route an event                          | [main_src/src/app/mod.rs](../main_src/src/app/mod.rs) (enum) + [update.rs](../main_src/src/app/update.rs) (handler) |
| Keyboard shortcuts (Cmd+Z/E/O/N/L/T, Esc)                     | [main_src/src/app/update.rs](../main_src/src/app/update.rs)                                                         |
| Native menu bar items                                         | [main_src/src/app/native_menu.rs](../main_src/src/app/native_menu.rs)                                               |

## Tray, mini window, hotkey

| Goal                                                       | Edit                                                                                                                     |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Mini window layout/content                                 | [tray_src/src/app/mini.rs](../tray_src/src/app/mini.rs)                                                                  |
| Mini window position/size/open-close behavior              | [tray_src/src/app/update.rs](../tray_src/src/app/update.rs) (`toggle_mini_window`, `mini_window_position`, `mouse_window_position`)               |
| Mini window drag / focus-on-open / close-on-blur           | [tray_src/src/app/update.rs](../tray_src/src/app/update.rs) (`DragMini`, `mini_focused`) + [tray_src/src/app/tray.rs](../tray_src/src/app/tray.rs) (`focus_window`, `mouse_position_top_left`) |
| Menu-bar icon, its menu, the drawn glyph                   | [tray_src/src/app/tray.rs](../tray_src/src/app/tray.rs)                                                                  |
| Mini window rounded corners / no shadow / Retina anchoring | [tray_src/src/app/tray.rs](../tray_src/src/app/tray.rs) (`round_window`, `main_screen_scale`)                            |
| Global hotkey default / registration / rebind              | [tray_src/src/app/hotkey.rs](../tray_src/src/app/hotkey.rs) + [common/src/hotkey.rs](../common/src/hotkey.rs) (the spec) |
| Quit confirmation popover                                  | [tray_src/src/app/mini.rs](../tray_src/src/app/mini.rs) (`quit_confirm_view`) + update.rs (`open_quit_confirm`, `quit`)  |

## Cross-process / IPC

| Goal                                        | Edit                                                                                                                                                                                             |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Add/change an IPC message                   | [common/src/ipc/mod.rs](../common/src/ipc/mod.rs) **and** [main_src/.../sync.rs](../main_src/src/app/sync.rs) **and** [tray_src/.../sync.rs](../tray_src/src/app/sync.rs) — see [ipc.md](ipc.md) |
| Socket path / framing / transport           | [common/src/ipc/mod.rs](../common/src/ipc/mod.rs), [client.rs](../common/src/ipc/client.rs), [server.rs](../common/src/ipc/server.rs)                                                            |
| How a mutation mirrors to the other process | the matching `broadcast` / `apply_remote` in `*/sync.rs` (mind the `applying_remote` echo guard)                                                                                                 |

## Look & feel

| Goal                                                         | Edit                                                                                                       |
| ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Colors / palette / dark-vs-light                             | [common/src/thememanager/palette.rs](../common/src/thememanager/palette.rs) — see [theming.md](theming.md) |
| A specific element's style (button/container/input/dropdown) | the matching `thememanager/*.rs` factory                                                                   |
| A reusable widget's behavior/markup                          | the `common/src/widgets/t_*.rs` file — see [theming.md](theming.md)                                        |
| Fonts                                                        | [common/src/utils/fonts.rs](../common/src/utils/fonts.rs) + [assets/fonts/](../assets/fonts/)              |

## Startup, build, packaging

| Goal                                              | Edit                                                                                                                               |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Main app boot / window open / reopen-last-list    | [main_src/src/app/mod.rs](../main_src/src/app/mod.rs) (`boot`)                                                                     |
| How the main app spawns the tray                  | [main_src/src/app/launch.rs](../main_src/src/app/launch.rs)                                                                        |
| How the tray opens/foregrounds the main app       | [tray_src/src/app/launch.rs](../tray_src/src/app/launch.rs)                                                                        |
| Dev build/run flow                                | [run-dev.sh](../run-dev.sh)                                                                                                        |
| Release `.app` bundle assembly                    | [make-app.sh](../make-app.sh) + the two `macos/Info.plist` files                                                                   |
| Bundle id / version / min OS / Dock-icon behavior | [main_src/macos/Info.plist](../main_src/macos/Info.plist), [tray_src/macos/Info.plist](../tray_src/macos/Info.plist) (LSUIElement) |
| Dependencies / crate config                       | the per-crate `Cargo.toml` + workspace [Cargo.toml](../Cargo.toml)                                                                 |
