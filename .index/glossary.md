# Glossary

Project-specific terms and where they live.

| Term                         | Meaning                                                                                                                              |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| **main app**                 | The `taskscape` binary ([main_src/](../main_src/)) — the full task window. IPC client, source of truth.                              |
| **tray / tray service**      | The `taskscape-tray` binary ([tray_src/](../tray_src/)) — background menu-bar service. IPC server, no Dock icon.                     |
| **common**                   | The `taskscape-common` library ([common/](../common/)), shared by both binaries.                                                     |
| **mini window**              | The compact popup the tray shows under the menu-bar icon ([tray_src/src/app/ui/mini.rs](../tray_src/src/app/ui/mini.rs)).                  |
| **bootstrap window**         | A hidden window the tray opens at startup only to install the tray icon + hotkey, then runs windowless.                              |
| **daemon**                   | Iced run mode where windows open programmatically and the process survives window close. Both binaries use it.                       |
| **source of truth**          | The main app owns the canonical list; the tray mirrors it and sends edits back.                                                      |
| **`Hello`**                  | IPC message carrying the full list for a wholesale sync (on connect / list switch / after bulk ops). See [ipc.md](ipc.md).           |
| **`applying_remote`**        | Per-process flag set while applying an IPC-received mutation, so it isn't echoed back into a loop.                                   |
| **`resync_tray`**            | Main-app helper that re-sends a full `Hello` after bulk changes instead of per-item messages.                                        |
| **`HotkeySpec`**             | Serializable hotkey binding (modifiers + W3C key code), shared via config and IPC ([common/src/hotkey.rs](../common/src/hotkey.rs)). |
| **`ThemeMode`**              | `Dark` / `Light`; persisted in config; drives every style factory. See [theming.md](theming.md).                                     |
| **`AppPalette` / `tokens`**  | The named theme colors and the function that returns them for a `ThemeMode`.                                                         |
| **`ButtonKind`**             | `Primary` / `Ghost` / `Icon` / `Plain` — selects a button's styling.                                                                 |
| **`t_*` widgets**            | The reusable themed Iced builders in `common::ui` (e.g. `t_button`, `t_input_box`).                                             |
| **`TrayAnchor` / icon rect** | Menu-bar icon rectangle (physical pixels) the tray uses to position the mini window.                                                 |
| **LSUIElement**              | macOS Info.plist flag making the tray a background agent with no Dock icon.                                                          |
| **`Taskscape` / `TrayApp`**  | The Iced state structs of the main app and tray, respectively.                                                                       |
