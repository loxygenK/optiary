use std::cmp::Ordering::{Less, Equal, Greater};

pub struct Time {
    pub hour: u8,
    pub minute: u8
}
impl Time {
    fn is_before_than(&self, other: &Time) -> bool {
        match self.hour.cmp(&other.hour) {
            Less => true,
            Equal => self.minute <= other.minute,
            Greater => false
        }
    }

    fn is_after_than(&self, other: &Time) -> bool {
        other.is_before_than(self)
    }

    fn is_in_range(&self, start: &Time, end: &Time) -> bool {
        self.is_after_than(start) && self.is_before_than(end)
    }
}

#[cfg(test)]
mod tests {
    use super::Time;
    use rstest::rstest;

    fn time_of(hour: u8, minute: u8) -> Time {
        Time { hour, minute }
    }

    #[rstest(comparing, before, after,
        case(time_of(11, 00), false, true),
        case(time_of(12, 00), false, true),
        case(time_of(12, 30), true, true),
        case(time_of(12, 59), true, false),
        case(time_of(13, 00), true, false)
    )]
    fn 時間を比較できる(comparing: Time, before: bool, after: bool) {
        let base_time = Time { hour: 12, minute: 30 };

        assert_eq!(base_time.is_before_than(&comparing), before);
        assert_eq!(base_time.is_after_than(&comparing), after);
    }
}
