//! Client side of the link, owned by the **main app**.
//!
//! Connects to the tray service's Unix socket, retrying with a short backoff so
//! it links whenever the tray service is (or becomes) available — regardless of
//! startup order — and auto-relinks after the tray restarts. Inbound messages
//! surface as [`IpcInbound`] events; outbound messages (main → tray) are written
//! through a process-global handle the `update` loop pokes via [`send`].

use super::{IpcInbound, IpcMessage, read_messages, socket_path, write_message};
use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::sync::Mutex;
use std::time::Duration;

/// The write half of the current connection, if linked.
static OUTBOUND: Mutex<Option<UnixStream>> = Mutex::new(None);

/// How long to wait between connection attempts while the tray service is down.
const RECONNECT_DELAY: Duration = Duration::from_millis(1000);

/// Sends a message to the tray service, if linked. Best-effort: a failed write
/// (peer gone) clears the handle until the next connection is established.
pub fn send(message: &IpcMessage) {
    let mut guard = OUTBOUND.lock().unwrap();
    if let Some(stream) = guard.as_mut() {
        if write_message(stream, message).is_err() {
            *guard = None;
        }
    }
}

/// Subscription that connects (with retry) and streams link events into iced.
pub fn subscription() -> Subscription<IpcInbound> {
    Subscription::run(client_stream)
}

fn client_stream() -> impl iced::futures::Stream<Item = IpcInbound> {
    let (tx, rx) = mpsc::channel::<IpcInbound>(64);

    if let Err(error) = std::thread::Builder::new()
        .name("taskscape-ipc-client".into())
        .spawn(move || run_client(tx))
    {
        eprintln!("ipc(client): failed to spawn connect thread: {error}");
    }

    rx
}

fn run_client(mut tx: mpsc::Sender<IpcInbound>) {
    let path = socket_path();

    loop {
        let stream = match UnixStream::connect(&path) {
            Ok(stream) => stream,
            Err(_) => {
                // Tray service not up yet — wait and retry.
                std::thread::sleep(RECONNECT_DELAY);
                continue;
            }
        };

        // Publish the write half so `send` can reach the tray service.
        match stream.try_clone() {
            Ok(write_half) => *OUTBOUND.lock().unwrap() = Some(write_half),
            Err(error) => {
                eprintln!("ipc(client): could not clone stream: {error}");
                std::thread::sleep(RECONNECT_DELAY);
                continue;
            }
        }

        if iced::futures::executor::block_on(tx.send(IpcInbound::Connected)).is_err() {
            return; // subscription dropped
        }

        // Read until the tray service goes away.
        read_messages(BufReader::new(stream), |message| {
            let _ = iced::futures::executor::block_on(tx.send(IpcInbound::Message(message)));
        });

        // Link lost: clear the handle, notify, then loop to reconnect.
        *OUTBOUND.lock().unwrap() = None;
        if iced::futures::executor::block_on(tx.send(IpcInbound::Disconnected)).is_err() {
            return;
        }

        std::thread::sleep(RECONNECT_DELAY);
    }
}
