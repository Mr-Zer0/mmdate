// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).
//! Astronomically computed Thingyan (Myanmar New Year) transition points.
//! Ported from the Thingyan calculation embedded in `cal_holiday`
//! (`ceMmDateTime.js:1352-1366`).
//!
//! This module computes only the astronomical transition instants and the
//! Julian Day Numbers they round to. It deliberately does not decide which
//! days are "holidays" — that is consumer policy, out of scope for this
//! crate (see `Claude.md`). [`ThingyanPhase`] names the astronomically
//! defined phases of the transition (Akyo/Akya/Akyat/Atat), which are
//! calendrical facts, not holiday labels.

// This module's `f64 -> i64` casts are the same "exact whole number by
// mathematical construction" pattern as `myanmar.rs` (see that module's
// top-of-file note); the cast/precision lints have nothing meaningful to add.
#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::consts::{BGNTG, MO, SE3, SY};
use crate::error::Error;
use crate::gregorian::GregorianDate;
use crate::math;
use crate::myanmar::MyanmarDate;

/// One of the four astronomically-defined days of the Thingyan transition.
///
/// # Examples
///
/// ```
/// use myanmar_date::{Thingyan, ThingyanPhase};
///
/// let t = Thingyan::for_year(1386)?;
/// assert_eq!(t.phase_for_jdn(t.akya_jdn()), Some(ThingyanPhase::Akya));
/// # Ok::<(), myanmar_date::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThingyanPhase {
    /// The day before Akya.
    Akyo,
    /// The day the astronomical transition begins.
    Akya,
    /// Any day strictly between Akya and Atat.
    Akyat,
    /// The day the astronomical transition completes.
    Atat,
}

/// The astronomically computed Thingyan (Myanmar New Year) transition points
/// for a single Myanmar year.
///
/// Computed directly from the same solar-year constant (`SY`) and epoch
/// (`MO`) the rest of this crate's conversions use, so it agrees exactly with
/// [`MyanmarDate`]'s own Julian-Day-Number bridge.
///
/// This holds `f64` time-of-day fields (`akya_time`/`atat_time`), so it
/// derives `PartialEq` but not `Eq` (floats aren't `Eq`).
///
/// # Examples
///
/// ```
/// use myanmar_date::{GregorianDate, Thingyan};
///
/// // Myanmar year 1386 (2024-2025): Thingyan fell on 2024-04-13..=16.
/// let t = Thingyan::for_year(1386)?;
/// assert_eq!(t.akyo_date(), GregorianDate::new(2024, 4, 13)?);
/// assert_eq!(t.akya_date(), GregorianDate::new(2024, 4, 14)?);
/// assert_eq!(t.atat_date(), GregorianDate::new(2024, 4, 16)?);
/// assert_eq!(t.new_year_date(), GregorianDate::new(2024, 4, 17)?);
/// # Ok::<(), myanmar_date::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thingyan {
    akya_time: f64,
    atat_time: f64,
    akya: i64,
    atat: i64,
    new_year: i64,
}

impl Thingyan {
    /// Computes the Thingyan transition points for Myanmar year `my`.
    ///
    /// This is the astronomical calculation for an *ordinary* month (the
    /// common case). A date that falls in one of the two "late" months
    /// inserted at the end of a big-watat year ([`crate::Month::LateTagu`] or
    /// [`crate::Month::LateKason`]) belongs, astronomically, to the
    /// following year's transition; use [`MyanmarDate::thingyan`] for a date
    /// that might be in one of those months, since it applies that
    /// adjustment for you.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if `my` is earlier than Myanmar year
    /// 1100 (the start of the Thandeikta era), the earliest year for which
    /// this calculation is calibrated. This crate never fabricates a
    /// Thingyan date for a year outside that range.
    ///
    /// # Examples
    ///
    /// ```
    /// use myanmar_date::{Error, Thingyan};
    ///
    /// assert_eq!(Thingyan::for_year(1099).unwrap_err(), Error::OutOfRange);
    /// assert!(Thingyan::for_year(1100).is_ok());
    /// ```
    pub fn for_year(my: i32) -> Result<Self, Error> {
        Self::compute(my, 0)
    }

