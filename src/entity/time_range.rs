use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct DateTimeRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>
}
#[derive(PartialEq, Debug)]
pub enum DateTimeRangeValidationError {
    OppositeEnd,
    SameEnd
}
impl DateTimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, DateTimeRangeValidationError> {
        DateTimeRange::validate_range(&start, &end)?;
        Ok(Self { start, end })
    }

    pub fn includes(&self, other: &DateTime<Utc>) -> bool {
        other == &self.start || other > &self.start && other < &self.end
    }

    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }

    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }

    pub fn set_start(&mut self, start: DateTime<Utc>) -> Result<(), DateTimeRangeValidationError> {
        DateTimeRange::validate_range(&start, &self.end)?;
        self.start = start;
        Ok(())
    }

    pub fn set_end(&mut self, end: DateTime<Utc>) -> Result<(), DateTimeRangeValidationError> {
        DateTimeRange::validate_range(&self.start, &end)?;
        self.end = end;
        Ok(())
    }

    fn validate_range(start: &DateTime<Utc>, end: &DateTime<Utc>) -> Result<(), DateTimeRangeValidationError> {
        if start == end {
            return Err(DateTimeRangeValidationError::SameEnd);
        }
        if start > end {
            return Err(DateTimeRangeValidationError::OppositeEnd);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::{DateTimeRange, DateTimeRangeValidationError};
    use rstest::rstest;

    fn create_time(hour: u32, minute: u32) -> DateTime<Utc> {
        Utc.ymd(2022, 1, 1).and_hms(hour, minute, 00)
    }

    #[rstest(start, end, expected,
        case(create_time(10, 00), create_time(11, 0), None),
        case(create_time(10, 00), create_time(10, 0), Some(DateTimeRangeValidationError::SameEnd)),
        case(create_time(11, 00), create_time(10, 0), Some(DateTimeRangeValidationError::OppositeEnd)),
    )]
    fn opposite_end_not_allowed(start: DateTime<Utc>, end: DateTime<Utc>, expected: Option<DateTimeRangeValidationError>) {
        let maybe_todo = DateTimeRange::new(start, end);

        assert_eq!(maybe_todo.err(), expected);
    }

    #[rstest(new_start, expected,
        case(create_time(10, 00), None),
        case(create_time(11, 00), None),
        case(create_time(12, 00), Some(DateTimeRangeValidationError::SameEnd)),
        case(create_time(13, 00), Some(DateTimeRangeValidationError::OppositeEnd)),
    )]
    fn opposite_end_not_allowed_when_setting_new_start(new_start: DateTime<Utc>, expected: Option<DateTimeRangeValidationError>) {
        let mut todo = DateTimeRange::new(
            create_time(11, 00),
            create_time(12, 00)
        ).unwrap();

        assert_eq!(todo.set_start(new_start).err(), expected);
    }

    #[rstest(new_end, expected,
        case(create_time(10, 00), Some(DateTimeRangeValidationError::OppositeEnd)),
        case(create_time(11, 00), Some(DateTimeRangeValidationError::SameEnd)),
        case(create_time(12, 00), None),
        case(create_time(13, 00), None),
    )]
    fn opposite_end_not_allowed_when_setting_new_end(new_end: DateTime<Utc>, expected: Option<DateTimeRangeValidationError>) {
        let mut todo = DateTimeRange::new(
            create_time(11, 00),
            create_time(12, 00)
        ).unwrap();

        assert_eq!(todo.set_end(new_end).err(), expected);
    }

    #[rstest(time, expected,
        case(create_time(10, 00), false),
        case(create_time(11, 00), true),
        case(create_time(12, 00), true),
        case(create_time(13, 00), false),
        case(create_time(14, 00), false),
    )]
    fn can_check_if_the_range_includes_time(time: DateTime<Utc>, expected: bool) {
        let base_time_range = DateTimeRange::new(
            create_time(11, 00),
            create_time(13, 00),
        ).unwrap();

        assert_eq!(base_time_range.includes(&time), expected);
    }
}
