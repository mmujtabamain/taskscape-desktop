//! IPC glue for the main app (the source of truth and IPC *client*).
//!
//! On connect it pushes its full task list to the tray service; thereafter it
//! mirrors local add/remove/toggle mutations and applies the ones it receives
//! without echoing them back.

use crate::app::{AppTask, Taskscape};
use common::ipc::{self, IpcInbound, IpcMessage};

impl Taskscape {
    /// Mirrors a local task mutation to the tray service, unless we are currently
    /// applying a mutation that came from it (which would loop).
    pub(crate) fn broadcast(&self, message: IpcMessage) {
        if self.applying_remote || !self.ipc_connected {
            return;
        }
        ipc::client::send(&message);
    }

    /// Handles an inbound link event. Returns any follow-up task.
    pub(crate) fn handle_ipc(&mut self, event: IpcInbound) -> AppTask {
        match event {
            IpcInbound::Connected => {
                self.ipc_connected = true;
                // Main app is the source of truth: push the full list.
                ipc::client::send(&IpcMessage::Hello {
                    tasks: self.tasks.to_vec(),
                });
                self.status_message = String::from("Linked to mini service.");
            }
            IpcInbound::Disconnected => {
                self.ipc_connected = false;
                self.status_message = String::from("Mini service offline — running standalone.");
            }
            IpcInbound::Message(message) => self.apply_remote(message),
        }
        iced::Task::none()
    }

    /// Applies a mutation received from the tray service. The `applying_remote`
    /// guard stops the change from being broadcast straight back.
    fn apply_remote(&mut self, message: IpcMessage) {
        self.applying_remote = true;

        match message {
            IpcMessage::AddTask { title } => self.add_task_with_title(title),
            IpcMessage::RemoveTask { index } => self.remove_task(index),
            IpcMessage::ToggleTaskCompleted { index, completed } => {
                self.toggle_task_completed(index, completed)
            }
            // The main app is the source of truth, so it never adopts a peer's
            // `Hello`; a `Bye` is handled by the transport's disconnect.
            IpcMessage::Hello { .. } | IpcMessage::Bye => {}
        }

        self.applying_remote = false;
    }
}
