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

    /// Pushes the current mini-window hotkey binding to the tray so it can
    /// re-register it live. No-op when not linked (the tray reads the same config
    /// on its next launch).
    pub(crate) fn send_hotkey_config(&self) {
        if !self.ipc_connected {
            return;
        }
        ipc::client::send(&IpcMessage::SetHotkey {
            hotkey: Some(self.hotkey.clone()),
            enabled: self.hotkey_enabled,
        });
    }

    /// Pushes the open list's name and tasks to the tray so the mini window
    /// follows a list switch. No-op when not linked.
    pub(crate) fn resync_tray(&self) {
        if !self.ipc_connected {
            return;
        }
        ipc::client::send(&IpcMessage::Hello {
            list_name: self.current_list.clone(),
            tasks: self.tasks.to_vec(),
        });
    }

    /// Handles an inbound link event. Returns any follow-up task.
    pub(crate) fn handle_ipc(&mut self, event: IpcInbound) -> AppTask {
        match event {
            IpcInbound::Connected => {
                self.ipc_connected = true;
                // Main app is the source of truth: push the open list + tasks.
                ipc::client::send(&IpcMessage::Hello {
                    list_name: self.current_list.clone(),
                    tasks: self.tasks.to_vec(),
                });
                self.status_message = String::from("Linked to mini service.");
            }
            IpcInbound::Disconnected => {
                self.ipc_connected = false;
                self.status_message = String::from("Mini service offline — running standalone.");
            }
            IpcInbound::Message(message) => return self.apply_remote(message),
        }
        iced::Task::none()
    }

    /// Applies a message received from the tray. The `applying_remote` guard
    /// stops mutations from being broadcast straight back.
    fn apply_remote(&mut self, message: IpcMessage) -> AppTask {
        self.applying_remote = true;
        let mut task = iced::Task::none();

        match message {
            IpcMessage::AddTask { title } => self.add_task_with_title(title),
            IpcMessage::RemoveTask { index } => self.remove_task(index),
            IpcMessage::ToggleTaskCompleted { index, completed } => {
                self.toggle_task_completed(index, completed)
            }
            // The tray asks us to come forward and open the list sidebar so the
            // user can switch lists.
            IpcMessage::ShowMain => {
                self.show_list_panel = true;
                if let Some(id) = self.window_id {
                    task = iced::Task::batch([
                        iced::window::gain_focus(id),
                        iced::window::minimize(id, false),
                    ]);
                }
            }
            // The user quit Taskscape from the tray: exit too.
            IpcMessage::Shutdown => {
                self.applying_remote = false;
                return iced::exit();
            }
            // The main app is the source of truth, so it never adopts a peer's
            // `Hello`; a `Bye` is handled by the transport's disconnect.
            // `SetHotkey` is main→tray only, so it never arrives here.
            IpcMessage::Hello { .. } | IpcMessage::Bye | IpcMessage::SetHotkey { .. } => {}
        }

        self.applying_remote = false;
        task
    }
}
