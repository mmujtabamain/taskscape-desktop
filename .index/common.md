# common — `taskscape-common` (shared library)

Non-executable library shared by both binaries. Manifest:
[common/Cargo.toml](../common/Cargo.toml) (iced built with the `image` +
`advanced` features — the latter for the custom animated widget). Crate root:
[common/src/lib.rs](../common/src/lib.rs) — declares modules `attachments`,
`hotkey`, `ipc`, `models`, `storage`, `tasklist`, `ui`, `utils`; re-exports `Task`
and `TaskList`.

> The `ui` design system + component toolkit has its own page: [theming.md](theming.md).
> The IPC protocol has its own page: [ipc.md](ipc.md).

## Core

| File                                               | Purpose                                               | Key items                                                                                                                                                                         |
| -------------------------------------------------- | ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [src/lib.rs](../common/src/lib.rs)                 | Crate root, module list, re-exports                   | `pub use` `Task`, `TaskList`                                                                                                                                                      |
| [src/models/mod.rs](../common/src/models/mod.rs)   | Re-export aggregator                                  | re-exports `Task`, `Attachment`, `AttachmentKind`                                                                                                                                |
| [src/models/task.rs](../common/src/models/task.rs) | The task entity (serde)                               | `Task { title, completed, attachments }`, `new`, `is_completed`                                                                                                                  |
| [src/models/attachment.rs](../common/src/models/attachment.rs) | Attached-file entity (serde)              | `Attachment { name, path, kind, owned }`, `AttachmentKind { Image, File }`, `is_image`                                                                                            |
| [src/tasklist.rs](../common/src/tasklist.rs)       | In-memory task collection + mutations                 | `TaskList`; `add`/`add_with_attachments`/`remove`/`set_completed`/`add_attachment`/`remove_attachment`/`clear_completed`/`clear`/`total`/`completed`/`open`/`enumerated`         |
| [src/attachments.rs](../common/src/attachments.rs) | Attachment filesystem helpers                         | `files_dir`, `is_image`, `copy_into_files`, `attachment_from_path(copy)`, `capture_screenshot` (macOS), `open_path`                                                              |
| [src/storage.rs](../common/src/storage.rs)         | On-disk persistence (JSON)                            | `Config`, `ListEntry`, `TaskListFile`; `list_all`/`load`/`save`/`rename`/`delete`/`import_from`/`export_to`/`load_config`/`save_config`; `home_dir`/`app_data_dir`/`lists_dir`/`config_path` |
| [src/hotkey.rs](../common/src/hotkey.rs)           | Serializable hotkey binding (shared via IPC + config) | `HotkeySpec { alt, ctrl, shift, meta, code }`; `default_mini_toggle`, `has_strong_modifier`, `label`, `is_modifier_code`                                                          |

Storage location: `~/.taskscape/` — `lists/*.json`, copied attachments under
`files/`, plus `config.json`. Image attachments (and screenshots) are always
copied into `files/`; other files link to their original path unless copied.

## IPC — `src/ipc/` → see [ipc.md](ipc.md)

| File                                         | Purpose                                                        |
| -------------------------------------------- | -------------------------------------------------------------- |
| [ipc/mod.rs](../common/src/ipc/mod.rs)       | `IpcMessage`, `IpcInbound`, `socket_path`, encode/read framing |
| [ipc/client.rs](../common/src/ipc/client.rs) | Main-app client: `send`, `subscription` (auto-reconnect)       |
| [ipc/server.rs](../common/src/ipc/server.rs) | Tray server: `send`, `subscription` (one client)               |

## UI design system — `src/ui/` → see [theming.md](theming.md)

The "Concrete & Bronze" system: tokens, theme, motion, and the animated component
toolkit. Replaces the former `thememanager` + `widgets`.

| File                                                               | Purpose                                                                                  |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- |
| [ui/mod.rs](../common/src/ui/mod.rs)                               | Aggregator; re-exports theme items + `components::*`                                      |
| [ui/theme.rs](../common/src/ui/theme.rs)                           | `ThemeMode` (serde); `Palette`; `palette(mode)`/`app_theme(mode)`; `color`/`with_alpha`/`mix`/`border`/`shadow` |
| [ui/tokens.rs](../common/src/ui/tokens.rs)                         | `radius` / `space` / `text` size scales, `HAIRLINE_WIDTH`                                 |
| [ui/motion.rs](../common/src/ui/motion.rs)                         | `EASING`, `QUICK`/`PRESS`/`BASE`/`SLOW`, `reduce_motion()`/`set_reduce_motion`, `progress` |
| [ui/components/interactive.rs](../common/src/ui/components/interactive.rs) | **Custom animated `Widget`**: `Interactive`, `Surface`, `Style` (hover/press fill+lift) |
| [ui/components/button.rs](../common/src/ui/components/button.rs)   | `ButtonKind { Primary, Ghost, Icon, Plain }`, `surface_style`, `t_button`                |
| [ui/components/icon_button.rs](../common/src/ui/components/icon_button.rs) | `t_icon_button` / `t_icon_button_ghost`                                          |
| [ui/components/input.rs](../common/src/ui/components/input.rs)     | `text_input_style`, `t_input_box`                                                        |
| [ui/components/dropdown.rs](../common/src/ui/components/dropdown.rs) | `pick_list_style`, `t_dropdown`                                                         |
| [ui/components/checkbox.rs](../common/src/ui/components/checkbox.rs) | `t_checkbox` (animated box → bronze fill + check)                                       |
| [ui/components/toggle.rs](../common/src/ui/components/toggle.rs)   | `t_toggle` (rounded-rect, not a pill)                                                     |
| [ui/components/chip.rs](../common/src/ui/components/chip.rs)       | `t_small_chip`, `t_attachment_chip`                                                      |
| [ui/components/typography.rs](../common/src/ui/components/typography.rs) | `t_heading` / `t_display` / `t_body` / `t_caption`                                 |
| [ui/components/editable_title.rs](../common/src/ui/components/editable_title.rs) | `t_editable_title`; `TITLE_INPUT_ID`                                       |
| [ui/components/metric.rs](../common/src/ui/components/metric.rs)   | `t_metric` (flat value+label)                                                            |
| [ui/components/icon.rs](../common/src/ui/components/icon.rs)       | `Icon` enum + `icon(symbol, size, color)` (Material Symbols Sharp)                       |
| [ui/components/containers.rs](../common/src/ui/components/containers.rs) | `shell`/`glass_shell`/`surface`/`raised`/`bar`/`divider`/`sidebar`/`modal_*` styles |

## Utils — `src/utils/`

| File                                           | Purpose                                                               |
| ---------------------------------------------- | --------------------------------------------------------------------- |
| [utils/mod.rs](../common/src/utils/mod.rs)     | Module declarations                                                   |
| [utils/fonts.rs](../common/src/utils/fonts.rs) | Embedded TTFs + `Font` builders: `montserrat_*`, `raleway_*`, `icon_font`; `REGISTERED_FONT_BYTES` |
