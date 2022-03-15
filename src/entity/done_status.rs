use crate::entity::Time;

pub struct DoneStatus {
    applicable_time: Time,
    done: bool
}
impl DoneStatus {
    fn new(applicable_time: &Time, done: bool) -> DoneStatus {
        DoneStatus {
            applicable_time: Time::new(applicable_time.hour(), applicable_time.minute()),
            done
        }
    }

    fn applicable_time(&self) -> Time {
        Time::new(self.applicable_time.hour(), self.applicable_time.minute())
    }

    fn done(&self) -> bool {
        self.done
    }

    fn undone(&self) -> bool {
        !self.done()
    }
}

pub struct DoneStatusList {
    statuses: Vec<DoneStatus>
}
impl DoneStatusList {
    pub fn new(statuses: Vec<DoneStatus>) -> DoneStatusList {
        DoneStatusList { statuses }
    }

    pub fn get_all(&self) -> &[DoneStatus] {
        &self.statuses
    }

    pub fn get_from_range(&self, start: &Time, end: &Time) -> Vec<&DoneStatus> {
        self.statuses
            .iter()
            .filter(|s| s.applicable_time().is_in_range(start, end))
            .collect::<Vec<&DoneStatus>>()
    }

    pub fn dones(&self) -> usize {
        self.statuses
            .iter()
            .filter(|s| s.done())
            .count()
    }

    pub fn undones(&self) -> usize {
        self.statuses
            .iter()
            .filter(|s| s.undone())
            .count()
    }

    pub fn max_dones(&self) -> usize {
        self.statuses.len()
    }

    pub fn complete(&self) -> bool {
        if self.statuses.len() == 0 {
            return false
        }

        self.dones() == self.max_dones()
    }
}

#[cfg(test)]
mod tests {
    use super::{Time, DoneStatus, DoneStatusList};
    use rstest::rstest;

    fn todo_status_list_from_done_count(done: usize, undone: usize) -> DoneStatusList {
        let dones = (0..done)
            .map(|_| DoneStatus::new(&Time::new(12, 30), true));
        let undones = (0..undone)
            .map(|_| DoneStatus::new(&Time::new(12, 30), false));

        let done_status: Vec<DoneStatus> = dones.chain(undones).collect();
        DoneStatusList::new(done_status)
    }

    fn todo_status_list_from_time(time: &[(u8, u8)]) -> DoneStatusList {
        DoneStatusList::new(
            time.iter()
                .map(|(h, m)| DoneStatus::new(&Time::new(*h, *m), false))
                .collect()
        )
    }

    #[rstest(start, end, expected_count,
        case(Time::new(9, 00), Time::new(13,00), 3),
        case(Time::new(10, 00), Time::new(12,00), 3),
        case(Time::new(10, 00), Time::new(11,00), 2),
        case(Time::new(11, 00), Time::new(12,00), 2),
        case(Time::new(10, 00), Time::new(10,00), 1),
        case(Time::new(9, 00), Time::new(10,00), 1),
        case(Time::new(13, 00), Time::new(14,00), 0),
    )]
    fn can_get_todo_status_by_the_time(start: Time, end: Time, expected_count: usize) {
        let todo_status_list = todo_status_list_from_time(&[(10, 00), (11, 00), (12, 00)]);

        assert_eq!(todo_status_list.get_from_range(&start, &end).len(), expected_count);
    }

    #[rstest(done, undone, expected_complete,
        case(5, 0, true),
        case(4, 1, false),
        case(1, 4, false),
        case(0, 0, false)
    )]
    fn marked_as_complete_when_all_todo_status_is_done(done: usize, undone: usize, expected_complete: bool) {
        let todo_status_list = todo_status_list_from_done_count(done, undone);

        assert_eq!(todo_status_list.complete(), expected_complete);
    }

    #[rstest(done, undone,
        case(5, 0),
        case(4, 1),
        case(1, 4),
        case(0, 0)
    )]
    fn can_count_todos_marked_as_done(done: usize, undone: usize) {
        let todo_status_list = todo_status_list_from_done_count(done, undone);

        assert_eq!(todo_status_list.dones(), done);
    }

    #[rstest(done, undone, expected_max_dones,
        case(5, 0, 5),
        case(0, 3, 3),
        case(3, 3, 6),
        case(0, 0, 0)
    )]
    fn can_count_maximum_dones(done: usize, undone: usize, expected_max_dones: usize) {
        let todo_status_list = todo_status_list_from_done_count(done, undone);

        assert_eq!(todo_status_list.max_dones(), expected_max_dones);
    }
}
