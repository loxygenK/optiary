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

    fn 時間を元に正しくDoneStatusを取れる() {

    }

    fn コンプリート状態を正しく判断できる() {

    }

    fn 完了した数を正しく計算できる() {

    }

    fn 最大の完了数を正しく計算できる() {

    }
}
