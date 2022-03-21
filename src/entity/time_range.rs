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
    pub fn new(start: Time, end: Time) -> Result<Self, TimeRangeValidationError> {
        TimeRange::validate_range(&start, &end)?;
        Ok(Self { start, end })
    }

    pub fn includes(&self, other: &Time) -> bool {
        other == &self.start || other > &self.start && other < &self.end
    }

    pub fn start(&self) -> &Time {
        &self.start
    }

    pub fn end(&self) -> &Time {
        &self.end
    }

    pub fn set_start(&mut self, start: Time) -> Result<(), TimeRangeValidationError> {
        TimeRange::validate_range(&start, &self.end)?;
        self.start = start;
        Ok(())
    }

    pub fn set_end(&mut self, end: Time) -> Result<(), TimeRangeValidationError> {
        TimeRange::validate_range(&self.start, &end)?;
        self.end = end;
        Ok(())
    }

    fn validate_range(start: &Time, end: &Time) -> Result<(), TimeRangeValidationError> {
        if start == end {
            return Err(TimeRangeValidationError::SameEnd);
        }
        if start > end {
            return Err(TimeRangeValidationError::OppositeEnd);
        }

        Ok(())
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

    #[rstest(new_start, expected,
        case(Time::new(10, 00).unwrap(), None),
        case(Time::new(11, 00).unwrap(), None),
        case(Time::new(12, 00).unwrap(), Some(TimeRangeValidationError::SameEnd)),
        case(Time::new(13, 00).unwrap(), Some(TimeRangeValidationError::OppositeEnd)),
    )]
    fn opposite_end_not_allowed_when_setting_new_start(new_start: Time, expected: Option<TimeRangeValidationError>) {
        let mut todo = TimeRange::new(
            Time::new(11, 00).unwrap(),
            Time::new(12, 00).unwrap()
        ).unwrap();

        assert_eq!(todo.set_start(new_start).err(), expected);
    }

    #[rstest(new_end, expected,
        case(Time::new(10, 00).unwrap(), Some(TimeRangeValidationError::OppositeEnd)),
        case(Time::new(11, 00).unwrap(), Some(TimeRangeValidationError::SameEnd)),
        case(Time::new(12, 00).unwrap(), None),
        case(Time::new(13, 00).unwrap(), None),
    )]
    fn opposite_end_not_allowed_when_setting_new_end(new_end: Time, expected: Option<TimeRangeValidationError>) {
        let mut todo = TimeRange::new(
            Time::new(11, 00).unwrap(),
            Time::new(12, 00).unwrap()
        ).unwrap();

        assert_eq!(todo.set_end(new_end).err(), expected);
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
