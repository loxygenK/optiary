use crate::entity::Time;

pub struct DoneStatus {
    pub id: String,
    pub applicable_time: Time,
    pub done: bool
}
impl DoneStatus {
    /* TODO: Write stuffs */
}

pub struct DoneStatusList {
    pub statuses: Vec<DoneStatus>
}
impl DoneStatusList {
    /* TODO: Write stuffs */
}

#[cfg(test)]
mod tests {
    use super::{DoneStatus, DoneStatusList};

    fn can_get_todo_status_by_the_time() {

    }

    fn marked_as_complete_when_all_todo_status_is_done() {

    }

    fn can_count_todos_marked_as_done() {

    }

    fn can_count_maximum_dones() {

    }
}
