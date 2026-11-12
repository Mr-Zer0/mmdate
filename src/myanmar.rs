// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).
//! Myanmar (Burmese) lunisolar calendar: era table lookup, watat (leap year)
//! determination, and Myanmar date <-> Julian Day Number conversion.
//! Ported from `ceMmDateTime.js` lines 684-1027 (`GetMyConst`, `bSearch1`,
//! `bSearch2`, `cal_watat`, `cal_my`, `j2m`, `cal_mp`, `cal_mml`, `cal_myl`,
//! `cal_mf`, `cal_md`, `m2j`, `my2sy`).
//!
//! # Why `f64` here (and not the integer approach used in `julian.rs`)
//!
//! Unlike the Western-date bridge, this algorithm's day counts are derived
//! from genuinely irrational solar/lunar constants (`SY`, `LM`, `MO`) mixed
//! with decimal fitting constants (`29.544`, `29.26`) that do not reduce to
//! exact integer ratios. We therefore keep the arithmetic in `f64` end to
//! end, exactly mirroring the upstream formulas and `Math.floor`/`Math.round`
//! call sites with our `math` helpers, and only convert to integer types at
//! the boundary once a value is mathematically guaranteed to be a whole
//! number (the same guarantee the upstream JS implementation itself relies
//! on).

// This module is a close port of upstream's floating-point formulas, with
// casts back to small integer types (years, month codes 0..=14, days
// 1..=30) at points the algorithm itself guarantees are exact whole numbers
// already in range — the same guarantee the upstream JS double-precision
// implementation relies on. `clippy::pedantic`'s cast/float-comparison lints
// are tuned for arbitrary-width numeric code, which this isn't; bounds are
// documented at each call site instead of threading `TryFrom` through a
// direct numerical port (which would obscure the line-by-line correspondence
// to `ceMmDateTime.js` that reviewers need to check this against upstream).
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp
)]

use crate::consts::{self, ERAS, MO, SASANA_OFFSET, SY};
use crate::error::Error;
use crate::gregorian::{GregorianDate, MAX_YEAR, MIN_YEAR};
use crate::math;
use crate::weekday::Weekday;

/// Constants and exception adjustments for the era containing a given
/// Myanmar year (`GetMyConst`).
struct EraConstants {
    ei: f64,
    wo: f64,
    nm: f64,
    ew: i32,
}

