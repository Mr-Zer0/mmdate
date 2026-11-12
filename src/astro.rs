// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).
//! Astrological days that are a deterministic function of the Myanmar
//! calendar date and weekday. Ported from `ceMmDateTime.js` lines 1056-1325
//! (`cal_sabbath` through `cal_astro`).
//!
//! Every function here is pure integer arithmetic over values already
//! produced by [`crate::myanmar`]'s conversion (month code, day of month,
//! year type, weekday code), so — like the rest of the crate — there is
//! nothing to allocate and every result is a `Copy` value.
//!
//! This module intentionally does **not** implement `cal_holiday`: whether a
//! day is a public/government/religious holiday is consumer policy, out of
//! scope for this crate (see `Claude.md`). The Thingyan astronomical
//! calculation itself (which `cal_holiday` also computes) lives in
//! [`crate::thingyan`] instead, separated from any holiday labeling.

// Mirrors `myanmar.rs`: this module is a close numeric port of upstream's
// integer formulas (see the module doc above), so the same cast/float lints
// that are tuned for arbitrary-width numeric code don't apply here either.
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

use crate::myanmar::{MyanmarDate, cal_mf, cal_mml};

/// Whether a Myanmar calendar day is the weekly Buddhist sabbath, or its eve.
///
/// # Examples
///
/// ```
/// use myanmar_date::{GregorianDate, MyanmarDate, Sabbath};
///
/// // ME 1385 Nadaw 15 (full moon day).
/// let g = GregorianDate::new(2023, 12, 27)?;
/// let m = MyanmarDate::from_gregorian(g)?;
/// assert_eq!(m.sabbath(), Some(Sabbath::Sabbath));
/// # Ok::<(), myanmar_date::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sabbath {
    /// The sabbath day itself.
    Sabbath,
    /// The evening before the sabbath.
    SabbathEve,
}

/// Whether a Myanmar calendar day is a pyathada, or an afternoon pyathada.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pyathada {
    /// An ordinary (all-day) pyathada.
    Pyathada,
    /// An afternoon-only pyathada.
    Afternoon,
}

/// The compass direction the mythical naga (serpent) faces this month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NagaDirection {
    /// West.
    West,
    /// North.
    North,
    /// East.
    East,
    /// South.
    South,
}

/// One of the seven mahabote (fortune) categories, derived from the Myanmar
/// year and the weekday.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mahabote {
    /// Binga.
    Binga,
    /// Atun.
    Atun,
    /// Yaza.
    Yaza,
    /// Adipati.
    Adipati,
    /// Marana.
    Marana,
    /// Thike.
    Thike,
    /// Puti.
    Puti,
}

/// One of the three nakhat (elemental) categories, derived from the Myanmar
/// year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nakhat {
    /// Ogre.
    Ogre,
    /// Elf.
    Elf,
    /// Human.
    Human,
}

/// The traditional Sanskrit year name, cycling every 12 Myanmar years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearName {
    /// Hpusha.
    Hpusha,
    /// Magha.
    Magha,
    /// Phalguni.
    Phalguni,
    /// Chitra.
    Chitra,
    /// Visakha.
    Visakha,
    /// Jyeshtha.
    Jyeshtha,
    /// Ashadha.
    Ashadha,
    /// Sravana.
    Sravana,
    /// Bhadrapaha.
    Bhadrapaha,
    /// Asvini.
    Asvini,
    /// Krittika.
    Krittika,
    /// Mrigasiras.
    Mrigasiras,
}

/// One of the ten astrological days reported by `cal_astro`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstroDay {
    /// Thamanyo.
    Thamanyo,
    /// Amyeittasote.
    Amyeittasote,
    /// Warameittugyi.
    Warameittugyi,
    /// Warameittunge.
    Warameittunge,
    /// Yatpote.
    Yatpote,
    /// Thamaphyu.
    Thamaphyu,
    /// Nagapor.
    Nagapor,
    /// Yatyotema.
    Yatyotema,
    /// Mahayatkyan.
    Mahayatkyan,
    /// Shanyat.
    Shanyat,
}

