use crate::entity::DoneStatusList;
use crate::entity::Time;
use crate::entity::Task;

pub struct Todo {
    task: Task,
    from: Time,
    to: Time,
    status: DoneStatusList
}

#[derive(PartialEq, Debug)]
pub enum TodoValidationError {
    TimeRangeOppositeEnd,
    TimeRangeSameEnd,
}
impl Todo {
    pub fn new(task: Task, from: Time, to: Time, status: DoneStatusList) -> Result<Todo, TodoValidationError> {
        if from.is_same_to(&to) {
            return Err(TodoValidationError::TimeRangeSameEnd);
        }
        if from.is_after_than(&to) {
            return Err(TodoValidationError::TimeRangeOppositeEnd);
        }
        Ok(Todo { task, from, to, status })
    }

    pub fn includes(&self, time: &Time) -> bool {
        time.is_in_range(&self.from, &self.to)
    }
}

#[cfg(test)]
mod tests {
    use super::{Task, Todo, Time, DoneStatusList, TodoValidationError};
    use rstest::rstest;

    #[rstest(start, end, expected,
        case(Time::new(10, 00).unwrap(), Time::new(11, 0).unwrap(), None),
        case(Time::new(10, 00).unwrap(), Time::new(10, 0).unwrap(), Some(TodoValidationError::TimeRangeSameEnd)),
        case(Time::new(11, 00).unwrap(), Time::new(10, 0).unwrap(), Some(TodoValidationError::TimeRangeOppositeEnd)),
    )]
    fn opposite_end_not_allowed(start: Time, end: Time, expected: Option<TodoValidationError>) {
        let maybe_todo = Todo::new(Task::new("Hoge").unwrap(), start, end, DoneStatusList::new(vec![]));

        assert_eq!(maybe_todo.err(), expected);
    }

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
