use crate::app::snapshot::AppSnapshot;
use crate::app::{AppTask, Message, Taskscape};
use crate::models::Task as TaskItem;
use crate::utils::persistence::{load_todos_from_path, save_todos_to_path};
use iced::Task;
use std::path::PathBuf;

impl Taskscape {
    pub(crate) fn snapshot(&self) -> AppSnapshot {
        AppSnapshot {
            tasks: self.tasks.clone(),
        }
    }

    pub(crate) fn restore_snapshot(&mut self, snapshot: AppSnapshot) {
        self.tasks = snapshot.tasks;
    }

    pub(crate) fn push_history(&mut self) {
        self.undo_stack.push(self.snapshot());
        self.redo_stack.clear();
    }

    /// Adds the task currently in the composer input. Returns the added title so
    /// the caller can mirror it to the linked peer over IPC, or `None` if the
    /// input was empty (nothing added).
    pub(crate) fn add_task(&mut self) -> Option<String> {
        let title = self.title_input.trim().to_owned();

        if title.is_empty() {
            return None;
        }

        self.push_history();

        self.tasks.push(TaskItem::new(title.clone()));

        self.title_input.clear();
        self.due_date_input.clear();
        self.status_message = String::from("Task added.");
        Some(title)
    }

    /// Appends a task with an explicit title (used when applying a peer's add over
    /// IPC, where there is no composer input to read).
    pub(crate) fn add_task_with_title(&mut self, title: String) {
        if title.trim().is_empty() {
            return;
        }
        self.push_history();
        self.tasks.push(TaskItem::new(title));
        self.status_message = String::from("Task added.");
    }

    pub(crate) fn remove_task(&mut self, index: usize) {
        if index >= self.tasks.len() {
            return;
        }

        self.push_history();
        self.tasks.remove(index);
        self.status_message = String::from("Task removed.");
    }

    pub(crate) fn toggle_task_completed(&mut self, index: usize, completed: bool) {
        if index >= self.tasks.len() {
            return;
        }

        self.push_history();
        self.tasks[index].completed = completed;
        self.status_message = if completed {
            String::from("Task marked complete.")
        } else {
            String::from("Task marked open.")
        };
    }

    pub(crate) fn undo(&mut self) {
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.snapshot());
            self.restore_snapshot(previous);
            self.status_message = String::from("Undid the last change.");
        }
    }

    pub(crate) fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.snapshot());
            self.restore_snapshot(next);
            self.status_message = String::from("Restored the undone change.");
        }
    }

    pub(crate) fn new_list(&mut self, status: &'static str) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.tasks.clear();
        self.status_message = status.to_owned();
    }

    /// Opens the native Save dialog asynchronously (non-blocking).
    /// Result arrives as Message::FileSaveResult.
    pub(crate) fn launch_save_dialog(&self) -> AppTask {
        let file_name = format!("{}.csv", self.file_name);
        Task::perform(
            async move {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Save Tasks")
                    .set_file_name(&file_name)
                    .add_filter("CSV", &["csv"])
                    .save_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            Message::FileSaveResult,
        )
    }

    /// Opens the native Open dialog asynchronously (non-blocking).
    /// Result arrives as Message::FileLoadResult.
    pub(crate) fn launch_load_dialog() -> AppTask {
        Task::perform(
            async {
                let handle = rfd::AsyncFileDialog::new()
                    .set_title("Load Tasks")
                    .add_filter("CSV", &["csv"])
                    .pick_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            Message::FileLoadResult,
        )
    }

    /// Persists tasks to path after the save dialog resolves.
    pub(crate) fn complete_save(&mut self, path: PathBuf) {
        self.status_message = match save_todos_to_path(&self.tasks, &path) {
            Ok(msg) => {
                self.undo_stack.clear();
                self.redo_stack.clear();
                msg
            }
            Err(msg) => msg,
        };
    }

    /// Loads tasks from path after the open dialog resolves.
    pub(crate) fn complete_load(&mut self, path: PathBuf) {
        match load_todos_from_path(&path) {
            Ok(tasks) => {
                self.undo_stack.clear();
                self.redo_stack.clear();
                let count = tasks.len();
                self.tasks = tasks;
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    self.file_name = stem.to_owned();
                }
                self.status_message = format!("Loaded {count} tasks from {}.", path.display());
            }
            Err(msg) => self.status_message = msg,
        }
    }
}
