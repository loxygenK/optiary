use crate::types::Id;

#[derive(Debug, Eq, PartialEq)]
pub struct Date {
    year: u32,
    month: u32,
    date: u32
}
#[derive(Debug, PartialEq)]
pub enum DateValidationError {
    OutOfRange
}
impl Date {

    const LAST_DAY: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    pub fn new(year: u32, month: u32, date: u32) -> Result<Self, DateValidationError> {
        if !(1..=12).contains(&month) {
            return Err(DateValidationError::OutOfRange);
        }
        if date < 1 || date > Date::get_last_day_of_month(year, month) {
            return Err(DateValidationError::OutOfRange);
        }

        Ok(Date { year, month, date })
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn date(&self) -> u32 {
        self.date
    }

    fn get_last_day_of_month(year: u32, month: u32) -> u32 {
        let leap_adjust = if month == 2 && Date::is_leap_year(year) { 1 } else { 0 };
        let index: usize = usize::try_from(month).unwrap();

        Date::LAST_DAY[index - 1] + leap_adjust
    }

    fn is_leap_year(year: u32) -> bool {
        year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Date, DateValidationError};
    use rstest::rstest;

    #[rstest(year, month, day, expected,
        case(2022, 1, 1, None),
        case(2022, 1, 31, None),
        case(2022, 1, 32, Some(DateValidationError::OutOfRange)),
        case(2022, 4, 31, Some(DateValidationError::OutOfRange)),
        case(2022, 13, 1, Some(DateValidationError::OutOfRange)),
    )]
    fn out_of_range_not_allowed(year: u32, month: u32, day: u32, expected: Option<DateValidationError>) {
        let maybe_date = Date::new(year, month, day);

        assert_eq!(maybe_date.err(), expected);
    }

    #[rstest(year, month, date, expected,
        case(2022, 1, 1, None),
        case(2022, 1, 0, Some(DateValidationError::OutOfRange)),
        case(2022, 0, 1, Some(DateValidationError::OutOfRange)),
    )]
    fn date_and_month_is_1_indexed(year: u32, month: u32, date: u32, expected: Option<DateValidationError>) {
        let maybe_date = Date::new(year, month, date);

        assert_eq!(maybe_date.err(), expected);
    }

    #[rstest(year, month, date, expected,
        case(2020, 2, 29, None),
        case(2022, 2, 29, Some(DateValidationError::OutOfRange)),
        case(2024, 2, 29, None),
        case(2000, 2, 29, None),
        case(2100, 2, 29, Some(DateValidationError::OutOfRange)),
    )]
    fn february_date_limit_extended_at_leap_year(year: u32, month: u32, date: u32, expected: Option<DateValidationError>) {
        let maybe_date = Date::new(year, month, date);

        assert_eq!(maybe_date.err(), expected);
    }
}
