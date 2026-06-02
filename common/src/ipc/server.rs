//! Server side of the link, owned by the **tray service**.
//!
//! Binds the shared Unix socket and accepts one main-app client at a time,
//! forwarding inbound messages into the iced runtime as [`IpcInbound`] events.
//! Outbound messages (tray → main) are written through a process-global handle
//! the `update` loop pokes via [`send`]. When the client disconnects we emit
//! [`IpcInbound::Disconnected`] and keep listening, so the tray keeps working
//! standalone and auto-relinks when the main app returns.

use super::{IpcInbound, IpcMessage, read_messages, socket_path, write_message};
use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use std::io::BufReader;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Mutex;

/// The write half of the currently-connected client, if any. The accept loop
/// installs it on connect and clears it on disconnect; [`send`] writes to it.
static OUTBOUND: Mutex<Option<UnixStream>> = Mutex::new(None);

/// Sends a message to the connected main app, if one is linked. Best-effort: a
/// write failure (peer gone) clears the handle so the next [`send`] no-ops until
/// a new client connects.
pub fn send(message: &IpcMessage) {
    let mut guard = OUTBOUND.lock().unwrap();
    if let Some(stream) = guard.as_mut() {
        if write_message(stream, message).is_err() {
            *guard = None;
        }
    }
}

/// Subscription that binds the socket and streams link events into iced.
pub fn subscription() -> Subscription<IpcInbound> {
    Subscription::run(server_stream)
}

fn server_stream() -> impl iced::futures::Stream<Item = IpcInbound> {
    let (tx, rx) = mpsc::channel::<IpcInbound>(64);

    if let Err(error) = std::thread::Builder::new()
        .name("taskscape-ipc-server".into())
        .spawn(move || run_server(tx))
    {
        eprintln!("ipc(server): failed to spawn accept thread: {error}");
    }

    rx
}

fn run_server(mut tx: mpsc::Sender<IpcInbound>) {
    let path = socket_path();

    // A leftover socket file from a previous run would make `bind` fail with
    // "address in use"; clear it first. (Safe: only the tray service binds here.)
    let _ = std::fs::remove_file(&path);

    let listener = match UnixListener::bind(&path) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("ipc(server): failed to bind {}: {error}", path.display());
            return;
        }
    };

    // Accept one client at a time. When it disconnects, loop back and wait for
    // the next one — the tray service is long-lived.
    for incoming in listener.incoming() {
        let stream = match incoming {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("ipc(server): accept error: {error}");
                continue;
            }
        };

        // Publish the write half so `send` can reach this client.
        match stream.try_clone() {
            Ok(write_half) => *OUTBOUND.lock().unwrap() = Some(write_half),
            Err(error) => {
                eprintln!("ipc(server): could not clone client stream: {error}");
                continue;
            }
        }

        if iced::futures::executor::block_on(tx.send(IpcInbound::Connected)).is_err() {
            break; // subscription dropped
        }

        // Read until the client goes away.
        read_messages(BufReader::new(stream), |message| {
            let _ = iced::futures::executor::block_on(tx.send(IpcInbound::Message(message)));
        });

        // Client gone: drop the outbound handle and notify the app.
        *OUTBOUND.lock().unwrap() = None;
        if iced::futures::executor::block_on(tx.send(IpcInbound::Disconnected)).is_err() {
            break;
        }
    }
}