    /// Core computation, taking the raw Myanmar year `my` and the month-type
    /// flag `mmt` (`0` for an ordinary month, `1` for a "late" month)
    /// separately, exactly mirroring upstream's `cal_holiday`: the atat-time
    /// formula uses `my + mmt`, but the era cutover comparison (`my >= SE3`)
    /// deliberately uses the *unadjusted* `my` — reproduced faithfully here
    /// rather than "simplified" to use the same value in both places.
    pub(crate) fn compute(my: i32, mmt: i32) -> Result<Self, Error> {
        if my + mmt < BGNTG {
            return Err(Error::OutOfRange);
        }
        let ja = SY * f64::from(my + mmt) + MO; // atat time
        let jk = if my >= SE3 {
            ja - 2.169_918_982
        } else {
            ja - 2.1675
        }; // akya time
        let akya = math::round(jk) as i64;
        let atat = math::round(ja) as i64;
        Ok(Self {
            akya_time: jk,
            atat_time: ja,
            akya,
            atat,
            new_year: atat + 1,
        })
    }

    /// The akya time, as a fractional Julian Date.
    #[must_use]
    pub fn akya_time(&self) -> f64 {
        self.akya_time
    }

    /// The atat time, as a fractional Julian Date.
    #[must_use]
    pub fn atat_time(&self) -> f64 {
        self.atat_time
    }

    /// The Julian Day Number of Thingyan Akya.
    #[must_use]
    pub fn akya_jdn(&self) -> i64 {
        self.akya
    }

    /// The Julian Day Number of Thingyan Atat.
    #[must_use]
    pub fn atat_jdn(&self) -> i64 {
        self.atat
    }

    /// The Julian Day Number of Myanmar New Year's Day (`atat + 1`).
    #[must_use]
    pub fn new_year_jdn(&self) -> i64 {
        self.new_year
    }

    /// The Gregorian date of Thingyan Akyo (the day before Akya).
    #[must_use]
    pub fn akyo_date(&self) -> GregorianDate {
        GregorianDate::from_jdn(self.akya - 1)
    }

    /// The Gregorian date of Thingyan Akya.
    #[must_use]
    pub fn akya_date(&self) -> GregorianDate {
        GregorianDate::from_jdn(self.akya)
    }

    /// The Gregorian date of Thingyan Atat.
    #[must_use]
    pub fn atat_date(&self) -> GregorianDate {
        GregorianDate::from_jdn(self.atat)
    }

    /// The Gregorian date of Myanmar New Year's Day.
    #[must_use]
    pub fn new_year_date(&self) -> GregorianDate {
        GregorianDate::from_jdn(self.new_year)
    }

    /// Classifies `jdn` relative to this Thingyan's transition points.
    ///
    /// ```
    /// use myanmar_date::{Thingyan, ThingyanPhase};
    ///
    /// let t = Thingyan::for_year(1386)?;
    /// assert_eq!(t.phase_for_jdn(t.akya_jdn() - 1), Some(ThingyanPhase::Akyo));
    /// assert_eq!(t.phase_for_jdn(t.atat_jdn() + 1), None); // New Year's Day itself has no phase
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn phase_for_jdn(&self, jdn: i64) -> Option<ThingyanPhase> {
        if jdn == self.akya - 1 {
            Some(ThingyanPhase::Akyo)
        } else if jdn == self.akya {
            Some(ThingyanPhase::Akya)
        } else if jdn > self.akya && jdn < self.atat {
            Some(ThingyanPhase::Akyat)
        } else if jdn == self.atat {
            Some(ThingyanPhase::Atat)
        } else {
            None
        }
    }
}

