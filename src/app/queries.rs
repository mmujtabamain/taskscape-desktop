use crate::app::Taskscape;
use crate::models::Task;

impl Taskscape {
    pub(crate) fn visible_tasks(&self) -> Vec<(usize, &Task)> {
        self.tasks.iter().enumerate().collect()
    }

    pub(crate) fn open_count(&self) -> usize {
        self.tasks.iter().filter(|task| !task.is_completed()).count()
    }

    pub(crate) fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|task| task.is_completed()).count()
    }

    pub(crate) fn total_count(&self) -> usize {
        self.tasks.len()
    }
}
