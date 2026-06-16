# Taskscape

A macOS desktop task manager written in **Rust** with the **Iced 0.14** GUI
framework. It is a Cargo **workspace** of three crates: a main task window, a
menu-bar (tray) service with a mini popup window, and a shared library.

> **Navigation:** A full, navigation-only map of the codebase lives in
> [.index/](.index/). Start at [.index/README.md](.index/README.md). When you
> need to find or change something, check [.index/where-to-fix.md](.index/where-to-fix.md)
> first — it maps "I want to change X" to the exact file, so you rarely have to
> grep the tree.

## Crates (workspace members)

| Path                   | Crate name         | Binary           | Role                                                        |
| ---------------------- | ------------------ | ---------------- | ----------------------------------------------------------- |
| [common/](common/)     | `taskscape-common` | — (lib `common`) | Shared models, IPC, theme, widgets, storage                 |
| [main_src/](main_src/) | `taskscape`        | `taskscape`      | Main task window; IPC **client**; source of truth           |
| [tray_src/](tray_src/) | `taskscape-tray`   | `taskscape-tray` | Menu-bar icon + global hotkey + mini window; IPC **server** |

The user launches **one** app (`taskscape`). On startup it auto-spawns the tray
binary if not already running (see [main_src/src/app/launch.rs](main_src/src/app/launch.rs)).
The two processes talk over a local Unix socket (newline-delimited JSON).

## Build & run

```bash
./run-dev.sh             # debug build, run main app (auto-spawns tray)
./run-dev.sh --release   # release build, run main app
./run-dev.sh tray        # run ONLY the tray service (no main window)
./make-app.sh            # release build + assemble macOS .app bundles into dist/

cargo build [--release]                  # build the whole workspace
cargo run --bin taskscape                # main window only
cargo run --bin taskscape-tray           # tray service only
cargo check                              # fast type-check
```

`make-app.sh` produces `dist/Taskscape.app` with the tray nested at
`Contents/Library/LoginItems/Taskscape Tray.app` (LSUIElement → no Dock icon).

## Runtime model

- **Iced `daemon`** (not `application`): windows are opened programmatically and
  the process survives window close. The main window can close and reopen while
  the process stays alive for IPC.
- **App = state struct + `Message` enum + `update`/`view`.** Each binary has its
  own `Taskscape` / `TrayApp` state and `Message` enum in its `app/mod.rs`.
- **IPC asymmetry:** the main app is the source of truth and sends its full list
  on connect (`Hello`); the tray mirrors it. Both sides guard against echo loops
  with an `applying_remote` flag. Protocol: [common/src/ipc/](common/src/ipc/).
- **Persistence:** lists + config are JSON under
  `~/Library/Application Support/Taskscape/` (see [common/src/storage.rs](common/src/storage.rs)).

## Conventions

- Rust **edition 2024**.
- **Minimal comments** — names and structure carry meaning; comment only
  non-obvious intent/constraints. Match the existing `//!` module-doc style.
- Shared UI is built from the `common::widgets` toolkit (`t_*` helpers) and
  styled exclusively through `common::thememanager` factories — don't hardcode
  colors in the binaries.
- macOS-only native code (objc2 AppKit/QuartzCore) lives in the tray crate and
  is `#[cfg(target_os = "macos")]`-gated with non-macOS stubs.

## Known gotchas

- [main_src/src/app/entry.rs](main_src/src/app/entry.rs) is **orphaned dead code**:
  it is not declared in `app/mod.rs` and references a non-existent
  `crate::app::application` module. The real entry point is `app::run()` at the
  bottom of [main_src/src/app/mod.rs](main_src/src/app/mod.rs).
- The build artifact tree (`target/`) and `dist/` are large; ignore them when
  searching. Source is only under `common/src`, `main_src/src`, `tray_src/src`.
