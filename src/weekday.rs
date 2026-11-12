// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).
//! Weekday type and the `w = (jdn + 2) % 7` formula (`ceMmDateTime.js`
//! `j2s`/`get w()`), using mmcal's `[0=Sat, 1=Sun, ..., 6=Fri]` ordering.

/// A day of the week, in mmcal's native ordering (Saturday first).
///
/// # Examples
///
/// ```
/// use myanmar_date::Weekday;
///
/// assert_eq!(Weekday::Sat as i64, 0);
/// assert_eq!(Weekday::Fri as i64, 6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    /// Saturday (mmcal weekday index 0).
    Sat,
    /// Sunday (mmcal weekday index 1).
    Sun,
    /// Monday (mmcal weekday index 2).
    Mon,
    /// Tuesday (mmcal weekday index 3).
    Tue,
    /// Wednesday (mmcal weekday index 4).
    Wed,
    /// Thursday (mmcal weekday index 5).
    Thu,
    /// Friday (mmcal weekday index 6).
    Fri,
}

impl Weekday {
    /// This weekday's mmcal numeric code (`0..=6`, `[0=Sat, ..., 6=Fri]`),
    /// used internally to feed the astrological-day formulas in
    /// [`crate::astro`], which are keyed to upstream's own weekday numbering.
    pub(crate) fn code(self) -> i32 {
        match self {
            Self::Sat => 0,
            Self::Sun => 1,
            Self::Mon => 2,
            Self::Tue => 3,
            Self::Wed => 4,
            Self::Thu => 5,
            Self::Fri => 6,
        }
    }

    /// Computes the weekday for a Julian Day Number, using mmcal's
    /// `w = (jdn + 2) % 7` formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use myanmar_date::Weekday;
    ///
    /// // 2024-01-01 is JDN 2460311 and is a Monday.
    /// assert_eq!(Weekday::from_jdn(2_460_311), Weekday::Mon);
    /// ```
    #[must_use]
    pub fn from_jdn(jdn: i64) -> Self {
        // Rust's `%` is a truncated remainder and `jdn + 2` is always
        // non-negative for every JDN in this crate's supported range, so the
        // result is already in `0..7` and matches JS's `%` here.
        match (jdn + 2).rem_euclid(7) {
            0 => Self::Sat,
            1 => Self::Sun,
            2 => Self::Mon,
            3 => Self::Tue,
            4 => Self::Wed,
            5 => Self::Thu,
            _ => Self::Fri,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_anchor_2024_01_01_is_monday() {
        assert_eq!(Weekday::from_jdn(2_460_311), Weekday::Mon);
    }

    #[test]
    fn jdn_zero_maps_to_index_two() {
        assert_eq!(Weekday::from_jdn(0), Weekday::Mon);
    }

    #[test]
    fn full_week_cycles_correctly() {
        let expected = [
            Weekday::Sat,
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
        ];
        for (i, exp) in expected.iter().enumerate() {
            let i = i64::try_from(i).unwrap();
            assert_eq!(Weekday::from_jdn(i + 5), *exp); // jdn=5 => (5+2)%7=0=Sat
        }
    }
}