/// Binary search for `key` in the first element of each pair of `arr`.
/// Ported from `bSearch2` (expressed with the idiomatic half-open-interval
/// binary search shape rather than upstream's signed `l`/`u` indices, since
/// the exception tables being searched are faithfully reproduced data, not
/// this search's control flow).
fn bsearch2(key: i32, arr: &[consts::FullMoonException]) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].0.cmp(&key) {
            core::cmp::Ordering::Less => lo = mid + 1,
            core::cmp::Ordering::Greater => hi = mid,
            core::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

/// Binary search for `key` in `arr`. Ported from `bSearch1` (see
/// [`bsearch2`] for why the loop shape differs from upstream).
fn bsearch1(key: i32, arr: &[i32]) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].cmp(&key) {
            core::cmp::Ordering::Less => lo = mid + 1,
            core::cmp::Ordering::Greater => hi = mid,
            core::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

/// Looks up the era constants (with exception-table adjustments already
/// applied) for Myanmar year `my`. Ported from `GetMyConst`.
fn era_constants(my: i32) -> EraConstants {
    // ERAS ends with a `threshold: i32::MIN` catch-all, so this always finds
    // a match.
    let era = ERAS
        .iter()
        .find(|e| my >= e.threshold)
        .expect("era table has a catch-all entry for every myanmar year");
    let mut wo = era.wo;
    if let Some(idx) = bsearch2(my, era.fme) {
        wo += f64::from(era.fme[idx].1);
    }
    let ew = i32::from(bsearch1(my, era.wte).is_some());
    EraConstants {
        ei: era.ei,
        wo,
        nm: era.nm,
        ew,
    }
}

/// Result of checking whether `my` is a watat (intercalary month) year.
struct Watat {
    /// 1 = watat, 0 = common.
    watat: i32,
    /// Full moon day of the (2nd) Waso, as a Julian Day Number.
    fm: f64,
}

/// Checks watat (intercalary month) for Myanmar year `my`. Ported from
/// `cal_watat`.
fn cal_watat(my: i32) -> Watat {
    let c = era_constants(my);
    let ta = (SY / 12.0 - crate::consts::LM) * (12.0 - c.nm); // threshold to adjust
    let lm = crate::consts::LM;
    let mut ed = (SY * (f64::from(my) + 3739.0)) % lm; // excess day
    if ed < ta {
        ed += lm;
    }
    let fm = math::round(SY * f64::from(my) + MO - ed + 4.5 * lm + c.wo);
    let mut watat = if c.ei >= 2.0 {
        // 2nd era or later: find watat based on excess days.
        let tw = lm - (SY / 12.0 - lm) * c.nm;
        i32::from(ed >= tw)
    } else {
        // 1st era: 19-year Metonic cycle. Myanmar year divided by 19 has an
        // intercalary month if the remainder is 2,5,7,10,13,15,18.
        let mut w = (f64::from(my) * 7.0 + 2.0) % 19.0;
        if w < 0.0 {
            w += 19.0;
        }
        math::floor(w / 12.0) as i32
    };
    watat ^= c.ew; // correct watat exceptions
    Watat { watat, fm }
}

/// Myanmar year type and anchor days. Ported from `cal_my`.
struct YearInfo {
    /// Year type: 0 = common, 1 = little watat, 2 = big watat.
    myt: i32,
    /// First day of Tagu, as a Julian Day Number.
    tg1: f64,
    /// Full moon day of (2nd) Waso, as a Julian Day Number.
    #[allow(
        dead_code,
        reason = "parity field, not consumed by any Phase 1 conversion; useful for Phase 2 astro days"
    )]
    fm: f64,
    /// Watat discrepancy: 0 = ok, 1 = error (upstream parity flag, not
    /// currently surfaced on the public API).
    #[allow(
        dead_code,
        reason = "parity field, not consumed by any Phase 1 conversion"
    )]
    werr: i32,
}

fn cal_my(my: i32) -> YearInfo {
    let y2 = cal_watat(my);
    let mut myt = y2.watat;
    let mut yd = 0i32;
    let mut y1;
    // Ported do-while: always runs at least once, then continues while the
    // previous year was common and we haven't looked back 3 years.
    loop {
        yd += 1;
        y1 = cal_watat(my - yd);
        if !(y1.watat == 0 && yd < 3) {
            break;
        }
    }
    let (fm, werr) = if myt != 0 {
        let nd = (y2.fm - y1.fm) % 354.0;
        myt = math::floor(nd / 31.0) as i32 + 1;
        let werr = i32::from(nd != 30.0 && nd != 31.0);
        (y2.fm, werr)
    } else {
        (y1.fm + 354.0 * f64::from(yd), 0)
    };
    let tg1 = y1.fm + 354.0 * f64::from(yd) - 102.0;
    YearInfo { myt, tg1, fm, werr }
}

/// A Myanmar calendar date as decomposed from a Julian Day Number: year,
/// year type, month (mmcal's non-sequential numbering), and day of month.
pub(crate) struct MyanmarYmd {
    pub(crate) myt: i32,
    pub(crate) my: i32,
    pub(crate) mm: i32,
    pub(crate) md: i32,
}

