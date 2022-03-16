use crate::entity::{TimeRange, DoneStatusList, Task};

pub struct Todo {
    task: Task,
    range: TimeRange,
    status: DoneStatusList
}
impl Todo {
    pub fn new(task: Task, range: TimeRange, status: DoneStatusList) -> Self {
        Self { task, range, status }
    }

    pub fn task(&self) -> &Task {
        &self.task
    }

    pub fn range(&self) -> &TimeRange {
        &self.range
    }

    pub fn status(&self) -> &DoneStatusList {
        &self.status
    }
}