impl AstroDay {
    /// All ten astrological days, in `cal_astro`'s native order.
    const ALL: [Self; 10] = [
        Self::Thamanyo,
        Self::Amyeittasote,
        Self::Warameittugyi,
        Self::Warameittunge,
        Self::Yatpote,
        Self::Thamaphyu,
        Self::Nagapor,
        Self::Yatyotema,
        Self::Mahayatkyan,
        Self::Shanyat,
    ];
}

/// A `Copy`, allocation-free set of [`AstroDay`]s, as returned by
/// [`MyanmarDate::astro_days`].
///
/// # Examples
///
/// ```
/// use myanmar_date::{AstroDay, GregorianDate, MyanmarDate};
///
/// let g = GregorianDate::new(2024, 1, 1)?;
/// let m = MyanmarDate::from_gregorian(g)?;
/// assert!(m.astro_days().is_empty());
/// assert!(!m.astro_days().contains(AstroDay::Thamanyo));
/// # Ok::<(), myanmar_date::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AstroDays(u16);

impl AstroDays {
    fn insert(&mut self, day: AstroDay) {
        self.0 |= 1 << (day as u16);
    }

    /// Whether `day` is in this set.
    #[must_use]
    pub fn contains(&self, day: AstroDay) -> bool {
        self.0 & (1 << (day as u16)) != 0
    }

    /// Whether this set has no astrological days in it.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Iterates the astrological days in this set, in `cal_astro`'s native
    /// order.
    #[must_use]
    pub fn iter(&self) -> AstroDaysIter {
        AstroDaysIter { set: *self, idx: 0 }
    }
}

