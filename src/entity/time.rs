use std::cmp::Ordering::{Less, Equal, Greater};

pub struct Time {
    hour: u8,
    minute: u8
}
#[derive(PartialEq, Debug)]
pub enum TimeValidationError {
    OutOfRange
}
impl Time {
    pub fn new(hour: u8, minute: u8) -> Result<Time, TimeValidationError> {
        if hour > 23 || minute > 59 {
            return Err(TimeValidationError::OutOfRange)
        }
        Ok(Time { hour, minute })
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn is_before_than(&self, other: &Time) -> bool {
        match self.hour.cmp(&other.hour) {
            Less => true,
            Equal => self.minute <= other.minute,
            Greater => false
        }
    }

    pub fn is_after_than(&self, other: &Time) -> bool {
        other.is_before_than(self)
    }

    pub fn is_same_to(&self, other: &Time) -> bool {
        self.hour == other.hour && self.minute == other.minute
    }

    pub fn is_in_range(&self, start: &Time, end: &Time) -> bool {
        self.is_after_than(start) && self.is_before_than(end)
    }
}

#[cfg(test)]
mod tests {
    use super::{Time, TimeValidationError};
    use rstest::rstest;

    #[rstest(hour, minute, expected,
        case(0, 0, None),
        case(12, 0, None),
        case(23, 59, None),
        case(24, 00, Some(TimeValidationError::OutOfRange)),
        case(0, 60, Some(TimeValidationError::OutOfRange))
    )]
    fn out_of_range_not_allowed(hour: u8, minute: u8, expected: Option<TimeValidationError>) {
        let maybe_time = Time::new(hour, minute);

        assert_eq!(maybe_time.err(), expected);
    }

    #[rstest(comparing, before, same, after,
        case(Time::new(11, 00).unwrap(), false, false, true),
        case(Time::new(12, 00).unwrap(), false, false, true),
        case(Time::new(12, 30).unwrap(), true, true, true),
        case(Time::new(12, 59).unwrap(), true, false, false),
        case(Time::new(13, 00).unwrap(), true, false, false)
    )]
    fn can_compare_time(comparing: Time, before: bool, same: bool, after: bool) {
        let base_time = Time { hour: 12, minute: 30 };

        assert_eq!(base_time.is_before_than(&comparing), before);
        assert_eq!(base_time.is_same_to(&comparing), same);
        assert_eq!(base_time.is_after_than(&comparing), after);
    }

    #[rstest(start, end, expected,
        case(Time::new(11, 00).unwrap(), Time::new(13, 00).unwrap(), true),
        case(Time::new(12, 00).unwrap(), Time::new(12, 30).unwrap(), true),
        case(Time::new(12, 30).unwrap(), Time::new(13, 00).unwrap(), true),
        case(Time::new(12, 30).unwrap(), Time::new(12, 30).unwrap(), true),
        case(Time::new(11, 00).unwrap(), Time::new(12, 00).unwrap(), false),
        case(Time::new(12, 59).unwrap(), Time::new(13, 00).unwrap(), false)
    )]
    fn can_check_if_the_time_is_between_two_times(start: Time, end: Time, expected: bool) {
        let base_time = Time { hour: 12, minute: 30 };

        assert_eq!(base_time.is_in_range(&start, &end), expected);
    }
}
