use crate::app::snapshot::AppSnapshot;
use crate::app::{AppTask, AttachTarget, Message, Taskscape};
use common::ipc::IpcMessage;
use common::models::Attachment;
use common::storage;
use iced::Task;
use std::path::PathBuf;

impl Taskscape {
    pub(crate) fn snapshot(&self) -> AppSnapshot {
        AppSnapshot {
            tasks: self.tasks.to_vec(),
        }
    }

    pub(crate) fn restore_snapshot(&mut self, snapshot: AppSnapshot) {
        self.tasks.replace(snapshot.tasks);
    }

    pub(crate) fn push_history(&mut self) {
        self.undo_stack.push(self.snapshot());
        self.redo_stack.clear();
    }

    /// Writes the current list's tasks to its JSON file. No-op when no list is
    /// open. Called after every task mutation (write-through autosave) and keeps
    /// the cached sidebar task-count for this list in sync.
    pub(crate) fn persist_current(&mut self) {
        let Some(name) = self.current_list.clone() else {
            return;
        };
        match storage::save(&name, self.tasks.tasks()) {
            Ok(_) => {
                let count = self.tasks.total();
                if let Some(entry) = self.lists.iter_mut().find(|e| e.name == name) {
                    entry.task_count = count;
                }
            }
            Err(error) => self.status_message = error,
        }
    }

    // --- Task mutations (each autosaves the open list) ---

    /// Adds the task currently in the composer input, with any staged
    /// attachments. Returns the added title + attachments so the caller can
    /// mirror them to the tray service over IPC, or `None` if the input was
    /// empty (nothing added).
    pub(crate) fn add_task(&mut self) -> Option<(String, Vec<Attachment>)> {
        let title = self.title_input.trim().to_owned();
        if title.is_empty() {
            return None;
        }

        self.push_history();
        let attachments = std::mem::take(&mut self.staged_attachments);
        let added = self
            .tasks
            .add_with_attachments(title, attachments.clone())?;

        self.title_input.clear();
        self.status_message = String::from("Task added.");
        self.persist_current();
        Some((added, attachments))
    }

    /// Appends a task with an explicit title + attachments (used when applying
    /// the tray service's add over IPC).
    pub(crate) fn add_task_with(&mut self, title: String, attachments: Vec<Attachment>) {
        if title.trim().is_empty() {
            return;
        }
        self.push_history();
        self.tasks.add_with_attachments(title, attachments);
        self.status_message = String::from("Task added.");
        self.persist_current();
    }

    /// Attaches a file (chosen from the picker, already turned into an
    /// [`Attachment`]) or screenshot to the given target. For a task it pushes
    /// history, persists, and mirrors over IPC; for the composer it just stages.
    pub(crate) fn attach_to_target(&mut self, target: AttachTarget, attachment: Attachment) {
        match target {
            AttachTarget::Composer => {
                self.staged_attachments.push(attachment);
                self.status_message = String::from("Attachment staged.");
            }
            AttachTarget::Task(index) => {
                self.push_history();
                if self.tasks.add_attachment(index, attachment.clone()) {
                    self.status_message = String::from("Attachment added.");
                    self.persist_current();
                    self.broadcast(IpcMessage::AddAttachment { index, attachment });
                } else {
                    self.undo_stack.pop();
                }
            }
        }
    }

    /// Removes the attachment at `attachment` from the task at `task`,
    /// persisting and mirroring over IPC.
    pub(crate) fn remove_task_attachment(&mut self, task: usize, attachment: usize) {
        self.push_history();
        if self.tasks.remove_attachment(task, attachment) {
            self.status_message = String::from("Attachment removed.");
            self.persist_current();
            self.broadcast(IpcMessage::RemoveAttachment {
                index: task,
                attachment_index: attachment,
            });
        } else {
            self.undo_stack.pop();
        }
    }