impl MyanmarDate {
    /// Computes the Thingyan transition points for the Myanmar year this
    /// date belongs to, correctly accounting for "late" months
    /// ([`crate::Month::LateTagu`]/[`crate::Month::LateKason`]), which
    /// astronomically belong to the following year's transition.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] under the same condition as
    /// [`Thingyan::for_year`].
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?; // ME 1385 Nadaw 20
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// let t = m.thingyan()?; // the Thingyan at the end of ME 1385
    /// assert_eq!(t.new_year_date(), GregorianDate::new(2023, 4, 17)?);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    pub fn thingyan(&self) -> Result<Thingyan, Error> {
        let mmt = self.month().code() / 13;
        Thingyan::compute(self.year(), mmt)
    }

    /// The Thingyan phase (if any) this date itself falls on.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] under the same condition as
    /// [`Thingyan::for_year`].
    pub fn thingyan_phase(&self) -> Result<Option<ThingyanPhase>, Error> {
        let t = self.thingyan()?;
        Ok(t.phase_for_jdn(self.to_jdn()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Month;

    // Expected JDNs/dates below were computed directly from the formula in
    // `ceMmDateTime.js:1352-1366` in Node (not from a hand-wave): for
    // my=1386, ja=2460416.687106333 (atat), jk=2460414.5171873514 (akya),
    // round(jk)=2460415, round(ja)=2460417.

    #[test]
    fn for_year_1386_matches_known_thingyan_2024() {
        let t = Thingyan::for_year(1386).unwrap();
        assert_eq!(t.akya_jdn(), 2_460_415);
        assert_eq!(t.atat_jdn(), 2_460_417);
        assert_eq!(t.new_year_jdn(), 2_460_418);
        assert_eq!(t.akyo_date(), GregorianDate::new(2024, 4, 13).unwrap());
        assert_eq!(t.akya_date(), GregorianDate::new(2024, 4, 14).unwrap());
        assert_eq!(t.atat_date(), GregorianDate::new(2024, 4, 16).unwrap());
        assert_eq!(t.new_year_date(), GregorianDate::new(2024, 4, 17).unwrap());
    }

    #[test]
    fn for_year_1385_matches_known_thingyan_2023() {
        let t = Thingyan::for_year(1385).unwrap();
        assert_eq!(t.akya_jdn(), 2_460_049);
        assert_eq!(t.atat_jdn(), 2_460_051);
        assert_eq!(t.new_year_jdn(), 2_460_052);
        assert_eq!(t.new_year_date(), GregorianDate::new(2023, 4, 17).unwrap());
    }

    #[test]
    fn for_year_before_bgntg_is_out_of_range() {
        assert_eq!(Thingyan::for_year(1099).unwrap_err(), Error::OutOfRange);
    }

    #[test]
    fn for_year_at_bgntg_boundary_is_ok() {
        assert!(Thingyan::for_year(1100).is_ok());
    }

    #[test]
    fn phase_for_jdn_classifies_each_transition_day() {
        let t = Thingyan::for_year(1386).unwrap();
        assert_eq!(t.phase_for_jdn(2_460_414), Some(ThingyanPhase::Akyo));
        assert_eq!(t.phase_for_jdn(2_460_415), Some(ThingyanPhase::Akya));
        assert_eq!(t.phase_for_jdn(2_460_416), Some(ThingyanPhase::Akyat));
        assert_eq!(t.phase_for_jdn(2_460_417), Some(ThingyanPhase::Atat));
        assert_eq!(t.phase_for_jdn(2_460_418), None); // new year's day itself has no phase
        assert_eq!(t.phase_for_jdn(2_460_413), None);
    }

    #[test]
    fn myanmar_date_thingyan_matches_free_function_for_an_ordinary_month() {
        let m = MyanmarDate::new(1385, Month::Nadaw, 20).unwrap();
        let from_date = m.thingyan().unwrap();
        let from_year = Thingyan::for_year(1385).unwrap();
        assert_eq!(from_date, from_year);
    }

    #[test]
    fn myanmar_date_thingyan_out_of_range_before_bgntg() {
        // Construct a date whose Myanmar year is (just) below BGNTG by
        // walking back from a known-valid date's JDN.
        let early = GregorianDate::new(700, 1, 1).unwrap();
        let m = MyanmarDate::from_gregorian(early).unwrap();
        assert!(m.year() < BGNTG);
        assert_eq!(m.thingyan().unwrap_err(), Error::OutOfRange);
    }
}
