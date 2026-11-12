//! `chrono` interoperability (`chrono` feature, default-on).
//!
//! This is the crate's primary ergonomic entry point: most callers will
//! reach for `chrono::NaiveDate` rather than this crate's own
//! [`crate::GregorianDate`]. The bridges here are plain, allocation-free
//! `From`/`TryFrom` conversions.

use chrono::Datelike;

use crate::{Error, GregorianDate, MyanmarDate};

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<GregorianDate> for chrono::NaiveDate {
    /// Converts to `chrono::NaiveDate`.
    ///
    /// # Panics
    ///
    /// Never, for any `GregorianDate` obtained through this crate's public
    /// API: a `GregorianDate` is always already a valid proleptic-Gregorian
    /// calendar date within chrono's representable range (chrono supports
    /// years far beyond this crate's `640..=9999`), so the conversion cannot
    /// fail. The `expect` below documents that invariant rather than
    /// papering over caller-reachable failure.
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use myanmar_date::GregorianDate;
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let n: NaiveDate = g.into();
    /// assert_eq!(n, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    fn from(date: GregorianDate) -> Self {
        chrono::NaiveDate::from_ymd_opt(date.year(), u32::from(date.month()), u32::from(date.day()))
            .expect("GregorianDate is always a valid proleptic Gregorian calendar date")
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl TryFrom<chrono::NaiveDate> for GregorianDate {
    type Error = Error;

    /// Converts from `chrono::NaiveDate`, re-validating against this
    /// crate's supported year range.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if `date`'s year is outside
    /// [`myanmar_date::MIN_YEAR`](crate::MIN_YEAR)..=[`myanmar_date::MAX_YEAR`](crate::MAX_YEAR).
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use myanmar_date::GregorianDate;
    ///
    /// let n = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let g = GregorianDate::try_from(n)?;
    /// assert_eq!(g.year(), 2024);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[allow(clippy::cast_possible_truncation)] // chrono's month()/day() are always in 1..=31
    fn try_from(date: chrono::NaiveDate) -> Result<Self, Error> {
        // chrono's month()/day() are always in range for a constructed
        // NaiveDate, so only the year bound can fail here; `as u8` is safe
        // because both are small positive numbers (month 1..=12, day
        // 1..=31).
        GregorianDate::new(date.year(), date.month() as u8, date.day() as u8)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl TryFrom<chrono::NaiveDate> for MyanmarDate {
    type Error = Error;

    /// Converts a `chrono::NaiveDate` directly to its Myanmar equivalent.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] or [`Error::InvalidDate`] under the same
    /// conditions as [`MyanmarDate::from_gregorian`].
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use myanmar_date::MyanmarDate;
    ///
    /// let n = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let m = MyanmarDate::try_from(n)?;
    /// assert_eq!(m.sasana_year(), 2567);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    fn try_from(date: chrono::NaiveDate) -> Result<Self, Error> {
        MyanmarDate::from_gregorian(GregorianDate::try_from(date)?)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
impl From<MyanmarDate> for chrono::NaiveDate {
    /// Converts a Myanmar date to `chrono::NaiveDate` via its Gregorian
    /// equivalent.
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// let n: NaiveDate = m.into();
    /// assert_eq!(n, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    fn from(date: MyanmarDate) -> Self {
        date.to_gregorian().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_date_roundtrips_through_gregorian_date() {
        let n = chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let g = GregorianDate::try_from(n).unwrap();
        let back: chrono::NaiveDate = g.into();
        assert_eq!(back, n);
    }

    #[test]
    fn out_of_range_naive_date_is_rejected() {
        let n = chrono::NaiveDate::from_ymd_opt(1, 1, 1).unwrap();
        assert_eq!(GregorianDate::try_from(n).unwrap_err(), Error::OutOfRange);
    }

    #[test]
    fn naive_date_converts_to_myanmar_date_anchor() {
        let n = chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let m = MyanmarDate::try_from(n).unwrap();
        assert_eq!(m.sasana_year(), 2567);
    }
}
