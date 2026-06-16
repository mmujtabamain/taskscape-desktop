# Taskscape — Code Index

A **navigation map** of the repository. These files name _what lives where_ and
_how the pieces relate_ — not how they're implemented. Use them to jump straight
to the right file instead of grepping the tree.

> Read the actual source for implementation detail. This index intentionally
> omits it so it stays small and stays correct.

## Start here

| If you want to…                             | Open                               |
| ------------------------------------------- | ---------------------------------- |
| Understand the big picture / runtime model  | [architecture.md](architecture.md) |
| Find the file to change for a given task    | [where-to-fix.md](where-to-fix.md) |
| Browse the shared library, file by file     | [common.md](common.md)             |
| Browse the main window crate, file by file  | [main.md](main.md)                 |
| Browse the tray service crate, file by file | [tray.md](tray.md)                 |
| Understand the main↔tray IPC protocol       | [ipc.md](ipc.md)                   |
| Understand theming + the widget toolkit     | [theming.md](theming.md)           |
| Look up a term                              | [glossary.md](glossary.md)         |

## Repository shape

```
taskscape-desktop/
├── Cargo.toml            # workspace: members = common, main_src, tray_src
├── Cargo.lock
├── run-dev.sh            # dev build + run (main auto-spawns tray)
├── make-app.sh           # release build + assemble dist/ .app bundles
├── CLAUDE.md             # project guide for Claude Code
├── TODO.md
├── .index/               # ← you are here
├── common/               # taskscape-common  (shared lib)   → common.md
│   └── src/{ipc,models,thememanager,utils,widgets}/, storage.rs, tasklist.rs, hotkey.rs
├── main_src/             # taskscape         (main window)   → main.md
│   ├── src/app/{view/,update,actions,queries,snapshot,sync,launch,native_menu}
│   └── macos/Info.plist
├── tray_src/             # taskscape-tray    (menu-bar svc)  → tray.md
│   ├── src/app/{mini,tray,hotkey,sync,update,launch}
│   └── macos/Info.plist  # LSUIElement = true (no Dock icon)
├── assets/fonts/         # Inter + Poppins TTFs (embedded into binaries)
├── target/               # build output — ignore when searching
└── dist/                 # packaged Taskscape.app — ignore when searching
```

## The three crates at a glance

- **common** — non-executable library shared by both binaries: task model, the
  IPC protocol + transport, the theme manager, the `t_*` widget toolkit, fonts,
  and on-disk storage.
- **main_src** (`taskscape`) — the full task window. Source of truth for the
  list. IPC **client**. Owns the native menu, undo/redo, file import/export, and
  settings.
- **tray_src** (`taskscape-tray`) — background menu-bar service. IPC **server**.
  Owns the tray icon, the global hotkey, and the compact mini window.

See [architecture.md](architecture.md) for how they fit together at runtime.

## Maintaining this index

When you add/rename/remove a source file or change a cross-crate contract (the
IPC message set, the storage layout, the widget API), update the matching index
file. Keep entries to a line or two — purpose and relationships only, no
implementation. If an entry and the code disagree, the code wins; fix the entry.
