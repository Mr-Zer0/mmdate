//! Traditional Burmese-script rendering of a [`MyanmarDate`].
//!
//! Gated behind the `burmese` feature. The renderer is allocation-free: it
//! writes straight into a [`core::fmt::Formatter`], so it works in `no_std`
//! and never produces an owned `String` of its own. Callers choose whether to
//! allocate (`format!`/`to_string`) or stream (`write!`/`println!`).

use core::fmt::{self, Write};

use crate::MonthPhase;
use crate::myanmar::MyanmarDate;

/// The Burmese numeral (`၀`–`၉`, U+1040..=U+1049) for a single decimal digit.
fn burmese_digit(value: u64) -> char {
    match value % 10 {
        0 => '၀',
        1 => '၁',
        2 => '၂',
        3 => '၃',
        4 => '၄',
        5 => '၅',
        6 => '၆',
        7 => '၇',
        8 => '၈',
        _ => '၉',
    }
}

/// Write a non-negative integer to `f` using Burmese numerals.
fn write_burmese_number(f: &mut fmt::Formatter<'_>, n: u64) -> fmt::Result {
    if n >= 10 {
        write_burmese_number(f, n / 10)?;
    }
    f.write_char(burmese_digit(n))
}

/// A [`MyanmarDate`] that renders as a traditional Burmese-script date string
/// via its [`Display`](fmt::Display) implementation.
///
/// Create one with [`MyanmarDate::burmese`]. The rendered form is, e.g.,
/// `၁၃၅၃ ခု၊ နယုန်လဆန်း ၅ ရက်၊ သောကြာနေ့` — year, month, moon phase, fortnight
/// day (for waxing/waning phases only), and weekday. Full-moon and new-moon
/// days carry no day number, since the phase word already identifies the day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
pub struct BurmeseDate(MyanmarDate);

impl BurmeseDate {
    pub(crate) fn new(date: MyanmarDate) -> Self {
        Self(date)
    }
}

impl fmt::Display for BurmeseDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date = self.0;
        // Year, e.g. "၁၃၅၃ ခု၊ ". Years in the supported range are positive.
        write_burmese_number(f, u64::try_from(date.year()).unwrap_or(0))?;
        f.write_str(" ခု၊ ")?;
        // Month + moon phase, e.g. "နယုန်လဆန်း".
        f.write_str(date.month().burmese_name())?;
        f.write_str(date.moon_phase().burmese_name())?;
        // Fortnight day only applies to the waxing and waning halves.
        match date.moon_phase() {
            MonthPhase::Waxing | MonthPhase::Waning => {
                f.write_char(' ')?;
                write_burmese_number(f, u64::from(date.fortnight_day()))?;
                f.write_str(" ရက်")?;
            }
            MonthPhase::Full | MonthPhase::New => {}
        }
        // Weekday, e.g. "၊ သောကြာနေ့".
        f.write_str("၊ ")?;
        f.write_str(date.weekday().burmese_name())?;
        f.write_str("နေ့")
    }
}

impl MyanmarDate {
    /// Render this date in the traditional Burmese script.
    ///
    /// Returns an allocation-free [`Display`](fmt::Display) adapter; nothing is
    /// allocated until the caller formats it.
    ///
    /// ```
    /// use myanmar_date::{GregorianDate, MyanmarDate};
    ///
    /// let gregorian = GregorianDate::new(1991, 5, 17).unwrap();
    /// let myanmar = MyanmarDate::from_gregorian(gregorian).unwrap();
    /// assert_eq!(
    ///     myanmar.burmese().to_string(),
    ///     "၁၃၅၃ ခု၊ နယုန်လဆန်း ၅ ရက်၊ သောကြာနေ့",
    /// );
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "burmese")))]
    #[must_use]
    pub fn burmese(&self) -> BurmeseDate {
        BurmeseDate::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GregorianDate;

    use std::string::{String, ToString};

    fn burmese_for(y: i32, m: u8, d: u8) -> String {
        let gregorian = GregorianDate::new(y, m, d).unwrap();
        MyanmarDate::from_gregorian(gregorian)
            .unwrap()
            .burmese()
            .to_string()
    }

    #[test]
    fn renders_the_canonical_waxing_example() {
        // 1991-05-17 == ME 1353 Nayon, waxing 5, Friday.
        assert_eq!(burmese_for(1991, 5, 17), "၁၃၅၃ ခု၊ နယုန်လဆန်း ၅ ရက်၊ သောကြာနေ့");
    }

    #[test]
    fn full_moon_omits_the_day_number() {
        // 2024-01-25 == ME 1385 Pyatho full moon (verified against upstream).
        // A full-moon day renders the phase word alone, with no numeral or
        // "ရက်" suffix.
        let s = burmese_for(2024, 1, 25);
        assert!(s.contains("လပြည့်"), "expected full-moon word in {s}");
        assert!(!s.contains("ရက်"), "full moon should omit the day: {s}");
    }

    #[test]
    fn digits_render_in_burmese_numerals() {
        let s = burmese_for(1991, 5, 17);
        // No ASCII digits should survive in the output.
        assert!(!s.chars().any(|c| c.is_ascii_digit()), "{s}");
    }
}