/// Julian Day Number to Myanmar date. Ported from `j2m`.
// Single-letter names mirror upstream's own `j2m` variable names so this can
// be checked line-by-line against `ceMmDateTime.js`.
#[allow(clippy::many_single_char_names)]
pub(crate) fn j2m(jdn: i64) -> MyanmarYmd {
    let jdn_f = jdn as f64;
    let my = math::floor((jdn_f - 0.5 - MO) / SY) as i32;
    let yo = cal_my(my);
    let mut dd = jdn_f - yo.tg1 + 1.0; // day count
    let b = math::floor(f64::from(yo.myt) / 2.0);
    let c = math::floor(1.0 / (f64::from(yo.myt) + 1.0)); // big wa and common yr
    let myl = 354.0 + (1.0 - c) * 30.0 + b; // year length
    let mmt = math::floor((dd - 1.0) / myl); // month type: late=1 or early=0
    dd -= mmt * myl;
    let a = math::floor((dd + 423.0) / 512.0); // adjust day count and threshold
    let mut mm = math::floor((dd - b * a + c * a * 30.0 + 29.26) / 29.544); // month
    let e = math::floor((mm + 12.0) / 16.0);
    let f = math::floor((mm + 11.0) / 16.0);
    let md = dd - math::floor(29.544 * mm - 29.26) - b * e + c * f * 30.0; // day
    mm += f * 3.0 - e * 4.0 + 12.0 * mmt; // adjust month numbers for late months
    MyanmarYmd {
        myt: yo.myt,
        my,
        mm: mm as i32,
        md: md as i32,
    }
}

/// Myanmar date to Julian Day Number. Ported from `m2j`.
pub(crate) fn m2j(my: i32, mm_in: i32, md: i32) -> i64 {
    let yo = cal_my(my);
    let mmt = math::floor(f64::from(mm_in) / 13.0);
    let mut mm = (f64::from(mm_in) % 13.0) + mmt; // to 1-12 with month type
    let b = math::floor(f64::from(yo.myt) / 2.0);
    // Not a midpoint: this is `1 - floor((myt + 1) / 2)`, a direct port of
    // upstream's `1 - Math.floor((yo.myt + 1) / 2)`.
    #[allow(clippy::manual_midpoint)]
    let c = 1.0 - math::floor((f64::from(yo.myt) + 1.0) / 2.0); // if big watat and common year
    mm += 4.0 - math::floor((mm + 15.0) / 16.0) * 4.0 + math::floor((mm + 12.0) / 16.0); // adjust month
    let mut dd = f64::from(md) + math::floor(29.544 * mm - 29.26)
        - c * math::floor((mm + 11.0) / 16.0) * 30.0
        + b * math::floor((mm + 12.0) / 16.0);
    let myl = 354.0 + (1.0 - c) * 30.0 + b;
    dd += mmt * myl; // adjust day count with year length
    (dd + yo.tg1 - 1.0) as i64
}

/// Length of Myanmar month `mm` (mmcal numbering) in year type `myt`, in
/// days (29 or 30). Ported from `cal_mml`.
pub(crate) fn cal_mml(mm: i32, myt: i32) -> i32 {
    let mut mml = 30 - mm.rem_euclid(2);
    if mm == 3 {
        mml += myt / 2; // adjust if Nayon in big watat
    }
    mml
}

/// Apparent length of the Myanmar year, in days (354, 384, or 385). Ported
/// from `cal_myl`. Not currently exposed on the public API (Phase 1 only
/// asks for `month_length()`); kept for parity and potential future use.
#[allow(
    dead_code,
    reason = "ported for parity; no Phase 1 public getter needs it yet"
)]
pub(crate) fn cal_myl(myt: i32) -> i32 {
    354 + (1 - (1 / (myt + 1))) * 30 + myt / 2
}

/// Moon phase from day of month, month, and year type:
/// `[0=waxing, 1=full moon, 2=waning, 3=new moon]`. Ported from `cal_mp`.
pub(crate) fn cal_mp(md: i32, mm: i32, myt: i32) -> i32 {
    let mml = cal_mml(mm, myt);
    (md + 1) / 16 + md / 16 + md / mml
}

/// Fortnight day `[1-15]` from day of month `[1-30]`. Ported from `cal_mf`.
pub(crate) fn cal_mf(md: i32) -> i32 {
    md - 15 * (md / 16)
}

/// Day of month from fortnight day, moon phase, month, and year type.
/// Ported from `cal_md`. (Not currently exposed publicly; kept for parity
/// with upstream and for future use once construction from
/// fortnight-day/moon-phase is wanted.)
#[allow(dead_code)]
pub(crate) fn cal_md(mf: i32, mp: i32, mm: i32, myt: i32) -> i32 {
    let mml = cal_mml(mm, myt);
    let m1 = mp % 2;
    let m2 = mp / 2;
    m1 * (15 + m2 * (mml - 15)) + (1 - m1) * (mf + 15 * m2)
}

