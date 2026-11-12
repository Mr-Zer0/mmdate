//! Library-grade error type. No `anyhow`/`thiserror` in the public API, and
//! this must compile without `std` (the default build is `no_std`).

use core::fmt;

/// Everything that can go wrong converting or constructing a date in this
/// crate.
///
/// This type is intentionally small: every variant corresponds to a
/// condition a caller can actually trigger with ordinary input. The crate
/// never panics on caller-influenced input; it returns one of these instead.
///
/// # Examples
///
/// ```
/// use myanmar_date::{Error, GregorianDate};
///
/// let err = GregorianDate::new(2024, 2, 30).unwrap_err();
/// assert_eq!(err, Error::InvalidDate);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Error {
    /// The year (Gregorian or Myanmar) falls outside the range this crate
    /// supports. See the crate documentation for the exact supported range.
    OutOfRange,
    /// The year is in range, but the month/day combination does not name a
    /// real calendar date (e.g. month 13, or day 30 in a 29-day month).
    InvalidDate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange => {
                f.write_str("year is outside the range this crate supports for calendar conversion")
            }
            Self::InvalidDate => {
                f.write_str("month/day combination does not name a valid calendar date")
            }
        }
    }
}

impl core::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn display_messages_are_non_empty_and_distinct() {
        let a = Error::OutOfRange.to_string();
        let b = Error::InvalidDate.to_string();
        assert_ne!(a, "");
        assert_ne!(b, "");
        assert_ne!(a, b);
    }

    #[test]
    fn implements_core_error_trait() {
        fn assert_error<E: core::error::Error>() {}
        assert_error::<Error>();
    }
}
