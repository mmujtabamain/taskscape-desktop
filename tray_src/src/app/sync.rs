//! IPC glue for the tray service (the IPC *server*).
//!
//! Adopts the main app's source-of-truth list on `Hello`, applies the mutations
//! it receives without echoing them back, and mirrors its own mini-window
//! mutations to the main app — or, when the main app is offline, persists them
//! to disk itself.

use crate::app::{AppTask, TrayApp};
use common::ipc::{self, IpcInbound, IpcMessage};

impl TrayApp {
    /// Mirrors a local mini-window mutation to the main app, unless we are
    /// currently applying a mutation that came from it (which would loop).
    pub(crate) fn broadcast(&self, message: IpcMessage) {
        if self.applying_remote || !self.ipc_connected {
            return;
        }
        ipc::server::send(&message);
    }

    /// Persists the current list straight to disk — the fallback for
    /// [`broadcast`](Self::broadcast) when the main app is offline, so
    /// mini-window edits survive with the main window closed. While the main app
    /// is linked it owns the on-disk list (and saves there itself), so this
    /// no-ops to avoid two writers racing on the same file.
    pub(crate) fn persist_local(&mut self) {
        if self.ipc_connected {
            return;
        }
        let Some(name) = self.current_list.clone() else {
            return;
        };
        if let Err(error) = common::storage::save(&name, self.tasks.tasks()) {
            self.status_message = error;
        }
    }

    /// Handles an inbound link event. Returns any follow-up task.
    pub(crate) fn handle_ipc(&mut self, event: IpcInbound) -> AppTask {
        match event {
            IpcInbound::Connected => {
                self.ipc_connected = true;
                self.status_message = String::from("Linked to Taskscape.");
                // If the user clicked the title while the main app was closed, we
                // launched it; now that it's linked, ask it to come forward.
                if self.pending_show_main {
                    self.pending_show_main = false;
                    ipc::server::send(&IpcMessage::ShowMain);
                }
            }
            IpcInbound::Disconnected => {
                self.ipc_connected = false;
                self.status_message = String::from("Main app closed — running standalone.");
            }
            IpcInbound::Message(message) => self.apply_remote(message),
        }
        iced::Task::none()
    }

    /// Applies a mutation received from the main app. The `applying_remote` guard
    /// stops the change from being broadcast straight back.
    fn apply_remote(&mut self, message: IpcMessage) {
        self.applying_remote = true;

        match message {
            // Source-of-truth bulk sync: adopt the main app's list + name.
            IpcMessage::Hello { list_name, tasks } => {
                self.current_list = list_name;
                self.tasks.replace(tasks);
                self.status_message = String::from("Synced from Taskscape.");
            }
            IpcMessage::AddTask { title } => {
                self.tasks.add(title);
            }
            IpcMessage::RemoveTask { index } => {
                self.tasks.remove(index);
            }
            IpcMessage::ToggleTaskCompleted { index, completed } => {
                self.tasks.set_completed(index, completed);
            }
            // The main app changed the mini-window hotkey in settings: re-register
            // it live. Runs on the UI thread (update → handle_ipc), which is where
            // the hotkey manager lives.
            IpcMessage::SetHotkey { hotkey, enabled } => {
                self.status_message = match crate::app::hotkey::apply(hotkey, enabled) {
                    Ok(()) if enabled => String::from("Hotkey updated."),
                    Ok(()) => String::from("Hotkey disabled."),
                    Err(error) => format!("Hotkey: {error}"),
                };
            }
            // The tray never receives these (they are tray→main only).
            IpcMessage::ShowMain | IpcMessage::Shutdown | IpcMessage::Bye => {}
        }

        self.applying_remote = false;
    }
}
