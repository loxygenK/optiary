use crate::entity::{TimeRange, DoneStatusList, Task};

pub struct Todo {
    task: Task,
    range: TimeRange,
    status: DoneStatusList
}

impl Todo {
    pub fn new(task: Task, range: TimeRange, status: DoneStatusList) -> Result<Self, ()> {
        Ok(Todo { task, range, status })
    }
}
