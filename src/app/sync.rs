//! IPC glue tying the shared `Taskscape` state to the link with the peer
//! process. Handles role-correct sending, the source-of-truth `Hello`
//! handshake, and applying a peer's mutations without echoing them back.

use crate::app::application::AppRole;
use crate::app::Taskscape;
use crate::ipc::{IpcInbound, IpcMessage};

impl Taskscape {
    /// Sends a message to the peer over whichever side of the link this role
    /// owns (the main app is the client, the tray service the server).
    pub(crate) fn ipc_send(&self, message: &IpcMessage) {
        if !self.ipc_connected {
            return;
        }
        match self.role {
            AppRole::Main => crate::ipc::client::send(message),
            AppRole::Tray => crate::ipc::server::send(message),
        }
    }

    /// Mirrors a local task mutation to the peer, unless we are currently
    /// applying a mutation that *came from* the peer (which would loop).
    pub(crate) fn broadcast(&self, message: IpcMessage) {
        if self.applying_remote {
            return;
        }
        self.ipc_send(&message);
    }

    /// Handles an inbound link event. Returns any follow-up task (e.g. the main
    /// app's `Hello` on connect).
    pub(crate) fn handle_ipc(&mut self, event: IpcInbound) -> crate::app::AppTask {
        match event {
            IpcInbound::Connected => {
                self.ipc_connected = true;
                match self.role {
                    AppRole::Main => {
                        // Main app is the source of truth: push the full list.
                        self.ipc_send(&IpcMessage::Hello {
                            tasks: self.tasks.clone(),
                        });
                        self.status_message = String::from("Linked to mini service.");
                    }
                    AppRole::Tray => {
                        self.status_message = String::from("Linked to Taskscape.");
                    }
                }
            }
            IpcInbound::Disconnected => {
                self.ipc_connected = false;
                self.status_message = match self.role {
                    AppRole::Main => String::from("Mini service offline — running standalone."),
                    AppRole::Tray => String::from("Main app closed — running standalone."),
                };
            }
            IpcInbound::Message(message) => self.apply_remote(message),
        }
        iced::Task::none()
    }

    /// Applies a mutation received from the peer. The `applying_remote` guard
    /// stops the change from being broadcast straight back.
    fn apply_remote(&mut self, message: IpcMessage) {
        self.applying_remote = true;

        match message {
            // Source-of-truth bulk sync: adopt the peer's list wholesale. (Only
            // the tray service receives this; the main app never sends to itself.)
            IpcMessage::Hello { tasks } => {
                self.tasks = tasks;
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.status_message = String::from("Synced from Taskscape.");
            }
            IpcMessage::AddTask { title } => self.add_task_with_title(title),
            IpcMessage::RemoveTask { index } => self.remove_task(index),
            IpcMessage::ToggleTaskCompleted { index, completed } => {
                self.toggle_task_completed(index, completed)
            }
            IpcMessage::Bye => {
                // Treated like a disconnect; the transport also emits
                // `Disconnected` on EOF, so nothing extra to do here.
            }
        }

        self.applying_remote = false;
    }
}
