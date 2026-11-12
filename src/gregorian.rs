//! The Gregorian calendar date type and its conversion entry points.
//!
//! `julian::j2w` returns `i32` month/day (mirroring the Julian-bridge's own
//! bounds); narrowing to `u8` here is in range by construction (month
//! 1..=12, day 1..=31 for any JDN reachable from this crate's supported
//! year range).
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use crate::Error;
use crate::julian::{self, CalendarType};
use crate::myanmar::MyanmarDate;
use crate::weekday::Weekday;

/// Earliest Gregorian year this crate supports converting, inclusive.
///
/// The Myanmar calendar algorithm's empirical era table and watat (leap
/// year) rules are only validated against the real historical calendar,
/// which begins around 638 CE (Myanmar Era year 0). We leave a small margin
/// and start supporting conversions at year 640 CE; round-trip conversion
/// has been exhaustively verified across every era boundary within this
/// range (see `tests/roundtrip.rs` and `tests/golden.rs`).
pub const MIN_YEAR: i32 = 640;

/// Latest Gregorian year this crate supports converting, inclusive.
pub const MAX_YEAR: i32 = 9999;

/// A Gregorian (proleptic, not British-cutover) calendar date.
///
/// # Examples
///
/// ```
/// use myanmar_date::GregorianDate;
///
/// let date = GregorianDate::new(2024, 1, 1)?;
/// assert_eq!(date.year(), 2024);
/// assert_eq!(date.month(), 1);
/// assert_eq!(date.day(), 1);
/// # Ok::<(), myanmar_date::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GregorianDate {
    year: i32,
    month: u8,
    day: u8,
}

impl GregorianDate {
    /// Builds a validated Gregorian date.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if `year` is outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`], or [`Error::InvalidDate`] if `month`/`day`
    /// do not name a real calendar date (month outside 1..=12, or day
    /// outside 1..=the number of days in that month/year).
    ///
    /// # Examples
    ///
    /// ```
    /// use myanmar_date::{Error, GregorianDate};
    ///
    /// assert!(GregorianDate::new(2024, 2, 29).is_ok()); // 2024 is a leap year
    /// assert_eq!(GregorianDate::new(2023, 2, 29).unwrap_err(), Error::InvalidDate);
    /// assert_eq!(GregorianDate::new(300, 1, 1).unwrap_err(), Error::OutOfRange);
    /// ```
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, Error> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(Error::OutOfRange);
        }
        if !(1..=12).contains(&month) {
            return Err(Error::InvalidDate);
        }
        let max_day = days_in_month(year, month);
        if day < 1 || day > max_day {
            return Err(Error::InvalidDate);
        }
        Ok(Self { year, month, day })
    }

    /// The year.
    ///
    /// ```
    /// use myanmar_date::GregorianDate;
    /// assert_eq!(GregorianDate::new(2024, 1, 1)?.year(), 2024);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// The month, `1..=12`.
    ///
    /// ```
    /// use myanmar_date::GregorianDate;
    /// assert_eq!(GregorianDate::new(2024, 1, 1)?.month(), 1);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn month(&self) -> u8 {
        self.month
    }

    /// The day of the month, `1..=31`.
    ///
    /// ```
    /// use myanmar_date::GregorianDate;
    /// assert_eq!(GregorianDate::new(2024, 1, 1)?.day(), 1);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn day(&self) -> u8 {
        self.day
    }

    /// The day of the week.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, Weekday};
    /// assert_eq!(GregorianDate::new(2024, 1, 1)?.weekday(), Weekday::Mon);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn weekday(&self) -> Weekday {
        Weekday::from_jdn(self.to_jdn())
    }

    /// Converts a Myanmar date to its Gregorian equivalent. Infallible: a
    /// `MyanmarDate` is only ever constructed from an already-valid instant.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(GregorianDate::from_myanmar(&m), g);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn from_myanmar(date: &MyanmarDate) -> Self {
        Self::from_jdn(date.to_jdn())
    }

    /// Converts to a Julian Day Number using the proleptic Gregorian
    /// calendar (noon-referenced, but since this crate is date-only the
    /// result is always an exact integer).
    pub(crate) fn to_jdn(self) -> i64 {
        julian::w2j(
            self.year,
            i32::from(self.month),
            i32::from(self.day),
            CalendarType::Gregorian,
            crate::consts::SG,
        )
    }

    /// Builds a `GregorianDate` from a Julian Day Number already known to be
    /// in range (used internally by conversions that start from a validated
    /// `MyanmarDate`, so this does not need to return `Result`).
    pub(crate) fn from_jdn(jdn: i64) -> Self {
        let (year, month, day) = julian::j2w(jdn, CalendarType::Gregorian, crate::consts::SG);
        Self {
            year,
            month: month as u8,
            day: day as u8,
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0, // caller already validated month is 1..=12; unreachable in practice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_year_below_supported_range() {
        assert_eq!(
            GregorianDate::new(MIN_YEAR - 1, 1, 1).unwrap_err(),
            Error::OutOfRange
        );
    }

    #[test]
    fn rejects_year_above_supported_range() {
        assert_eq!(
            GregorianDate::new(MAX_YEAR + 1, 1, 1).unwrap_err(),
            Error::OutOfRange
        );
    }

    #[test]
    fn accepts_boundary_years() {
        assert!(GregorianDate::new(MIN_YEAR, 1, 1).is_ok());
        assert!(GregorianDate::new(MAX_YEAR, 12, 31).is_ok());
    }

    #[test]
    fn rejects_month_zero_and_thirteen() {
        assert_eq!(
            GregorianDate::new(2024, 0, 1).unwrap_err(),
            Error::InvalidDate
        );
        assert_eq!(
            GregorianDate::new(2024, 13, 1).unwrap_err(),
            Error::InvalidDate
        );
    }

    #[test]
    fn rejects_day_zero() {
        assert_eq!(
            GregorianDate::new(2024, 1, 0).unwrap_err(),
            Error::InvalidDate
        );
    }

    #[test]
    fn rejects_day_31_in_a_30_day_month() {
        assert_eq!(
            GregorianDate::new(2024, 4, 31).unwrap_err(),
            Error::InvalidDate
        );
    }

    #[test]
    fn leap_year_feb_29_is_valid_non_leap_year_feb_29_is_not() {
        assert!(GregorianDate::new(2024, 2, 29).is_ok());
        assert_eq!(
            GregorianDate::new(2023, 2, 29).unwrap_err(),
            Error::InvalidDate
        );
    }

    #[test]
    fn century_leap_year_rules() {
        assert!(GregorianDate::new(2000, 2, 29).is_ok()); // divisible by 400
        assert_eq!(
            GregorianDate::new(1900, 2, 29).unwrap_err(),
            Error::InvalidDate
        ); // divisible by 100, not 400
    }

    #[test]
    fn getters_return_constructed_values() {
        let d = GregorianDate::new(2024, 3, 15).unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 3);
        assert_eq!(d.day(), 15);
    }

    #[test]
    fn weekday_matches_known_anchor() {
        let d = GregorianDate::new(2024, 1, 1).unwrap();
        assert_eq!(d.weekday(), Weekday::Mon);
    }

    #[test]
    fn jdn_roundtrip() {
        let d = GregorianDate::new(2024, 1, 1).unwrap();
        assert_eq!(GregorianDate::from_jdn(d.to_jdn()), d);
    }
}
