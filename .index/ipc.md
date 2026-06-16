# IPC protocol (main ↔ tray)

The cross-crate contract that links the two processes. Code lives in
[common/src/ipc/](../common/src/ipc/). This is the single source of truth for
both binaries, so changing a message here means updating both `sync.rs` files.

## Transport

- **Unix domain socket**, one client at a time. Path from `socket_path()`
  (`XDG_RUNTIME_DIR` if set, else temp dir) — file name `taskscape.sock`.
- **Framing:** newline-delimited JSON. One `IpcMessage` per line
  (`encode` / `write_message` / `read_messages` in [ipc/mod.rs](../common/src/ipc/mod.rs)).
- **Roles:** tray = **server** ([ipc/server.rs](../common/src/ipc/server.rs)),
  main = **client** with auto-reconnect ([ipc/client.rs](../common/src/ipc/client.rs)).
- Each side exposes a `subscription()` that surfaces link events into the app's
  `Message::IpcEvent`.

## Messages — `IpcMessage`

| Variant                                    | Direction   | Meaning                                                                                             |
| ------------------------------------------ | ----------- | --------------------------------------------------------------------------------------------------- |
| `Hello { list_name, tasks }`               | main → tray | Full-state sync; tray adopts this list wholesale (sent on connect, list switch, and after bulk ops) |
| `AddTask { title }`                        | both        | Append a task                                                                                       |
| `RemoveTask { index }`                     | both        | Remove task at index                                                                                |
| `ToggleTaskCompleted { index, completed }` | both        | Set a task's completion                                                                             |
| `SetHotkey { hotkey, enabled }`            | main → tray | Live hotkey rebind; tray re-registers immediately                                                   |
| `ShowMain`                                 | tray → main | Bring the main window forward (mini "show app")                                                     |
| `Shutdown`                                 | tray → main | Tray is quitting                                                                                    |
| `Bye`                                      | both        | Graceful disconnect                                                                                 |

## Link events — `IpcInbound`

Surfaced by both `subscription()`s into the app `update`:

- `Connected` — socket linked
- `Message(IpcMessage)` — a decoded message arrived
- `Disconnected` — link dropped (client retries)

## Echo prevention

Single mutations are mirrored both ways, so each side guards with an
`applying_remote` flag: while applying a message received over IPC, its
`broadcast()` is suppressed so the change isn't bounced back. Bulk changes don't
emit per-item messages — the main app re-sends a full `Hello` instead
(`resync_tray`). See `sync.rs` in [main.md](main.md) and [tray.md](tray.md).

## Where each side handles it

- Main app: [main_src/src/app/sync.rs](../main_src/src/app/sync.rs)
  (`broadcast`, `resync_tray`, `send_hotkey_config`, `handle_ipc`, `apply_remote`)
- Tray: [tray_src/src/app/sync.rs](../tray_src/src/app/sync.rs)
  (`broadcast`, `handle_ipc`, `apply_remote`)
