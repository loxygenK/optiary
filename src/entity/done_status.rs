use chrono::{Utc, DateTime};

use crate::{entity::DateTimeRange, types::Id};

#[derive(Clone, Debug)]
pub struct DoneStatus {
    id: Id,
    applicable_time: DateTime<Utc>,
    done: bool
}
impl DoneStatus {
    pub fn new(id: Id, applicable_time: DateTime<Utc>, done: bool) -> Self {
        Self {
            id,
            applicable_time,
            done
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn applicable_time(&self) -> &DateTime<Utc> {
        &self.applicable_time
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn undone(&self) -> bool {
        !self.done()
    }

    pub fn mark_as_done(&mut self) {
        self.done = true
    }

    pub fn mark_as_undone(&mut self) {
        self.done = false
    }
}

#[derive(Clone, Debug)]
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

    pub fn get_from_range(&self, range: &DateTimeRange) -> Vec<&DoneStatus> {
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
    use crate::types::Id;
    use super::{DoneStatus, DoneStatusList, DateTimeRange};

    use chrono::{Utc, DateTime, TimeZone};
    use rstest::rstest;

    fn create_time(hour: u32, minute: u32) -> DateTime<Utc> {
        Utc.ymd(2022, 1, 1).and_hms(hour, minute, 00)
    }

    #[rstest]
    fn can_mark_as_done_or_undone() {
        let mut done_status = DoneStatus::new(Id::new("done-status"), create_time(12, 00), false);

        done_status.mark_as_done();
        assert_eq!(done_status.done(), true);
        assert_eq!(done_status.undone(), false);

        done_status.mark_as_undone();
        assert_eq!(done_status.done(), false);
        assert_eq!(done_status.undone(), true);
    }

    fn todo_status_list_from_done_count(done: u32, undone: u32) -> DoneStatusList {
        let dones = (0..done)
            .map(|_| DoneStatus::new(Id::new("done_status-id-1"), create_time(12, 30), true));
        let undones = (0..undone)
            .map(|_| DoneStatus::new(Id::new("done_status-id-2"), create_time(12, 30), false));

        let done_status: Vec<DoneStatus> = dones.chain(undones).collect();
        DoneStatusList::new(done_status)
    }

    fn todo_status_list_from_time(time: &[(u32, u32)]) -> DoneStatusList {
        DoneStatusList::new(
            time.iter()
                .map(|&(h, m)| DoneStatus::new(Id::new(format!("done_status-id-{}-{}", h, m)), create_time(h, m), false))
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
    fn can_get_todo_status_by_the_time(start: (u32, u32), end: (u32, u32), expected_count: u32) {
        let todo_status_list = todo_status_list_from_time(&[(10, 00), (11, 00), (12, 00)]);
        let range = DateTimeRange::new(
            create_time(start.0, start.1),
            create_time(end.0, end.1)
        ).unwrap();
        let todo_status_len: u32 = todo_status_list.get_from_range(&range).len().try_into().unwrap();

        assert_eq!(todo_status_len, expected_count);
    }

    #[rstest(done, undone, expected_complete,
        case(5, 0, true),
        case(4, 1, false),
        case(1, 4, false),
        case(0, 0, false)
    )]
    fn marked_as_complete_when_all_todo_status_is_done(done: u32, undone: u32, expected_complete: bool) {
        let todo_status_list = todo_status_list_from_done_count(done, undone);

        assert_eq!(todo_status_list.complete(), expected_complete);
    }

    #[rstest(done, undone,
        case(5, 0),
        case(4, 1),
        case(1, 4),
        case(0, 0)
    )]
    fn can_count_todos_marked_as_done(done: u32, undone: u32) {
        let todo_status_list = todo_status_list_from_done_count(done, undone);
        let dones: u32 = todo_status_list.dones().try_into().unwrap();

        assert_eq!(dones, done);
    }

    #[rstest(done, undone, expected_max_dones,
        case(5, 0, 5),
        case(0, 3, 3),
        case(3, 3, 6),
        case(0, 0, 0)
    )]
    fn can_count_maximum_dones(done: u32, undone: u32, expected_max_dones: u32) {
        let todo_status_list = todo_status_list_from_done_count(done, undone);
        let max_dones: u32 = todo_status_list.max_dones().try_into().unwrap();

        assert_eq!(max_dones, expected_max_dones);
    }
}
