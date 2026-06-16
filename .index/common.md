# common — `taskscape-common` (shared library)

Non-executable library shared by both binaries. Manifest:
[common/Cargo.toml](../common/Cargo.toml). Crate root:
[common/src/lib.rs](../common/src/lib.rs) — declares modules `hotkey`, `ipc`,
`models`, `storage`, `tasklist`, `thememanager`, `utils`, `widgets`; re-exports
`Task` and `TaskList`.

> Theming and widgets have their own page: [theming.md](theming.md).
> The IPC protocol has its own page: [ipc.md](ipc.md).

## Core

| File                                               | Purpose                                               | Key items                                                                                                                                                                         |
| -------------------------------------------------- | ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/lib.rs](../common/src/lib.rs)                 | Crate root, module list, re-exports                   | `pub use` `Task`, `TaskList`                                                                                                                                                      |
| [src/models/mod.rs](../common/src/models/mod.rs)   | Re-export aggregator                                  | re-exports `Task`                                                                                                                                                                 |
| [src/models/task.rs](../common/src/models/task.rs) | The task entity (serde)                               | `Task { title, completed }`, `new`, `is_completed`                                                                                                                                |
| [src/tasklist.rs](../common/src/tasklist.rs)       | In-memory task collection + mutations                 | `TaskList`; `add`/`remove`/`set_completed`/`clear_completed`/`clear`/`total`/`completed`/`open`/`enumerated`                                                                      |
| [src/storage.rs](../common/src/storage.rs)         | On-disk persistence (JSON)                            | `Config`, `ListEntry`, `TaskListFile`; `list_all`/`load`/`save`/`rename`/`delete`/`import_from`/`export_to`/`load_config`/`save_config`; `app_data_dir`/`lists_dir`/`config_path` |
| [src/hotkey.rs](../common/src/hotkey.rs)           | Serializable hotkey binding (shared via IPC + config) | `HotkeySpec { alt, ctrl, shift, meta, code }`; `default_mini_toggle`, `has_strong_modifier`, `label`, `is_modifier_code`                                                          |

Storage location: `~/Library/Application Support/Taskscape/` — `lists/*.json`
plus `config.json`.

## IPC — `src/ipc/` → see [ipc.md](ipc.md)

| File                                         | Purpose                                                        |
| -------------------------------------------- | -------------------------------------------------------------- |
| [ipc/mod.rs](../common/src/ipc/mod.rs)       | `IpcMessage`, `IpcInbound`, `socket_path`, encode/read framing |
| [ipc/client.rs](../common/src/ipc/client.rs) | Main-app client: `send`, `subscription` (auto-reconnect)       |
| [ipc/server.rs](../common/src/ipc/server.rs) | Tray server: `send`, `subscription` (one client)               |

## Theme manager — `src/thememanager/` → see [theming.md](theming.md)

| File                                                                     | Purpose                                                                             |
| ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| [thememanager/mod.rs](../common/src/thememanager/mod.rs)                 | Aggregator + re-exports of all style factories                                      |
| [thememanager/theme_mode.rs](../common/src/thememanager/theme_mode.rs)   | `ThemeMode { Dark, Light }` (serde, persisted)                                      |
| [thememanager/palette.rs](../common/src/thememanager/palette.rs)         | `AppPalette`; `app_theme(mode)`, `tokens(mode)`                                     |
| [thememanager/helpers.rs](../common/src/thememanager/helpers.rs)         | Color/border/shadow/gradient primitives                                             |
| [thememanager/button.rs](../common/src/thememanager/button.rs)           | `button_style(mode, kind)`                                                          |
| [thememanager/button_kind.rs](../common/src/thememanager/button_kind.rs) | `ButtonKind { Primary, Ghost, Icon, Plain }`                                        |
| [thememanager/container.rs](../common/src/thememanager/container.rs)     | `shell`/`mini_shell`/`panel_alt`/`empty_state`/`modal_*`/`sidebar` container styles |
| [thememanager/pick_list.rs](../common/src/thememanager/pick_list.rs)     | `pick_list_style(mode)`                                                             |
| [thememanager/text_input.rs](../common/src/thememanager/text_input.rs)   | `text_input_style(mode)`                                                            |

## Widget toolkit — `src/widgets/` → see [theming.md](theming.md)

| File                                                                     | Purpose                                                         |
| ------------------------------------------------------------------------ | --------------------------------------------------------------- |
| [widgets/mod.rs](../common/src/widgets/mod.rs)                           | Aggregator/re-exports + debug-outline helpers                   |
| [widgets/lucide_icon.rs](../common/src/widgets/lucide_icon.rs)           | `lucide_icon(icon, size, color)`                                |
| [widgets/t_typography.rs](../common/src/widgets/t_typography.rs)         | `t_heading` / `t_body` / `t_caption`                            |
| [widgets/t_button.rs](../common/src/widgets/t_button.rs)                 | `t_button(...)` (icon + label + kind)                           |
| [widgets/t_icon_button.rs](../common/src/widgets/t_icon_button.rs)       | `t_icon_button` (bordered) / `t_icon_button_ghost` (borderless) |
| [widgets/t_input_box.rs](../common/src/widgets/t_input_box.rs)           | `t_input_box(...)` themed text field                            |
| [widgets/t_dropdown.rs](../common/src/widgets/t_dropdown.rs)             | `t_dropdown(...)` themed select                                 |
| [widgets/t_editable_title.rs](../common/src/widgets/t_editable_title.rs) | `t_editable_title(...)` inline-editable title; `TITLE_INPUT_ID` |
| [widgets/t_metric_card.rs](../common/src/widgets/t_metric_card.rs)       | `t_metric_card(value, label)`                                   |
| [widgets/t_small_chip.rs](../common/src/widgets/t_small_chip.rs)         | `t_small_chip(label, accent)`                                   |

## Utils — `src/utils/`

| File                                           | Purpose                                                               |
| ---------------------------------------------- | --------------------------------------------------------------------- |
| [utils/mod.rs](../common/src/utils/mod.rs)     | Module declarations                                                   |
| [utils/fonts.rs](../common/src/utils/fonts.rs) | Embedded TTF bytes + `inter_regular()` / `poppins_semibold()` `Font`s |
