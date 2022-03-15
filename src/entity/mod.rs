mod time;

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

pub struct DoneStatusList {
    pub statuses: Vec<DoneStatus>
}

pub struct DoneStatus {
    pub id: String,
    pub applicable_time: Time,
    pub done: bool
}
