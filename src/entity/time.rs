use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    hour: usize,
    minute: usize
}
#[derive(PartialEq, Debug)]
pub enum TimeValidationError {
    OutOfRange
}
impl Time {
    pub fn new(hour: usize, minute: usize) -> Result<Self, TimeValidationError> {
        if hour > 23 || minute > 59 {
            return Err(TimeValidationError::OutOfRange)
        }
        Ok(Self { hour, minute })
    }

    pub fn hour(&self) -> usize {
        self.hour
    }

    pub fn minute(&self) -> usize {
        self.minute
    }

    pub fn diff(&self, other: &Self) -> Option<Self> {
        if self < &other {
            return None;
        }

        let self_minutes = self.hour * 60 + self.minute;
        let other_minutes = other.hour * 60 + other.minute;

        let diff = self_minutes - other_minutes;
        Some(Time::new(diff / 60, diff % 60).unwrap())
    }
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hour.cmp(&other.hour) {
            Less => Less,
            Equal => self.minute.cmp(&other.minute),
            Greater => Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::{Time, TimeValidationError};
    use rstest::rstest;

    #[rstest(hour, minute, expected,
        case(0, 0, None),
        case(12, 0, None),
        case(23, 59, None),
        case(24, 00, Some(TimeValidationError::OutOfRange)),
        case(0, 60, Some(TimeValidationError::OutOfRange))
    )]
    fn out_of_range_not_allowed(hour: usize, minute: usize, expected: Option<TimeValidationError>) {
        let maybe_time = Time::new(hour, minute);

        assert_eq!(maybe_time.err(), expected);
    }

    #[rstest(comparing, comparison,
        case(Time::new(11, 00).unwrap(), Ordering::Greater),
        case(Time::new(12, 00).unwrap(), Ordering::Greater),
        case(Time::new(12, 30).unwrap(), Ordering::Equal),
        case(Time::new(12, 59).unwrap(), Ordering::Less),
        case(Time::new(13, 00).unwrap(), Ordering::Less)
    )]
    fn can_compare_time(comparing: Time, comparison: Ordering) {
        let base_time = Time { hour: 12, minute: 30 };

        assert_eq!(base_time.cmp(&comparing), comparison);
    }

    #[rstest(other, expected,
        case(Time::new(10, 00).unwrap(), Some(Time::new(2, 0).unwrap())),
        case(Time::new(11, 30).unwrap(), Some(Time::new(0, 30).unwrap())),
        case(Time::new(12, 00).unwrap(), Some(Time::new(0, 0).unwrap())),
        case(Time::new(13, 00).unwrap(), None),
    )]
    fn can_calculate_differential(other: Time, expected: Option<Time>) {
        let base_time = Time::new(12, 00).unwrap();
        let diff = base_time.diff(&other);

        assert_eq!(diff, expected);
    }
}
