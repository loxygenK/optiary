mod done_status;
mod time;

pub use done_status::{DoneStatus, DoneStatusList};
pub use time::Time;

pub struct Task {
    pub id: String,
    pub name: String
}

pub struct Todo {
    pub id: String,
    pub task: Task,
    pub from: Time,
    pub to: Time,
    pub status: DoneStatusList
}
