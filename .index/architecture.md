# Architecture

High-level model of how Taskscape runs. File-level detail is in
[common.md](common.md), [main.md](main.md), [tray.md](tray.md).

## Two processes, one app

```
        ┌─────────────────────────┐         Unix socket          ┌──────────────────────────┐
        │  taskscape (main_src)   │   newline-delimited JSON      │ taskscape-tray (tray_src)│
        │  • main task window     │ <───────────────────────────> │ • menu-bar icon          │
        │  • IPC CLIENT           │      common/src/ipc/          │ • global hotkey          │
        │  • source of truth      │                               │ • mini popup window      │
        │  • native menu, undo,   │                               │ • IPC SERVER             │
        │    import/export        │                               │ • mirrors the list       │
        └───────────┬─────────────┘                               └────────────┬─────────────┘
                    │                                                           │
                    │ both link to ───────────────► common (taskscape-common) ◄┘
                    │   models · ipc · storage · ui · utils · hotkey
                    │
            on startup: app::ensure_tray_running() spawns the tray binary
            if the socket isn't already being served (main_src/.../launch.rs)
```

The user launches only `taskscape`. It boots the tray for them. Both binaries
depend on `common`; nothing in `common` is executable.

## Iced daemon structure

Both binaries are Iced **`daemon`**s (not `application`s), so windows are opened
programmatically and the process outlives any single window. Each follows the
same shape, defined in its own `app/mod.rs`:

- a **state struct** — `Taskscape` (main) / `TrayApp` (tray)
- a **`Message` enum** — every event the app can react to
- **`boot`** → initial state + startup tasks (load fonts, open window)
- **`update(msg)`** → mutate state, return follow-up `iced::Task`s
- **`view_window(id)`** → render the current state to widgets
- **`subscription`** → external event streams (keyboard, window, IPC, tray, hotkey, native menu)
- **`run()`** → wires it all into `iced::daemon(...).run()`

The main app opens its window immediately. The tray opens a **hidden bootstrap
window** to install the tray icon + hotkey, then runs windowless until the user
opens the mini window.

## Source of truth & sync (anti-echo)

The **main app owns the list.** On IPC connect it sends `Hello { tasks }`; the
tray adopts it wholesale. Afterwards, single mutations (add/remove/toggle) flow
both ways as `IpcMessage`s.

To avoid infinite echo, each side sets an **`applying_remote`** flag while
applying a mutation that arrived over IPC, and `broadcast()` is a no-op while
that flag is set. Bulk changes (clear-all, list switch, import) skip per-item
messages and instead re-send a full `Hello` (`resync_tray` on the main side).

While the main app is **offline**, the tray has no one to broadcast to, so it
persists its own mini-window edits to disk (`persist_local`) and loads the
last-open list at `boot` — that's how those edits survive a closed main window.
The two never write at once: `persist_local` no-ops whenever the link is up, so
the main app stays the sole writer while it's running.

See [ipc.md](ipc.md) for the message set and framing.

## Data flow (one mutation)

```
user action ─► Message ─► update() ─┬─► state mutation        (actions.rs / update.rs)
                                     ├─► persist to disk       (storage.rs)
                                     └─► broadcast over IPC     (sync.rs)  ─► other process mirrors it
view() reads state (+ queries.rs) ─► widgets (common::ui)
```

Undo/redo (main only): a snapshot of the task list is pushed before each
mutation; undo/redo swap between the undo and redo stacks
([main_src/src/app/snapshot.rs](../main_src/src/app/snapshot.rs)).

## Persistence

`~/.taskscape/`

- `lists/<name>.json` — one file per task list (tasks carry their attachments)
- `files/` — copies of attached files + screenshots (see
  [common/src/attachments.rs](../common/src/attachments.rs)); non-image files may
  instead link to their original path
- `config.json` — theme, last-open list, reopen-last toggle, confirm-clear
  toggle, hotkey spec + enabled flag

Lists + config in [common/src/storage.rs](../common/src/storage.rs). Writes are
write-through: every list/task mutation saves immediately.

## macOS-native bits (tray only)

The tray uses objc2 AppKit/QuartzCore for the mini window's native look:
rounded corners (CALayer clip), no drop shadow, and anchoring under the menu-bar
icon using the display's backing scale (NSScreen). All gated behind
`#[cfg(target_os = "macos")]` with no-op stubs elsewhere. See
[tray.md](tray.md) → `tray.rs` / `mini.rs`.

## Packaging

`make-app.sh` builds release binaries and assembles `dist/Taskscape.app`:

```
Taskscape.app/Contents/
├── MacOS/taskscape                                   # main binary
├── Info.plist                                        # main_src/macos/Info.plist
├── Resources/assets/…                                # fonts (also embedded)
└── Library/LoginItems/Taskscape Tray.app/            # nested tray bundle
    ├── MacOS/taskscape-tray
    └── Info.plist                                    # tray_src/macos/Info.plist (LSUIElement)
```

The tray bundle is nested so the main app can find it at a stable relative path
and the user installs one app.