/// Myanmar year to Sasana year. Ported from `my2sy`. `k=0` (year depends
/// only on the sun), `k=1` (Sasana year starts on first day of Tagu), `k=2`
/// (Sasana year starts on Kason full moon day).
pub(crate) fn my2sy(my: i32, mm: i32, md: i32, k: i32) -> i32 {
    let mut offset = SASANA_OFFSET;
    if k == 1 {
        if mm >= 13 {
            offset = 1183;
        }
    } else if k == 2 && (mm == 1 || (mm == 2 && md < 15)) {
        offset = 1181;
    }
    my + offset
}

/// A Myanmar calendar month, using mmcal's native (non-sequential)
/// numbering: the intercalary "1st Waso" is `0`, and the two "late" months
/// inserted at the end of a watat year come after `Tabaung` as `13`/`14`.
///
/// # Examples
///
/// ```
/// use myanmar_date::Month;
///
/// assert_eq!(Month::FirstWaso as i32, 0);
/// assert_eq!(Month::Tagu as i32, 1);
/// assert_eq!(Month::LateKason as i32, 14);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Month {
    /// The intercalary "1st Waso", inserted only in watat (leap) years.
    FirstWaso = 0,
    /// Tagu, the first ordinary month of the year.
    Tagu = 1,
    /// Kason.
    Kason = 2,
    /// Nayon.
    Nayon = 3,
    /// The ordinary "2nd Waso".
    SecondWaso = 4,
    /// Wagaung.
    Wagaung = 5,
    /// Tawthalin.
    Tawthalin = 6,
    /// Thadingyut.
    Thadingyut = 7,
    /// Tazaungmon.
    Tazaungmon = 8,
    /// Nadaw.
    Nadaw = 9,
    /// Pyatho.
    Pyatho = 10,
    /// Tabodwe.
    Tabodwe = 11,
    /// Tabaung, the last ordinary month of the year.
    Tabaung = 12,
    /// Late Tagu, inserted only in a big watat year.
    LateTagu = 13,
    /// Late Kason, inserted only in a big watat year.
    LateKason = 14,
}

impl Month {
    /// All 15 months, in mmcal's native numeric order.
    pub(crate) const ALL: [Month; 15] = [
        Month::FirstWaso,
        Month::Tagu,
        Month::Kason,
        Month::Nayon,
        Month::SecondWaso,
        Month::Wagaung,
        Month::Tawthalin,
        Month::Thadingyut,
        Month::Tazaungmon,
        Month::Nadaw,
        Month::Pyatho,
        Month::Tabodwe,
        Month::Tabaung,
        Month::LateTagu,
        Month::LateKason,
    ];

    pub(crate) fn from_code(code: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.code() == code)
    }

    pub(crate) fn code(self) -> i32 {
        self as i32
    }
}

/// Whether a Myanmar year is common, or carries an intercalary month
/// (watat), and if so whether that intercalary month has an extra day
/// (big watat).
///
/// # Examples
///
/// ```
/// use myanmar_date::YearType;
///
/// assert_ne!(YearType::Common, YearType::BigWatat);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearType {
    /// No intercalary month; 354 days.
    Common,
    /// One intercalary 30-day month; 384 days.
    LittleWatat,
    /// One intercalary 30-day month plus one extra day; 385 days.
    BigWatat,
}

impl YearType {
    pub(crate) fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Common),
            1 => Some(Self::LittleWatat),
            2 => Some(Self::BigWatat),
            _ => None,
        }
    }

    pub(crate) fn code(self) -> i32 {
        match self {
            Self::Common => 0,
            Self::LittleWatat => 1,
            Self::BigWatat => 2,
        }
    }
}

