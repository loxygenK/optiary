use crate::entity::Date;
use crate::entity::{TimeRange, DoneStatusList, Task};
use crate::types::Id;

pub struct Todo {
    id: Id,
    task: Task,
    date: Date,
    range: TimeRange,
    status: DoneStatusList
}
impl Todo {
    pub fn new(id: Id, task: Task, date: Date, range: TimeRange, status: DoneStatusList) -> Self {
        Self { id, task, date, range, status }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn task(&self) -> &Task {
        &self.task
    }

    pub fn range(&self) -> &TimeRange {
        &self.range
    }

    pub fn range_mut(&mut self) -> &mut TimeRange {
        &mut self.range
    }

    pub fn status(&self) -> &DoneStatusList {
        &self.status
    }

    pub fn status_mut(&mut self) -> &mut DoneStatusList {
        &mut self.status
    }
}
