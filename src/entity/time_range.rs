use crate::entity::Time;

pub struct TimeRange {
    start: Time,
    end: Time
}
#[derive(PartialEq, Debug)]
pub enum TimeRangeValidationError {
    OppositeEnd,
    SameEnd
}
impl TimeRange {
    pub fn new(start: Time, end: Time) -> Result<TimeRange, TimeRangeValidationError> {
        if start.is_same_to(&end) {
            return Err(TimeRangeValidationError::SameEnd);
        }
        if start.is_after_than(&end) {
            return Err(TimeRangeValidationError::OppositeEnd);
        }

        Ok(TimeRange { start, end })
    }

    pub fn includes(&self, other: &Time) -> bool {
        other.is_same_to(&self.start) || (other.is_after_than(&self.start) && other.is_before_than(&self.end))
    }
}

#[cfg(test)]
mod tests {
    use super::{Time, TimeRange, TimeRangeValidationError};
    use rstest::rstest;

    #[rstest(start, end, expected,
        case(Time::new(10, 00).unwrap(), Time::new(11, 0).unwrap(), None),
        case(Time::new(10, 00).unwrap(), Time::new(10, 0).unwrap(), Some(TimeRangeValidationError::SameEnd)),
        case(Time::new(11, 00).unwrap(), Time::new(10, 0).unwrap(), Some(TimeRangeValidationError::OppositeEnd)),
    )]
    fn opposite_end_not_allowed(start: Time, end: Time, expected: Option<TimeRangeValidationError>) {
        let maybe_todo = TimeRange::new(start, end);

        assert_eq!(maybe_todo.err(), expected);
    }

    #[rstest(time, expected,
        case(Time::new(10, 00).unwrap(), false),
        case(Time::new(11, 00).unwrap(), true),
        case(Time::new(12, 00).unwrap(), true),
        case(Time::new(13, 00).unwrap(), false),
        case(Time::new(14, 00).unwrap(), false),
    )]
    fn can_check_if_the_range_includes_time(time: Time, expected: bool) {
        let base_time_range = TimeRange::new(
            Time::new(11, 00).unwrap(),
            Time::new(13, 00).unwrap(),
        ).unwrap();

        assert_eq!(base_time_range.includes(&time), expected);
    }
}
