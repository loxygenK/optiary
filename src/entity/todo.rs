use crate::entity::{DateTimeRange, DoneStatusList, Task};
use crate::types::Id;

pub struct Todo {
    id: Id,
    task: Task,
    range: DateTimeRange,
    status: DoneStatusList
}
impl Todo {
    pub fn new(id: Id, task: Task, range: DateTimeRange, status: DoneStatusList) -> Self {
        Self { id, task, range, status }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn task(&self) -> &Task {
        &self.task
    }

    pub fn range(&self) -> &DateTimeRange {
        &self.range
    }

    pub fn range_mut(&mut self) -> &mut DateTimeRange {
        &mut self.range
    }

    pub fn status(&self) -> &DoneStatusList {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut DoneStatusList {
        &mut self.status
    }
}
