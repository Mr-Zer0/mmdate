// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).
//! Julian Day Number <-> Western (Gregorian/Julian/British) calendar date
//! conversion, ported from `ceDateTime.w2j` / `ceDateTime.j2w`
//! (`ceMmDateTime.js` lines 61-137).
//!
//! # Integer arithmetic, not float
//!
//! Upstream represents dates as a floating-point Julian Date with a time
//! component; this crate's Phase 1 scope is date-only (no time-of-day), so
//! every call fixes `h=12, n=0, s=0`, which makes the upstream `t2d` term
//! exactly zero and every `Math.floor(int / int)` a genuine integer floor
//! division. We therefore implement this module with `i64` arithmetic and a
//! `floor_div` helper instead of carrying `f64` all the way through — same
//! results, no floating-point representation risk for a pure date type.
//!
//! The `i64 -> i32` casts at each function's boundary are in range by
//! construction: callers only ever pass years/JDNs within this crate's
//! documented supported range (`gregorian::MIN_YEAR..=MAX_YEAR`), which is
//! many orders of magnitude inside `i32`'s range.
#![allow(clippy::cast_possible_truncation)]

/// Calendar type for the Western-date bridge, mirroring upstream's `ct`.
///
/// Phase 1's public API only ever constructs [`Self::Gregorian`] (per the
/// plan: the public surface defaults to proleptic Gregorian and keeps the
/// historical British/Julian cutover as, at most, a private advanced knob).
/// The other two variants are kept non-public but implemented and tested so
/// a later release can expose them without re-deriving or re-verifying the
/// underlying math.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "British/Julian cutover kept for future advanced-knob API; not used by Phase 1"
)]
pub(crate) enum CalendarType {
    /// British calendar: Julian before `sg`, Gregorian at/after it.
    British,
    /// Proleptic Gregorian calendar (the public API's default).
    Gregorian,
    /// Proleptic Julian calendar.
    Julian,
}

