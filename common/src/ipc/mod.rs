//! Local IPC linking the main-window app and the background tray service.
//!
//! The two apps are independent processes. They "link" over a Unix domain
//! socket: the **tray service is the server** (binds + listens) and the **main
//! app is the client** (connects, with retry). While linked, a small set of
//! task mutations sync between them; either side keeps working standalone when
//! the other is absent.
//!
//! The wire format is newline-delimited JSON: one [`IpcMessage`] per line.

pub mod client;
pub mod server;

use crate::models::Task;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

/// File name of the shared Unix socket.
pub const SOCKET_NAME: &str = "taskscape.sock";

/// Path to the shared socket. Prefers `$XDG_RUNTIME_DIR` (Linux) and otherwise
/// falls back to the OS temp dir, so both processes derive the same location.
pub fn socket_path() -> PathBuf {
    let dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    dir.join(SOCKET_NAME)
}

/// One message on the wire. Only the agreed-upon sync surface is represented:
/// the bulk source-of-truth handshake plus add / remove / toggle mutations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    /// Sent by the **main app** right after connecting: its full task list, which
    /// the tray service adopts wholesale (main app is the source of truth).
    Hello { tasks: Vec<Task> },
    /// A task was added with this title (appended to the end of the list).
    AddTask { title: String },
    /// The task at `index` was removed.
    RemoveTask { index: usize },
    /// The task at `index` had its completed flag set to `completed`.
    ToggleTaskCompleted { index: usize, completed: bool },
    /// Graceful "I'm closing / unlinking" notice. A plain socket EOF is treated
    /// the same way, so this is best-effort.
    Bye,
}

/// Events surfaced to an app's `update` loop from its IPC subscription. Shared
/// by both the [`client`] and [`server`] transports.
#[derive(Debug, Clone)]
pub enum IpcInbound {
    /// The link came up (client connected / server accepted a client).
    Connected,
    /// A peer message arrived.
    Message(IpcMessage),
    /// The link went down (peer closed, EOF, or a transport error).
    Disconnected,
}

/// Serializes a message as a single `\n`-terminated JSON line.
fn encode(message: &IpcMessage) -> Option<String> {
    serde_json::to_string(message).ok().map(|mut s| {
        s.push('\n');
        s
    })
}

/// Writes one framed message to `writer`, flushing it. Returns an error if the
/// peer has gone away.
fn write_message<W: Write>(writer: &mut W, message: &IpcMessage) -> io::Result<()> {
    let line = encode(message)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "failed to encode message"))?;
    writer.write_all(line.as_bytes())?;
    writer.flush()
}

/// Reads framed messages off `reader` and forwards each decoded [`IpcMessage`]
/// to `on_message`, returning when the peer closes (EOF) or errors. Malformed
/// lines are skipped rather than killing the link.
fn read_messages<R: BufRead>(reader: R, mut on_message: impl FnMut(IpcMessage)) {
    for line in reader.lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<IpcMessage>(&line) {
            Ok(message) => on_message(message),
            Err(error) => eprintln!("ipc: dropping malformed message: {error}"),
        }
    }
}
