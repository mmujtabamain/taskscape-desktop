use crate::app::Taskscape;
use common::models::Task;

impl Taskscape {
    pub(crate) fn visible_tasks(&self) -> Vec<(usize, &Task)> {
        self.tasks.enumerated()
    }

    pub(crate) fn open_count(&self) -> usize {
        self.tasks.open()
    }

    pub(crate) fn completed_count(&self) -> usize {
        self.tasks.completed()
    }

    pub(crate) fn total_count(&self) -> usize {
        self.tasks.total()
    }
}
