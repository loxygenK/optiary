use crate::entity::DoneStatusList;
use crate::entity::Time;
use crate::entity::Task;

pub struct Todo {
    task: Task,
    from: Time,
    to: Time,
    status: DoneStatusList
}
impl Todo {
    pub fn new(task: Task, from: Time, to: Time, status: DoneStatusList) -> Todo {
        Todo { task, from, to, status }
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
        case(Time::new(10, 00), false),
        case(Time::new(11, 00), true),
        case(Time::new(12, 00), true),
        case(Time::new(13, 00), true),
        case(Time::new(14, 00), false),
    )]
    fn can_check_if_the_time_includes_its_range(time: Time, expected_includes: bool) {
        let todo = Todo::new(Task::new("Hoge"), Time::new(11, 00), Time::new(13, 00), DoneStatusList::new(vec![]));

        assert_eq!(todo.includes(&time), expected_includes);
    }
}
