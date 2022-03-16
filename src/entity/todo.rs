use crate::entity::DoneStatusList;
use crate::entity::Time;
use crate::entity::Task;

pub struct Todo {
    task: Task,
    from: Time,
    to: Time,
    status: DoneStatusList
}

#[derive(Debug)]
pub enum TodoValidationError {
    TimeRangeSwapped
}
impl Todo {
    pub fn new(task: Task, from: Time, to: Time, status: DoneStatusList) -> Result<Todo, TodoValidationError> {
        if from.is_after_than(&to) {
            return Err(TodoValidationError::TimeRangeSwapped)
        }
        Ok(Todo { task, from, to, status })
    }

    pub fn includes(&self, time: &Time) -> bool {
        time.is_in_range(&self.from, &self.to)
    }
}

#[cfg(test)]
mod tests {
    use super::{Task, Todo, Time, DoneStatusList};
    use rstest::rstest;

    #[rstest(time, expected_includes,
        case(Time::new(10, 00).unwrap(), false),
        case(Time::new(11, 00).unwrap(), true),
        case(Time::new(12, 00).unwrap(), true),
        case(Time::new(13, 00).unwrap(), true),
        case(Time::new(14, 00).unwrap(), false),
    )]
    fn can_check_if_the_time_includes_its_range(time: Time, expected_includes: bool) {
        let todo = Todo::new(
            Task::new("Hoge").unwrap(),
            Time::new(11, 00).unwrap(),
            Time::new(13, 00).unwrap(),
            DoneStatusList::new(vec![])
        )
            .unwrap();

        assert_eq!(todo.includes(&time), expected_includes);
    }
}