/// Floor division matching JS `Math.floor(a / b)` for integers, i.e. it
/// rounds toward negative infinity rather than truncating toward zero.
pub(crate) fn floor_div(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if (r != 0) && ((r < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

/// Gregorian calendar year/month/day to Julian Day Number, at noon (so the
/// result is an exact integer JDN). Ported from `w2j` with `h=12,n=0,s=0`.
pub(crate) fn w2j(y: i32, m: i32, d: i32, ct: CalendarType, sg: i64) -> i64 {
    let a = floor_div(14 - i64::from(m), 12);
    let y = i64::from(y) + 4800 - a;
    let m = i64::from(m) + 12 * a - 3;
    let gregorian_jd = i64::from(d) + floor_div(153 * m + 2, 5) + 365 * y + floor_div(y, 4)
        - floor_div(y, 100)
        + floor_div(y, 400)
        - 32045;
    match ct {
        CalendarType::Gregorian => gregorian_jd,
        CalendarType::Julian => {
            i64::from(d) + floor_div(153 * m + 2, 5) + 365 * y + floor_div(y, 4) - 32083
        }
        CalendarType::British => {
            if gregorian_jd < sg {
                let julian_jd =
                    i64::from(d) + floor_div(153 * m + 2, 5) + 365 * y + floor_div(y, 4) - 32083;
                if julian_jd > sg { sg } else { julian_jd }
            } else {
                gregorian_jd
            }
        }
    }
}

/// Julian Day Number to Western calendar year/month/day, at noon. Ported
/// from `j2w` with the time component dropped (date-only, Phase 1 scope).
// Single-letter names (`b`, `c`, `f`, `e`, `m`, `d`, `y`) deliberately mirror
// upstream's own variable names so this can be checked line-by-line against
// `ceDateTime.j2w` in `ceMmDateTime.js`.
#[allow(clippy::many_single_char_names)]
pub(crate) fn j2w(jdn: i64, ct: CalendarType, sg: i64) -> (i32, i32, i32) {
    if matches!(ct, CalendarType::Julian) || (matches!(ct, CalendarType::British) && jdn < sg) {
        let b = jdn + 1524;
        let c = floor_div(100 * b - 12_210, 36_525); // floor((b - 122.1) / 365.25), scaled by 100 to stay integer
        let f = floor_div(36_525 * c, 100); // floor(365.25 * c)
        let e = floor_div(1_000_000 * (b - f), 30_600_100); // floor((b - f) / 30.6001)
        let m = if e > 13 { e - 13 } else { e - 1 };
        let d = b - f - floor_div(306_001 * e, 10_000); // floor(30.6001 * e)
        let y = if m < 3 { c - 4715 } else { c - 4716 };
        (y as i32, m as i32, d as i32)
    } else {
        let mut j = jdn - 1_721_119;
        let y1 = floor_div(4 * j - 1, 146_097);
        j = 4 * j - 1 - 146_097 * y1;
        let mut d = floor_div(j, 4);
        j = floor_div(4 * d + 3, 1461);
        d = 4 * d + 3 - 1461 * j;
        d = floor_div(d + 4, 4);
        let mut m = floor_div(5 * d - 3, 153);
        d = 5 * d - 3 - 153 * m;
        d = floor_div(d + 5, 5);
        let mut y = 100 * y1 + j;
        if m < 10 {
            m += 3;
        } else {
            m -= 9;
            y += 1;
        }
        (y as i32, m as i32, d as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_div_matches_truncating_division_for_positive_operands() {
        assert_eq!(floor_div(7, 2), 3);
        assert_eq!(floor_div(6, 2), 3);
    }

    #[test]
    fn floor_div_rounds_toward_negative_infinity_for_negative_numerator() {
        // JS: Math.floor(-7/2) === -4 (not -3, which truncation would give)
        assert_eq!(floor_div(-7, 2), -4);
        assert_eq!(floor_div(-6, 2), -3);
    }

    #[test]
    fn floor_div_with_negative_denominator() {
        assert_eq!(floor_div(7, -2), -4);
        assert_eq!(floor_div(-7, -2), 3);
    }

    #[test]
    fn w2j_2024_01_01_gregorian_matches_known_jdn() {
        // 2024-01-01 is JDN 2460311 (well-known reference value).
        assert_eq!(
            w2j(2024, 1, 1, CalendarType::Gregorian, 2_361_222),
            2_460_311
        );
    }

    #[test]
    fn w2j_j2w_roundtrip_gregorian() {
        for (y, m, d) in [
            (2024, 1, 1),
            (2000, 2, 29),
            (1, 1, 1),
            (9999, 12, 31),
            (1900, 3, 1),
        ] {
            let jdn = w2j(y, m, d, CalendarType::Gregorian, 2_361_222);
            let (ry, rm, rd) = j2w(jdn, CalendarType::Gregorian, 2_361_222);
            assert_eq!((ry, rm, rd), (y, m, d), "roundtrip failed for {y}-{m}-{d}");
        }
    }

    #[test]
    fn british_calendar_uses_julian_before_cutover_and_gregorian_after() {
        // 1752-09-02 (Julian, last day before the British switch) should be
        // one JDN less than 1752-09-14 (Gregorian, first day after).
        let before = w2j(1752, 9, 2, CalendarType::British, 2_361_222);
        let after = w2j(1752, 9, 14, CalendarType::British, 2_361_222);
        assert_eq!(after - before, 1);
    }

    #[test]
    fn weekday_formula_known_anchor() {
        // 2024-01-01 is a Monday. mmcal weekday: w = (jdn + 2) % 7, 0=Sat..6=Fri.
        let jdn = w2j(2024, 1, 1, CalendarType::Gregorian, 2_361_222);
        let w = (jdn + 2) % 7;
        assert_eq!(w, 2); // 2 = Monday in [Sat,Sun,Mon,...]
    }
}