impl IntoIterator for AstroDays {
    type Item = AstroDay;
    type IntoIter = AstroDaysIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for &AstroDays {
    type Item = AstroDay;
    type IntoIter = AstroDaysIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the astrological days in an [`AstroDays`] set, returned by
/// [`AstroDays::iter`].
#[derive(Debug, Clone)]
pub struct AstroDaysIter {
    set: AstroDays,
    idx: usize,
}

impl Iterator for AstroDaysIter {
    type Item = AstroDay;

    fn next(&mut self) -> Option<AstroDay> {
        while self.idx < AstroDay::ALL.len() {
            let day = AstroDay::ALL[self.idx];
            self.idx += 1;
            if self.set.contains(day) {
                return Some(day);
            }
        }
        None
    }
}

// --- pure ports of the `cal_*` astrological-day formulas ---

fn cal_sabbath(md: i32, mm: i32, myt: i32) -> Option<Sabbath> {
    let mml = cal_mml(mm, myt);
    let mut s = 0;
    if md == 8 || md == 15 || md == 23 || md == mml {
        s = 1;
    }
    if md == 7 || md == 14 || md == 22 || md == mml - 1 {
        s = 2;
    }
    match s {
        1 => Some(Sabbath::Sabbath),
        2 => Some(Sabbath::SabbathEve),
        _ => None,
    }
}

fn cal_yatyaza(mm: i32, wd: i32) -> bool {
    // First Waso is considered (ordinary) Waso, mirroring upstream's comment;
    // `mm % 4` already folds `FirstWaso = 0` and `SecondWaso = 4` together.
    let m1 = mm % 4;
    let wd1 = m1 / 2 + 4;
    let wd2 = (1 - m1 / 2 + (m1 % 2)) * (1 + 2 * (m1 % 2));
    wd == wd1 || wd == wd2
}

fn cal_pyathada(mm: i32, wd: i32) -> Option<Pyathada> {
    let m1 = mm % 4;
    let wda = [1, 3, 3, 0, 2, 1, 2];
    let mut p = 0;
    if m1 == 0 && wd == 4 {
        p = 2;
    }
    if m1 == wda[wd as usize] {
        p = 1;
    }
    match p {
        1 => Some(Pyathada::Pyathada),
        2 => Some(Pyathada::Afternoon),
        _ => None,
    }
}

fn cal_thamanyo(mm: i32, wd: i32) -> bool {
    let mmt = mm / 13;
    let mut mm = (mm % 13) + mmt; // to 1-12 with month type
    if mm <= 0 {
        mm = 4; // first waso is considered waso
    }
    let m1 = mm - 1 - mm / 9;
    let wd1 = (m1 * 2 - m1 / 8) % 7;
    let wd2 = (wd + 7 - wd1) % 7;
    wd2 <= 1
}

fn cal_amyeittasote(md: i32, wd: i32) -> bool {
    let mf = cal_mf(md);
    let wda = [5, 8, 3, 7, 2, 4, 1];
    mf == wda[wd as usize]
}

fn cal_warameittugyi(md: i32, wd: i32) -> bool {
    let mf = cal_mf(md);
    let wda = [7, 1, 4, 8, 9, 6, 3];
    mf == wda[wd as usize]
}

fn cal_warameittunge(md: i32, wd: i32) -> bool {
    let mf = cal_mf(md);
    let wn = (wd + 6) % 7;
    12 - mf == wn
}

fn cal_yatpote(md: i32, wd: i32) -> bool {
    let mf = cal_mf(md);
    let wda = [8, 1, 4, 6, 9, 8, 7];
    mf == wda[wd as usize]
}

fn cal_thamaphyu(md: i32, wd: i32) -> bool {
    let mf = cal_mf(md);
    let wda = [1, 2, 6, 6, 5, 6, 7];
    let wdb = [0, 1, 0, 0, 0, 3, 3];
    mf == wda[wd as usize] || mf == wdb[wd as usize] || (mf == 4 && wd == 5)
}

fn cal_nagapor(md: i32, wd: i32) -> bool {
    let wda = [26, 21, 2, 10, 18, 2, 21];
    let wdb = [17, 19, 1, 0, 9, 0, 0];
    md == wda[wd as usize]
        || md == wdb[wd as usize]
        || (md == 2 && wd == 1)
        || ((md == 12 || md == 4 || md == 18) && wd == 2)
}

fn cal_yatyotema(mm: i32, md: i32) -> bool {
    let mmt = mm / 13;
    let mut mm = (mm % 13) + mmt;
    if mm <= 0 {
        mm = 4;
    }
    let mf = cal_mf(md);
    let m1 = if mm % 2 != 0 { mm } else { (mm + 9) % 12 };
    let m1 = ((m1 + 4) % 12) + 1;
    mf == m1
}

fn cal_mahayatkyan(mm: i32, md: i32) -> bool {
    // Deliberately *not* normalized through `mm % 13 + mmt` first: upstream's
    // `cal_mahayatkyan` is the one astro function that reads the raw month
    // code directly (see `ceMmDateTime.js:1254-1261`).
    let mm = if mm <= 0 { 4 } else { mm };
    let mf = cal_mf(md);
    let m1 = (((mm % 12) / 2 + 4) % 6) + 1;
    mf == m1
}

fn cal_shanyat(mm: i32, md: i32) -> bool {
    let mmt = mm / 13;
    let mut mm = (mm % 13) + mmt;
    if mm <= 0 {
        mm = 4;
    }
    let mf = cal_mf(md);
    let sya = [8, 8, 2, 2, 9, 3, 3, 5, 1, 4, 7, 4];
    mf == sya[(mm - 1) as usize]
}

fn cal_nagahle(mm: i32) -> NagaDirection {
    let mm = if mm <= 0 { 4 } else { mm }; // first waso is considered waso
    match (mm % 12) / 3 {
        0 => NagaDirection::West,
        1 => NagaDirection::North,
        2 => NagaDirection::East,
        _ => NagaDirection::South,
    }
}

// `(my - wd) % 7` in upstream's own truncated-modulo semantics; we use
// `rem_euclid` so the array index is always valid (0..=6) even for the
// small/negative `my - wd` upstream's formula can only produce outside this
// crate's supported range (`my` is always large and positive for every
// Gregorian year `>= MIN_YEAR`), rather than reproducing upstream's latent
// out-of-bounds risk for those inputs.
fn cal_mahabote(my: i32, wd: i32) -> Mahabote {
    match (my - wd).rem_euclid(7) {
        0 => Mahabote::Binga,
        1 => Mahabote::Atun,
        2 => Mahabote::Yaza,
        3 => Mahabote::Adipati,
        4 => Mahabote::Marana,
        5 => Mahabote::Thike,
        _ => Mahabote::Puti,
    }
}

/// See the `rem_euclid` note on [`cal_mahabote`]: same reasoning applies here
/// for `my % 3`.
fn cal_nakhat(my: i32) -> Nakhat {
    match my.rem_euclid(3) {
        0 => Nakhat::Ogre,
        1 => Nakhat::Elf,
        _ => Nakhat::Human,
    }
}

fn cal_year_name(my: i32) -> YearName {
    const ALL: [YearName; 12] = [
        YearName::Hpusha,
        YearName::Magha,
        YearName::Phalguni,
        YearName::Chitra,
        YearName::Visakha,
        YearName::Jyeshtha,
        YearName::Ashadha,
        YearName::Sravana,
        YearName::Bhadrapaha,
        YearName::Asvini,
        YearName::Krittika,
        YearName::Mrigasiras,
    ];
    ALL[my.rem_euclid(12) as usize]
}

impl MyanmarDate {
    /// Whether this day is the weekly Buddhist sabbath, or its eve.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.sabbath(), None);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn sabbath(&self) -> Option<Sabbath> {
        cal_sabbath(
            i32::from(self.day()),
            self.month().code(),
            self.year_type().code(),
        )
    }

    /// Whether this day is yatyaza.
    #[must_use]
    pub fn yatyaza(&self) -> bool {
        cal_yatyaza(self.month().code(), self.weekday().code())
    }

    /// Whether this day is a pyathada, or an afternoon pyathada.
    #[must_use]
    pub fn pyathada(&self) -> Option<Pyathada> {
        cal_pyathada(self.month().code(), self.weekday().code())
    }

    /// Whether this day is thamanyo.
    #[must_use]
    pub fn thamanyo(&self) -> bool {
        cal_thamanyo(self.month().code(), self.weekday().code())
    }

    /// Whether this day is amyeittasote.
    #[must_use]
    pub fn amyeittasote(&self) -> bool {
        cal_amyeittasote(i32::from(self.day()), self.weekday().code())
    }

    /// Whether this day is warameittugyi.
    #[must_use]
    pub fn warameittugyi(&self) -> bool {
        cal_warameittugyi(i32::from(self.day()), self.weekday().code())
    }

    /// Whether this day is warameittunge.
    #[must_use]
    pub fn warameittunge(&self) -> bool {
        cal_warameittunge(i32::from(self.day()), self.weekday().code())
    }

    /// Whether this day is yatpote.
    #[must_use]
    pub fn yatpote(&self) -> bool {
        cal_yatpote(i32::from(self.day()), self.weekday().code())
    }

    /// Whether this day is thamaphyu.
    #[must_use]
    pub fn thamaphyu(&self) -> bool {
        cal_thamaphyu(i32::from(self.day()), self.weekday().code())
    }

    /// Whether this day is nagapor.
    #[must_use]
    pub fn nagapor(&self) -> bool {
        cal_nagapor(i32::from(self.day()), self.weekday().code())
    }

    /// Whether this day is yatyotema.
    #[must_use]
    pub fn yatyotema(&self) -> bool {
        cal_yatyotema(self.month().code(), i32::from(self.day()))
    }

    /// Whether this day is mahayatkyan.
    #[must_use]
    pub fn mahayatkyan(&self) -> bool {
        cal_mahayatkyan(self.month().code(), i32::from(self.day()))
    }

    /// Whether this day is shanyat.
    #[must_use]
    pub fn shanyat(&self) -> bool {
        cal_shanyat(self.month().code(), i32::from(self.day()))
    }

    /// The direction the mythical naga (serpent) faces this month.
    #[must_use]
    pub fn nagahle(&self) -> NagaDirection {
        cal_nagahle(self.month().code())
    }

    /// This date's mahabote (fortune) category.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, Mahabote, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.mahabote(), Mahabote::Marana);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn mahabote(&self) -> Mahabote {
        cal_mahabote(self.year(), self.weekday().code())
    }

