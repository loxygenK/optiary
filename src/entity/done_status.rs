use crate::entity::{TimeRange, Time};

pub struct DoneStatus {
    applicable_time: Time,
    done: bool
}
impl DoneStatus {
    fn new(applicable_time: Time, done: bool) -> Self {
        Self {
            applicable_time,
            done
        }
    }

    fn applicable_time(&self) -> &Time {
        &self.applicable_time
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
    pub fn new(statuses: Vec<DoneStatus>) -> Self {
        DoneStatusList { statuses }
    }

    pub fn get_all(&self) -> &[DoneStatus] {
        &self.statuses
    }

    pub fn get_from_range(&self, range: &TimeRange) -> Vec<&DoneStatus> {
        self.statuses
            .iter()
            .filter(|&s| range.includes(&s.applicable_time))
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
        if self.statuses.is_empty() {
            return false
        }

        self.dones() == self.max_dones()
    }
}

#[cfg(test)]
mod tests {
    use super::{Time, TimeRange, DoneStatus, DoneStatusList};
    use rstest::rstest;

    fn todo_status_list_from_done_count(done: usize, undone: usize) -> DoneStatusList {
        let dones = (0..done)
            .map(|_| DoneStatus::new(Time::new(12, 30).unwrap(), true));
        let undones = (0..undone)
            .map(|_| DoneStatus::new(Time::new(12, 30).unwrap(), false));

        let done_status: Vec<DoneStatus> = dones.chain(undones).collect();
        DoneStatusList::new(done_status)
    }

    fn todo_status_list_from_time(time: &[(u8, u8)]) -> DoneStatusList {
        DoneStatusList::new(
            time.iter()
                .map(|&(h, m)| DoneStatus::new(Time::new(h, m).unwrap(), false))
                .collect()
        )
    }

    #[rstest(start, end, expected_count,
        case((9, 00), (13,00), 3),
        case((10, 00), (12,00), 2),
        case((10, 00), (11,00), 1),
        case((11, 00), (12,00), 1),
        case((9, 00), (10,00), 0),
        case((13, 00), (14,00), 0),
    )]
    fn can_get_todo_status_by_the_time(start: (u8, u8), end: (u8, u8), expected_count: usize) {
        let todo_status_list = todo_status_list_from_time(&[(10, 00), (11, 00), (12, 00)]);
        let range = TimeRange::new(
            Time::new(start.0, start.1).unwrap(),
            Time::new(end.0, end.1).unwrap()
        ).unwrap();

        assert_eq!(todo_status_list.get_from_range(&range).len(), expected_count);
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