/// The moon's phase on a given Myanmar calendar day.
///
/// # Examples
///
/// ```
/// use myanmar_date::MonthPhase;
///
/// assert_ne!(MonthPhase::Waxing, MonthPhase::Full);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonthPhase {
    /// The moon is waxing (growing).
    Waxing,
    /// Full moon day.
    Full,
    /// The moon is waning (shrinking).
    Waning,
    /// New moon day.
    New,
}

impl MonthPhase {
    pub(crate) fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Waxing),
            1 => Some(Self::Full),
            2 => Some(Self::Waning),
            3 => Some(Self::New),
            _ => None,
        }
    }
}

/// A Myanmar (Burmese) calendar date.
///
/// Every `MyanmarDate` in existence represents a real, round-trip-verified
/// instant: it is only ever produced by [`MyanmarDate::from_gregorian`] or
/// [`MyanmarDate::new`], both of which validate against the conversion
/// algorithm itself rather than a hand-written rule table, so there is no
/// way to end up with (for example) a "1st Waso" in a common year.
///
/// # Examples
///
/// ```
/// use myanmar_date::{GregorianDate, Month, MyanmarDate};
///
/// let g = GregorianDate::new(2024, 1, 1)?;
/// let m = MyanmarDate::from_gregorian(g)?;
/// assert_eq!(m.year(), 1385);
/// assert_eq!(m.month(), Month::Nadaw);
/// assert_eq!(m.day(), 20);
/// # Ok::<(), myanmar_date::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MyanmarDate {
    year: i32,
    month: Month,
    day: u8,
    year_type: YearType,
    jdn: i64,
}

