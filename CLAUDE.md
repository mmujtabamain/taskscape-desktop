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
  a process can keep running with zero windows. The **tray** relies on this —
  it runs windowless until the mini window opens. The **main app**, by contrast,
  quits when its window closes (`WindowCloseRequested` → `iced::exit()`); the
  tray relaunches it on demand.
- **App = state struct + `Message` enum + `update`/`view`.** Each binary has its
  own `Taskscape` / `TrayApp` state and `Message` enum in its `app/mod.rs`.
- **IPC asymmetry:** the main app is the source of truth and sends its full list
  on connect (`Hello`); the tray mirrors it. Both sides guard against echo loops
  with an `applying_remote` flag. Protocol: [common/src/ipc/](common/src/ipc/).
- **Persistence:** lists + config are JSON under `~/.taskscape/` (`lists/*.json`,
  `config.json`; see [common/src/storage.rs](common/src/storage.rs)). Copied task
  attachments live in `~/.taskscape/files/` (see
  [common/src/attachments.rs](common/src/attachments.rs)).

## Conventions

- Rust **edition 2024**.
- **Minimal comments** — names and structure carry meaning; comment only
  non-obvious intent/constraints. Match the existing `//!` module-doc style.
- Shared UI is built from the `common::widgets` toolkit (`t_*` helpers) and
  styled exclusively through `common::thememanager` factories — don't hardcode
  colors in the binaries.
- macOS-only native code (objc2 AppKit/QuartzCore) lives in the tray crate and
  is `#[cfg(target_os = "macos")]`-gated with non-macOS stubs.

## Design Context

Strategic design intent lives in [PRODUCT.md](PRODUCT.md); the visual system is
captured in [DESIGN.md](DESIGN.md). Read them before any UI work.

- **Register:** `product` — design serves the workflow (fast capture, calm list).
- **Users:** macOS power users; keyboard-first, menu-bar-resident, native feel.
- **Principles:** (1) capture beats organize, (2) calm by subtraction, (3) warmth
  without noise, (4) native craft, (5) legible in both themes (WCAG AA).
- **Avoid:** corporate SaaS-dashboard density, sterile gray minimalism, clutter,
  gamification.

## Known gotchas

- The main app's entry point is `app::run()` at the bottom of
  [main_src/src/app/mod.rs](main_src/src/app/mod.rs) (called from
  [main_src/src/main.rs](main_src/src/main.rs)) — there is no separate `entry`
  module.
- The build artifact tree (`target/`) and `dist/` are large; ignore them when
  searching. Source is only under `common/src`, `main_src/src`, `tray_src/src`.
