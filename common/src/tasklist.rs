//! The shared task collection both apps operate on.
//!
//! Encapsulates the `Vec<Task>` plus the add / remove / toggle mutations and the
//! count queries, so the main window and the tray service apply identical
//! semantics (which is what keeps their indices aligned across the IPC link).

use crate::models::{Attachment, Task};

#[derive(Debug, Clone, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read-only view of the tasks.
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Replaces the whole list (used by the tray service on the source-of-truth
    /// `Hello` sync from the main app).
    pub fn replace(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    /// Snapshot clone of the tasks (for the `Hello` handshake / undo history).
    pub fn to_vec(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    /// Appends a task with the given title. No-ops on a blank title. Returns the
    /// trimmed title that was added, or `None` if nothing was added.
    pub fn add(&mut self, title: impl Into<String>) -> Option<String> {
        let title = title.into().trim().to_owned();
        if title.is_empty() {
            return None;
        }
        self.tasks.push(Task::new(title.clone()));
        Some(title)
    }

    /// Appends a task with the given title and attachments. No-ops on a blank
    /// title. Returns the trimmed title that was added (for IPC mirroring), or
    /// `None` if nothing was added.
    pub fn add_with_attachments(
        &mut self,
        title: impl Into<String>,
        attachments: Vec<Attachment>,
    ) -> Option<String> {
        let title = title.into().trim().to_owned();
        if title.is_empty() {
            return None;
        }
        let mut task = Task::new(title.clone());
        task.attachments = attachments;
        self.tasks.push(task);
        Some(title)
    }

    /// Appends `attachment` to the task at `index`, if it exists. Returns whether
    /// anything was added.
    pub fn add_attachment(&mut self, index: usize, attachment: Attachment) -> bool {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.attachments.push(attachment);
                true
            }
            None => false,
        }
    }

    /// Removes the attachment at `attachment_index` from the task at `index`, if
    /// both exist. Returns whether anything was removed.
    pub fn remove_attachment(&mut self, index: usize, attachment_index: usize) -> bool {
        match self.tasks.get_mut(index) {
            Some(task) if attachment_index < task.attachments.len() => {
                task.attachments.remove(attachment_index);
                true
            }
            _ => false,
        }
    }

    /// Removes the task at `index`, if it exists. Returns whether anything was
    /// removed.
    pub fn remove(&mut self, index: usize) -> bool {
        if index >= self.tasks.len() {
            return false;
        }
        self.tasks.remove(index);
        true
    }

    /// Sets the completed flag of the task at `index`, if it exists. Returns
    /// whether anything changed.
    pub fn set_completed(&mut self, index: usize, completed: bool) -> bool {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.completed = completed;
                true
            }
            None => false,
        }
    }

    /// Removes every completed task.
    pub fn clear_completed(&mut self) {
        self.tasks.retain(|task| !task.is_completed());
    }

    /// Removes all tasks.
    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    /// `(index, task)` pairs in display order.
    pub fn enumerated(&self) -> Vec<(usize, &Task)> {
        self.tasks.iter().enumerate().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn total(&self) -> usize {
        self.tasks.len()
    }

    pub fn completed(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_completed()).count()
    }

    pub fn open(&self) -> usize {
        self.tasks.iter().filter(|t| !t.is_completed()).count()
    }
}