impl MyanmarDate {
    /// Converts a Gregorian date to its Myanmar equivalent.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the resulting Myanmar year, or
    /// [`Error::InvalidDate`] if the algorithm otherwise produces a
    /// representation this crate doesn't recognize (not expected to happen
    /// for any `GregorianDate`, which is already range-checked, but checked
    /// defensively rather than panicking).
    ///
    /// # Examples
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.sasana_year(), 2567);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    pub fn from_gregorian(date: GregorianDate) -> Result<Self, Error> {
        let jdn = date.to_jdn();
        let ymd = j2m(jdn);
        let month = Month::from_code(ymd.mm).ok_or(Error::InvalidDate)?;
        let year_type = YearType::from_code(ymd.myt).ok_or(Error::InvalidDate)?;
        let day = u8::try_from(ymd.md).map_err(|_| Error::InvalidDate)?;
        Ok(Self {
            year: ymd.my,
            month,
            day,
            year_type,
            jdn,
        })
    }

    /// Builds a validated Myanmar date directly from its year/month/day.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the equivalent Gregorian year falls
    /// outside the crate's supported range, or [`Error::InvalidDate`] if
    /// `(year, month, day)` does not name a real Myanmar calendar date (for
    /// example, day 30 in a 29-day month, or [`Month::FirstWaso`] in a
    /// common year). Validity is established by round-tripping through the
    /// same algorithm used for all conversions, so it is exact rather than
    /// an approximation.
    ///
    /// # Examples
    ///
    /// ```
    /// use myanmar_date::{Error, Month, MyanmarDate};
    ///
    /// assert_eq!(MyanmarDate::new(1385, Month::Nadaw, 32).unwrap_err(), Error::InvalidDate);
    /// ```
    pub fn new(year: i32, month: Month, day: u8) -> Result<Self, Error> {
        let jdn = m2j(year, month.code(), i32::from(day));
        let gregorian_year = GregorianDate::from_jdn(jdn).year();
        if !(MIN_YEAR..=MAX_YEAR).contains(&gregorian_year) {
            return Err(Error::OutOfRange);
        }
        let roundtrip = j2m(jdn);
        if roundtrip.my != year || roundtrip.mm != month.code() || roundtrip.md != i32::from(day) {
            return Err(Error::InvalidDate);
        }
        let year_type = YearType::from_code(roundtrip.myt).ok_or(Error::InvalidDate)?;
        Ok(Self {
            year,
            month,
            day,
            year_type,
            jdn,
        })
    }

    /// The Myanmar year.
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// The month.
    #[must_use]
    pub fn month(&self) -> Month {
        self.month
    }

    /// The day of the month, `1..=30`.
    #[must_use]
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Whether the year is common, little watat, or big watat.
    #[must_use]
    pub fn year_type(&self) -> YearType {
        self.year_type
    }

    /// The moon's phase on this date.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MonthPhase, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.moon_phase(), MonthPhase::Waning);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn moon_phase(&self) -> MonthPhase {
        let code = cal_mp(
            i32::from(self.day),
            self.month.code(),
            self.year_type.code(),
        );
        // SAFETY net, not `unsafe`: `cal_mp` always returns 0..=3 for a
        // `self.day` that passed this type's own round-trip validation.
        MonthPhase::from_code(code).unwrap_or(MonthPhase::New)
    }

    /// The fortnight day, `1..=15` (day of the waxing or waning half-month).
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.fortnight_day(), 5);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn fortnight_day(&self) -> u8 {
        cal_mf(i32::from(self.day)) as u8
    }

    /// The length of this date's month, in days (29 or 30).
    #[must_use]
    pub fn month_length(&self) -> u8 {
        cal_mml(self.month.code(), self.year_type.code()) as u8
    }

    /// The Sasana (Buddhist) year, using the traditional sun-only offset
    /// (`k=0`: does not take the moon phase into account).
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.sasana_year(), 2567);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn sasana_year(&self) -> i32 {
        my2sy(self.year, self.month.code(), i32::from(self.day), 0)
    }

    /// The day of the week.
    #[must_use]
    pub fn weekday(&self) -> Weekday {
        Weekday::from_jdn(self.jdn)
    }

    /// Converts this Myanmar date to its Gregorian equivalent.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.to_gregorian(), g);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn to_gregorian(&self) -> GregorianDate {
        GregorianDate::from_jdn(self.jdn)
    }

    pub(crate) fn to_jdn(self) -> i64 {
        self.jdn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- era_constants (GetMyConst) pins, derived analytically from the
    // upstream table in ceMmDateTime.js:692-780. ---

    #[test]
    fn era3_base_values_with_no_exceptions() {
        let e = era_constants(1312);
        assert_eq!(e.ei, 3.0);
        assert_eq!(e.wo, -0.5);
        assert_eq!(e.nm, 8.0);
        assert_eq!(e.ew, 0);
    }

    #[test]
    fn era3_wte_exception_years_set_ew() {
        for my in [1344, 1345] {
            assert_eq!(era_constants(my).ew, 1, "my={my}");
        }
    }

    #[test]
    fn era3_fme_exception_year_adjusts_wo() {
        let e = era_constants(1377);
        assert_eq!(e.wo, -0.5 + 1.0);
        assert_eq!(e.ew, 0);
    }

    #[test]
    fn era2_base_values_and_exceptions() {
        let base = era_constants(1217);
        assert_eq!(base.ei, 2.0);
        assert_eq!(base.wo, -1.0);
        assert_eq!(base.nm, 4.0);

        assert_eq!(era_constants(1234).wo, -1.0 + 1.0);
        assert_eq!(era_constants(1261).wo, -1.0 - 1.0);
        for my in [1263, 1264] {
            assert_eq!(era_constants(my).ew, 1, "my={my}");
        }
    }

    #[test]
    fn era1_3_thandeikta_base_values_and_exceptions() {
        let base = era_constants(1100);
        assert_eq!(base.ei, 1.3);
        assert_eq!(base.wo, -0.85);
        assert_eq!(base.nm, -1.0);

        assert_eq!(era_constants(1207).wo, -0.85 + 1.0);
        for my in [1201, 1202] {
            assert_eq!(era_constants(my).ew, 1, "my={my}");
            // 1201/1202 are not in the fme table, so wo is unmodified.
            assert_eq!(era_constants(my).wo, -0.85);
        }
    }

    #[test]
    fn era1_2_makaranta2_base_values_and_exceptions() {
        let base = era_constants(798);
        assert_eq!(base.ei, 1.2);
        assert_eq!(base.wo, -1.1);
        assert_eq!(base.nm, -1.0);
        assert_eq!(base.ew, 0);

        for my in [
            813, 849, 851, 854, 927, 933, 936, 938, 949, 952, 963, 968, 1039,
        ] {
            assert_eq!(era_constants(my).wo, -1.1 - 1.0, "my={my}");
        }
    }

    #[test]
    fn era1_1_makaranta1_base_values_and_exceptions() {
        let base = era_constants(797);
        assert_eq!(base.ei, 1.1);
        assert_eq!(base.wo, -1.1);
        assert_eq!(base.nm, -1.0);
        assert_eq!(base.ew, 0);

        assert_eq!(era_constants(653).wo, -1.1 + 2.0);
        assert_eq!(era_constants(572).wo, -1.1 - 1.0);
        assert_eq!(era_constants(767).wo, -1.1 - 1.0);
    }

    // --- month/day helper pins ---

    #[test]
    fn cal_mml_is_29_or_30_except_nayon_big_watat() {
        assert_eq!(cal_mml(1, 0), 29); // Tagu, odd -> 30-1=29
        assert_eq!(cal_mml(2, 0), 30); // Kason, even -> 30-0=30
        assert_eq!(cal_mml(3, 0), 29); // Nayon common
        assert_eq!(cal_mml(3, 2), 30); // Nayon big watat: +floor(2/2)=1
        assert_eq!(cal_mml(3, 1), 29); // Nayon little watat: +floor(1/2)=0
    }

    #[test]
    fn cal_mf_wraps_at_16() {
        assert_eq!(cal_mf(1), 1);
        assert_eq!(cal_mf(15), 15);
        assert_eq!(cal_mf(16), 1);
        assert_eq!(cal_mf(30), 15);
    }

    #[test]
    fn cal_mp_waxing_full_waning_new() {
        // common Tagu (mm=1, myt=0) has mml=29
        assert_eq!(cal_mp(1, 1, 0), 0); // waxing
        assert_eq!(cal_mp(15, 1, 0), 1); // full moon
        assert_eq!(cal_mp(16, 1, 0), 2); // waning
        assert_eq!(cal_mp(29, 1, 0), 3); // new moon (mml=29)
    }

    #[test]
    fn cal_md_inverts_cal_mf_and_cal_mp() {
        for mm in [1, 2, 3, 4, 9] {
            for myt in [0, 1, 2] {
                let mml = cal_mml(mm, myt);
                for md in 1..=mml {
                    let mf = cal_mf(md);
                    let mp = cal_mp(md, mm, myt);
                    assert_eq!(cal_md(mf, mp, mm, myt), md, "mm={mm} myt={myt} md={md}");
                }
            }
        }
    }

    #[test]
    fn my2sy_default_k0_is_plain_offset() {
        assert_eq!(my2sy(1385, 9, 20, 0), 1385 + 1182);
    }

    #[test]
    fn my2sy_k1_bumps_late_months_to_next_year() {
        assert_eq!(my2sy(1385, 13, 1, 1), 1385 + 1183); // late Tagu
        assert_eq!(my2sy(1385, 1, 1, 1), 1385 + 1182); // ordinary month unaffected
    }

    #[test]
    fn my2sy_k2_treats_pre_kason_full_moon_as_previous_year() {
        assert_eq!(my2sy(1385, 1, 1, 2), 1385 + 1181); // Tagu, before Kason FM
        assert_eq!(my2sy(1385, 2, 10, 2), 1385 + 1181); // Kason, before FM day 15
        assert_eq!(my2sy(1385, 2, 15, 2), 1385 + 1182); // Kason, on/after FM day
    }

    // --- anchor: 2024-01-01 is JDN 2460311, hand-verified against
    // cpp/test/cedate/cedate.cpp to be ME 1385 Nadaw 20. ---

    #[test]
    fn anchor_2024_01_01_converts_to_1385_nadaw_20() {
        let ymd = j2m(2_460_311);
        assert_eq!(ymd.my, 1385);
        assert_eq!(ymd.mm, 9); // Nadaw
        assert_eq!(ymd.md, 20);
    }

    #[test]
    fn anchor_moon_phase_and_fortnight_day_match_cedate_cpp() {
        let ymd = j2m(2_460_311);
        assert_eq!(cal_mp(ymd.md, ymd.mm, ymd.myt), 2); // waning
        assert_eq!(cal_mf(ymd.md), 5);
    }

    #[test]
    fn anchor_sasana_year_matches_cedate_cpp() {
        let ymd = j2m(2_460_311);
        assert_eq!(my2sy(ymd.my, ymd.mm, ymd.md, 0), 2567);
    }

    #[test]
    fn m2j_j2m_roundtrip_around_anchor() {
        let ymd = j2m(2_460_311);
        let jdn = m2j(ymd.my, ymd.mm, ymd.md);
        assert_eq!(jdn, 2_460_311);
    }

    #[test]
    fn j2m_m2j_roundtrip_over_a_dense_range() {
        // A multi-year dense range spanning at least one watat year.
        for jdn in 2_460_000..2_461_500 {
            let ymd = j2m(jdn);
            let back = m2j(ymd.my, ymd.mm, ymd.md);
            assert_eq!(back, jdn, "roundtrip failed for jdn={jdn}");
        }
    }

    // --- public `MyanmarDate`/`Month`/`YearType`/`MonthPhase` API ---

    #[test]
    fn from_gregorian_anchor_matches_cedate_cpp() {
        let g = GregorianDate::new(2024, 1, 1).unwrap();
        let m = MyanmarDate::from_gregorian(g).unwrap();
        assert_eq!(m.year(), 1385);
        assert_eq!(m.month(), Month::Nadaw);
        assert_eq!(m.day(), 20);
        assert_eq!(m.moon_phase(), MonthPhase::Waning);
        assert_eq!(m.fortnight_day(), 5);
        assert_eq!(m.sasana_year(), 2567);
        assert_eq!(m.weekday(), Weekday::Mon);
        assert_eq!(m.month_length(), 29);
    }

    #[test]
    fn gregorian_from_myanmar_roundtrips() {
        let g = GregorianDate::new(2024, 1, 1).unwrap();
        let m = MyanmarDate::from_gregorian(g).unwrap();
        assert_eq!(GregorianDate::from_myanmar(&m), g);
        assert_eq!(m.to_gregorian(), g);
    }

    #[test]
    fn new_rejects_day_beyond_month_length() {
        assert_eq!(
            MyanmarDate::new(1385, Month::Nadaw, 30).unwrap_err(),
            Error::InvalidDate
        );
        assert_eq!(
            MyanmarDate::new(1385, Month::Nadaw, 0).unwrap_err(),
            Error::InvalidDate
        );
    }

    #[test]
    fn new_accepts_the_anchor_date_directly() {
        let m = MyanmarDate::new(1385, Month::Nadaw, 20).unwrap();
        let g = GregorianDate::new(2024, 1, 1).unwrap();
        assert_eq!(m.to_gregorian(), g);
    }

    #[test]
    fn first_waso_only_valid_in_watat_years() {
        // Find both a common year and a watat year to exercise both branches
        // without hard-coding external ground truth for which years those are.
        let common_year = (1300..1400)
            .find(|&y| cal_my(y).myt == 0)
            .expect("a common year exists nearby");
        let watat_year = (1300..1400)
            .find(|&y| cal_my(y).myt != 0)
            .expect("a watat year exists nearby");

        assert_eq!(
            MyanmarDate::new(common_year, Month::FirstWaso, 1).unwrap_err(),
            Error::InvalidDate
        );
        assert!(MyanmarDate::new(watat_year, Month::FirstWaso, 1).is_ok());
    }

    #[test]
    fn month_from_code_round_trips_for_every_valid_code() {
        for code in 0..15 {
            assert_eq!(Month::from_code(code).unwrap().code(), code);
        }
        assert!(Month::from_code(-1).is_none());
        assert!(Month::from_code(15).is_none());
    }

    #[test]
    fn year_type_from_code_round_trips() {
        for code in 0..3 {
            assert_eq!(YearType::from_code(code).unwrap().code(), code);
        }
        assert!(YearType::from_code(3).is_none());
        assert!(YearType::from_code(-1).is_none());
    }
}