    /// Opens the native file picker to attach a file to `target`. `copy`
    /// (captured from the Option/Alt modifier at press time) forces a copy into
    /// Taskscape rather than linking to the original.
    pub(crate) fn launch_file_attach_dialog(target: AttachTarget, copy: bool) -> AppTask {
        Task::perform(
            async {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Attach File")
                    .pick_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            move |path| Message::FileChosen { target, copy, path },
        )
    }

    pub(crate) fn remove_task(&mut self, index: usize) {
        self.push_history();
        if self.tasks.remove(index) {
            self.status_message = String::from("Task removed.");
            self.persist_current();
        } else {
            // Nothing removed: drop the no-op history entry we just pushed.
            self.undo_stack.pop();
        }
    }

    pub(crate) fn toggle_task_completed(&mut self, index: usize, completed: bool) {
        self.push_history();
        if self.tasks.set_completed(index, completed) {
            self.status_message = if completed {
                String::from("Task marked complete.")
            } else {
                String::from("Task marked open.")
            };
            self.persist_current();
        } else {
            self.undo_stack.pop();
        }
    }

    pub(crate) fn clear_completed(&mut self) {
        self.push_history();
        self.tasks.clear_completed();
        self.status_message = String::from("Completed tasks removed.");
        self.persist_current();
    }

    pub(crate) fn clear_all(&mut self) {
        self.push_history();
        self.tasks.clear();
        self.status_message = String::from("All tasks removed.");
        self.persist_current();
    }

    pub(crate) fn undo(&mut self) {
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.snapshot());
            self.restore_snapshot(previous);
            self.status_message = String::from("Undid the last change.");
            self.persist_current();
        }
    }

    pub(crate) fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.snapshot());
            self.restore_snapshot(next);
            self.status_message = String::from("Restored the undone change.");
            self.persist_current();
        }
    }

    // --- List management ---

    /// Re-scans the lists directory into the sidebar cache.
    pub(crate) fn refresh_lists(&mut self) {
        self.lists = storage::list_all();
    }

    /// Records the open list in the persisted config (for reopen-on-launch).
    fn remember_open(&self) {
        let mut config = storage::load_config();
        config.last_open = self.current_list.clone();
        storage::save_config(&config);
    }

    /// Leaves the settings page (and stops any in-progress hotkey capture).
    pub(crate) fn leave_settings(&mut self) {
        self.show_settings = false;
        self.recording_hotkey = false;
    }

    /// Writes the user-facing settings to the config, preserving fields managed
    /// elsewhere (e.g. `last_open`) via load-modify-save.
    pub(crate) fn persist_settings(&self) {
        let mut config = storage::load_config();
        config.theme = Some(self.theme_mode);
        config.reopen_last_list = self.reopen_last_list;
        config.confirm_clear_all = self.confirm_clear_all;
        config.hotkey_enabled = self.hotkey_enabled;
        config.hotkey = Some(self.hotkey.clone());
        storage::save_config(&config);
    }

    /// Loads a list's tasks into state without touching status/config. Used at
    /// startup to restore the last-used list.
    pub(crate) fn open_list_quiet(&mut self, name: &str) {
        if let Ok(file) = storage::load(name) {
            self.tasks.replace(file.tasks);
            self.current_list = Some(file.name);
            self.undo_stack.clear();
            self.redo_stack.clear();
        }
    }

    /// Opens an existing list by display name (user-initiated).
    pub(crate) fn open_list(&mut self, name: &str) {
        match storage::load(name) {
            Ok(file) => {
                self.tasks.replace(file.tasks);
                self.current_list = Some(file.name.clone());
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.status_message = format!("Opened \"{}\".", file.name);
                self.remember_open();
            }
            Err(error) => self.status_message = error,
        }
    }

    /// Creates a new empty list from `new_list_name` and opens it.
    pub(crate) fn create_list(&mut self) {
        let name = self.new_list_name.trim().to_owned();
        if name.is_empty() {
            self.status_message = String::from("Enter a name for the new list.");
            return;
        }
        if self.lists.iter().any(|e| e.name == name) {
            self.status_message = format!("A list named \"{name}\" already exists.");
            return;
        }

        match storage::save(&name, &[]) {
            Ok(_) => {
                self.tasks.replace(Vec::new());
                self.current_list = Some(name.clone());
                self.new_list_name.clear();
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.refresh_lists();
                self.status_message = format!("Created \"{name}\".");
                self.remember_open();
            }
            Err(error) => self.status_message = error,
        }
    }

    /// Deletes a list by display name. If it was the open list, clears the view
    /// back to the empty-state prompt.
    pub(crate) fn delete_list(&mut self, name: &str) {
        let Some(entry) = self.lists.iter().find(|e| e.name == name).cloned() else {
            return;
        };
        match storage::delete(&entry.path) {
            Ok(()) => {
                if self.current_list.as_deref() == Some(name) {
                    self.current_list = None;
                    self.tasks.replace(Vec::new());
                    self.undo_stack.clear();
                    self.redo_stack.clear();
                    self.remember_open();
                }
                if self.renaming.as_ref().map(|(n, _)| n.as_str()) == Some(name) {
                    self.renaming = None;
                }
                self.refresh_lists();
                self.status_message = format!("Deleted \"{name}\".");
            }
            Err(error) => self.status_message = error,
        }
    }

    /// Commits an in-progress rename.
    pub(crate) fn commit_rename(&mut self) {
        let Some((old_name, new_name)) = self.renaming.take() else {
            return;
        };
        match storage::rename(&old_name, &new_name) {
            Ok(entry) => {
                if self.current_list.as_deref() == Some(old_name.as_str()) {
                    self.current_list = Some(entry.name.clone());
                    self.remember_open();
                }
                self.refresh_lists();
                self.status_message = format!("Renamed to \"{}\".", entry.name);
            }
            Err(error) => {
                // Keep the rename open so the user can correct it.
                self.renaming = Some((old_name, new_name));
                self.status_message = error;
            }
        }
    }

    // --- Import / export (native JSON file dialogs) ---

    pub(crate) fn launch_import_dialog() -> AppTask {
        Task::perform(
            async {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Import List")
                    .add_filter("JSON", &["json"])
                    .pick_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            Message::ImportListResult,
        )
    }

    pub(crate) fn complete_import(&mut self, path: PathBuf) {
        match storage::import_from(&path) {
            Ok(entry) => {
                let name = entry.name.clone();
                self.refresh_lists();
                self.open_list(&name);
                self.status_message = format!("Imported \"{name}\".");
            }
            Err(error) => self.status_message = error,
        }
    }

    pub(crate) fn launch_export_dialog(&self) -> AppTask {
        let suggested = self
            .current_list
            .clone()
            .map(|n| format!("{n}.json"))
            .unwrap_or_else(|| String::from("list.json"));
        Task::perform(
            async move {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Export List")
                    .set_file_name(&suggested)
                    .add_filter("JSON", &["json"])
                    .save_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            Message::ExportListResult,
        )
    }

    pub(crate) fn complete_export(&mut self, path: PathBuf) {
        let Some(name) = self.current_list.clone() else {
            self.status_message = String::from("No list open to export.");
            return;
        };
        self.status_message = match storage::export_to(&name, &path) {
            Ok(()) => format!("Exported to {}.", path.display()),
            Err(error) => error,
        };
    }
}
