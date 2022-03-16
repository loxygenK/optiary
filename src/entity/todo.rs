use crate::entity::TimeRange;
use crate::entity::DoneStatusList;
use crate::entity::Time;
use crate::entity::Task;

pub struct Todo {
    task: Task,
    range: TimeRange,
    status: DoneStatusList
}

impl Todo {
    pub fn new(task: Task, range: TimeRange, status: DoneStatusList) -> Result<Todo, ()> {
        Ok(Todo { task, range, status })
    }
}