    /// This date's nakhat (elemental) category.
    #[must_use]
    pub fn nakhat(&self) -> Nakhat {
        cal_nakhat(self.year())
    }

    /// This date's traditional Sanskrit year name.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate, YearName};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert_eq!(m.year_name(), YearName::Jyeshtha);
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn year_name(&self) -> YearName {
        cal_year_name(self.year())
    }

    /// The set of [`AstroDay`]s (as defined by upstream's `cal_astro`) that
    /// fall on this date.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let g = GregorianDate::new(2024, 1, 1)?;
    /// let m = MyanmarDate::from_gregorian(g)?;
    /// assert!(m.astro_days().is_empty());
    /// # Ok::<(), myanmar_date::Error>(())
    /// ```
    #[must_use]
    pub fn astro_days(&self) -> AstroDays {
        let mm = self.month().code();
        let md = i32::from(self.day());
        let wd = self.weekday().code();
        let mut set = AstroDays::default();
        if cal_thamanyo(mm, wd) {
            set.insert(AstroDay::Thamanyo);
        }
        if cal_amyeittasote(md, wd) {
            set.insert(AstroDay::Amyeittasote);
        }
        if cal_warameittugyi(md, wd) {
            set.insert(AstroDay::Warameittugyi);
        }
        if cal_warameittunge(md, wd) {
            set.insert(AstroDay::Warameittunge);
        }
        if cal_yatpote(md, wd) {
            set.insert(AstroDay::Yatpote);
        }
        if cal_thamaphyu(md, wd) {
            set.insert(AstroDay::Thamaphyu);
        }
        if cal_nagapor(md, wd) {
            set.insert(AstroDay::Nagapor);
        }
        if cal_yatyotema(mm, md) {
            set.insert(AstroDay::Yatyotema);
        }
        if cal_mahayatkyan(mm, md) {
            set.insert(AstroDay::Mahayatkyan);
        }
        if cal_shanyat(mm, md) {
            set.insert(AstroDay::Shanyat);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GregorianDate, Month, YearType};

    // All expected values below were obtained by calling the corresponding
    // static `cal_*` functions directly against unmodified
    // `/Users/yannaing/experiment/mmcal/javascript/ceMmDateTime.js` in Node,
    // for the JDN/field combination noted on each test.

    #[test]
    fn sabbath_detects_sabbath_eve_and_sabbath_day() {
        // jdn=2460305: mm=9 myt=2 md=14 -> Sabbath Eve
        assert_eq!(cal_sabbath(14, 9, 2), Some(Sabbath::SabbathEve));
        // jdn=2460306: md=15 -> Sabbath
        assert_eq!(cal_sabbath(15, 9, 2), Some(Sabbath::Sabbath));
        // jdn=2460311 (anchor): md=20 -> neither
        assert_eq!(cal_sabbath(20, 9, 2), None);
        // mml-1/mml boundary: jdn=2460319/2460320, Nadaw has mml=29 in this year type
        assert_eq!(cal_sabbath(28, 9, 2), Some(Sabbath::SabbathEve));
        assert_eq!(cal_sabbath(29, 9, 2), Some(Sabbath::Sabbath));
    }

    #[test]
    fn yatyaza_matches_upstream_samples() {
        // jdn=2460301: mm=9 wd=6 -> true; jdn=2460311 (anchor): mm=9 wd=2 -> false
        assert!(cal_yatyaza(9, 6));
        assert!(!cal_yatyaza(9, 2));
    }

    #[test]
    fn pyathada_distinguishes_ordinary_and_afternoon() {
        // jdn=2460300: mm=9 wd=5 -> Pyathada
        assert_eq!(cal_pyathada(9, 5), Some(Pyathada::Pyathada));
        // jdn=2460311 (anchor): mm=9 wd=2 -> none
        assert_eq!(cal_pyathada(9, 2), None);
        // first waso (m1==0) at wd==4 -> Afternoon; mm=4 (second waso) wd=4 reproduces m1=0
        assert_eq!(cal_pyathada(4, 4), Some(Pyathada::Afternoon));
    }

    #[test]
    fn thamanyo_matches_upstream_samples() {
        // jdn=2460302: mm=9 wd=0 -> true
        assert!(cal_thamanyo(9, 0));
        // jdn=2460311 (anchor): mm=9 wd=2 -> false
        assert!(!cal_thamanyo(9, 2));
    }

    #[test]
    fn amyeittasote_matches_upstream_sample() {
        // jdn=2460336: mm=10 md=16 wd=6 -> true
        assert!(cal_amyeittasote(16, 6));
        assert!(!cal_amyeittasote(20, 2)); // anchor: false
    }

    #[test]
    fn warameittugyi_matches_upstream_sample() {
        // jdn=2460339: md=19 wd=2 -> true
        assert!(cal_warameittugyi(19, 2));
        assert!(!cal_warameittugyi(20, 2)); // anchor: false
    }

    #[test]
    fn warameittunge_matches_upstream_sample() {
        // jdn=2460303: md=12 wd=1 -> true
        assert!(cal_warameittunge(12, 1));
        assert!(!cal_warameittunge(20, 2)); // anchor: false
    }

    #[test]
    fn yatpote_matches_upstream_sample() {
        // jdn=2460312: md=21 wd=3 -> true
        assert!(cal_yatpote(21, 3));
        assert!(!cal_yatpote(20, 2)); // anchor: false
    }

    #[test]
    fn thamaphyu_matches_upstream_sample() {
        // jdn=2460312: md=21 wd=3 -> true
        assert!(cal_thamaphyu(21, 3));
        assert!(!cal_thamaphyu(20, 2)); // anchor: false
    }

    #[test]
    fn nagapor_matches_upstream_sample() {
        // jdn=2460310: md=19 wd=1 -> true
        assert!(cal_nagapor(19, 1));
        assert!(!cal_nagapor(20, 2)); // anchor: false
    }

    #[test]
    fn yatyotema_matches_upstream_sample() {
        // jdn=2460308: mm=9 md=17 -> true
        assert!(cal_yatyotema(9, 17));
        assert!(!cal_yatyotema(9, 20)); // anchor: false
    }

    #[test]
    fn mahayatkyan_matches_upstream_sample() {
        // jdn=2460309: mm=9 md=18 -> true
        assert!(cal_mahayatkyan(9, 18));
        assert!(!cal_mahayatkyan(9, 20)); // anchor: false
    }

    #[test]
    fn shanyat_matches_upstream_sample() {
        // jdn=2460307: mm=9 md=16 -> true
        assert!(cal_shanyat(9, 16));
        assert!(!cal_shanyat(9, 20)); // anchor: false
    }

    #[test]
    fn nagahle_direction_cycles_every_three_months_and_folds_first_waso() {
        assert_eq!(cal_nagahle(0), NagaDirection::North); // first waso -> treated as 4
        assert_eq!(cal_nagahle(1), NagaDirection::West);
        assert_eq!(cal_nagahle(3), NagaDirection::North);
        assert_eq!(cal_nagahle(6), NagaDirection::East);
        assert_eq!(cal_nagahle(9), NagaDirection::South); // anchor month (Nadaw)
        assert_eq!(cal_nagahle(12), NagaDirection::West);
    }

    #[test]
    fn mahabote_matches_upstream_table_for_1385() {
        let expected = [
            Mahabote::Puti,
            Mahabote::Thike,
            Mahabote::Marana,
            Mahabote::Adipati,
            Mahabote::Yaza,
            Mahabote::Atun,
            Mahabote::Binga,
        ];
        for (wd, exp) in (0i32..7).zip(expected) {
            assert_eq!(cal_mahabote(1385, wd), exp, "wd={wd}");
        }
    }

    #[test]
    fn nakhat_cycles_every_three_years() {
        assert_eq!(cal_nakhat(1383), Nakhat::Ogre);
        assert_eq!(cal_nakhat(1384), Nakhat::Elf);
        assert_eq!(cal_nakhat(1385), Nakhat::Human);
        assert_eq!(cal_nakhat(1386), Nakhat::Ogre);
    }

    #[test]
    fn year_name_is_jyeshtha_for_1385() {
        assert_eq!(cal_year_name(1385), YearName::Jyeshtha);
    }

    #[test]
    fn year_name_cycles_every_twelve_years() {
        assert_eq!(cal_year_name(0), YearName::Hpusha);
        assert_eq!(cal_year_name(11), YearName::Mrigasiras);
        assert_eq!(cal_year_name(12), YearName::Hpusha);
    }

    // --- public `MyanmarDate` API, anchored on 2024-01-01 / ME 1385 Nadaw 20
    // (cpp/test/cedate/cedate.cpp): "no astrological days on this date". ---

    #[test]
    fn anchor_date_has_no_astro_days_and_matches_cedate_cpp() {
        let g = GregorianDate::new(2024, 1, 1).unwrap();
        let m = MyanmarDate::from_gregorian(g).unwrap();
        assert_eq!(m.sabbath(), None);
        assert!(!m.yatyaza());
        assert_eq!(m.pyathada(), None);
        assert_eq!(m.year_name(), YearName::Jyeshtha);
        assert_eq!(m.nagahle(), NagaDirection::South);
        assert_eq!(m.mahabote(), Mahabote::Marana);
        assert!(m.astro_days().is_empty());
    }

    #[test]
    fn astro_days_reports_every_flag_set_on_a_known_busy_date() {
        // jdn=2460339 (ME 1385 Pyatho 19, wd=2): Thamanyo, Warameittugyi,
        // Yatpote, Mahayatkyan, Shanyat all true, matching upstream
        // `cal_astro`'s output for this exact day.
        let m = MyanmarDate::new(1385, Month::Pyatho, 19).unwrap();
        let days = m.astro_days();
        assert!(days.contains(AstroDay::Thamanyo));
        assert!(days.contains(AstroDay::Warameittugyi));
        assert!(days.contains(AstroDay::Yatpote));
        assert!(days.contains(AstroDay::Mahayatkyan));
        assert!(days.contains(AstroDay::Shanyat));
        assert!(!days.contains(AstroDay::Amyeittasote));
        assert!(!days.contains(AstroDay::Nagapor));
        let collected: std::vec::Vec<AstroDay> = days.iter().collect();
        assert_eq!(collected.len(), 5);
    }

    #[test]
    fn astro_days_default_is_empty() {
        assert!(AstroDays::default().is_empty());
        assert_eq!(AstroDays::default().iter().count(), 0);
    }

    #[test]
    fn year_type_code_helper_used_by_sabbath_matches_myt() {
        // Sanity check that `YearType::code()` round-trips through this
        // module's use of it (regression guard for the glue, not the
        // algorithm itself, which is pinned above).
        assert_eq!(YearType::BigWatat.code(), 2);
    }
}
