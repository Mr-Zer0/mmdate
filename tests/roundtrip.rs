//! Property-based round-trip tests across the crate's whole supported
//! range, using `proptest` (a dev-dependency only — it does not affect the
//! shipped crate's zero-required-dependency guarantee).

use myanmar_date::{Error, GregorianDate, MAX_YEAR, MIN_YEAR, MyanmarDate};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4096))]

    /// Gregorian -> Myanmar -> Gregorian always returns the original date,
    /// via both `to_gregorian()` and `GregorianDate::from_myanmar()`, and
    /// the weekday computed on each side agrees.
    #[test]
    fn gregorian_myanmar_gregorian_roundtrips(
        year in MIN_YEAR..=MAX_YEAR,
        month in 1u8..=12,
        day in 1u8..=31,
    ) {
        let Ok(gregorian) = GregorianDate::new(year, month, day) else {
            return Ok(()); // skip the (year, month, day) combos that aren't real calendar dates
        };
        let myanmar = MyanmarDate::from_gregorian(gregorian)
            .expect("every in-range GregorianDate must convert");

        prop_assert_eq!(myanmar.to_gregorian(), gregorian);
        prop_assert_eq!(GregorianDate::from_myanmar(&myanmar), gregorian);
        prop_assert_eq!(myanmar.weekday(), gregorian.weekday());
    }

    /// Every canonical Myanmar date (one obtained from `from_gregorian`)
    /// round-trips through `MyanmarDate::new` to produce the identical
    /// value — i.e. "canonical" and "constructible" agree.
    #[test]
    fn myanmar_date_reconstructs_via_new(
        year in MIN_YEAR..=MAX_YEAR,
        month in 1u8..=12,
        day in 1u8..=31,
    ) {
        let Ok(gregorian) = GregorianDate::new(year, month, day) else {
            return Ok(());
        };
        let myanmar = MyanmarDate::from_gregorian(gregorian).unwrap();

        let reconstructed = MyanmarDate::new(myanmar.year(), myanmar.month(), myanmar.day())
            .expect("a canonical MyanmarDate must round-trip through new()");
        prop_assert_eq!(reconstructed, myanmar);
    }

    /// Gregorian years outside the supported range are always rejected with
    /// `Error::OutOfRange`, never silently clamped or wrapped.
    #[test]
    fn out_of_range_years_are_always_rejected(
        year in prop_oneof![i32::MIN..MIN_YEAR, (MAX_YEAR + 1)..=i32::MAX],
        month in 1u8..=12,
        day in 1u8..=28, // stay inside every month's minimum length so only the year bound is exercised
    ) {
        prop_assert_eq!(GregorianDate::new(year, month, day).unwrap_err(), Error::OutOfRange);
    }
}
