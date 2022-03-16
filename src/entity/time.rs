use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Eq, PartialEq, PartialOrd)]
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
    fn out_of_range_not_allowed(hour: u8, minute: u8, expected: Option<TimeValidationError>) {
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
}
