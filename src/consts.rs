// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).
//! Empirical constants and era tables for the Myanmar calendar algorithm.
//!
//! These values are reproduced **verbatim** from `ceMmDateTime.js` (see
//! `GetMyConst`, lines 692-780). They are the result of historical and
//! astronomical research by the upstream author and must not be re-derived,
//! rounded, or "simplified" — doing so would silently change which years are
//! watat (leap) years and where full-moon days fall.

/// Solar year length in days (365.2587565...).
pub(crate) const SY: f64 = 1_577_917_828.0 / 4_320_000.0;

/// Lunar month length in days (29.53058795...).
pub(crate) const LM: f64 = 1_577_917_828.0 / 53_433_336.0;

/// Julian day number of the beginning of Myanmar Era (ME) year 0.
pub(crate) const MO: f64 = 1_954_168.050_623;

/// Beginning of the Gregorian calendar in JDN, British-calendar default
/// (1752-09-14). Only relevant to the advanced British/Julian cutover path;
/// the public API's default Gregorian conversion does not use it.
pub(crate) const SG: i64 = 2_361_222;

/// Start of the Thandeikta era (ME 1100), "BGNTG" in upstream naming.
pub(crate) const BGNTG: i32 = 1100;

/// Start of the third/modern era (ME 1312, after independence), "SE3" in
/// upstream naming.
pub(crate) const SE3: i32 = 1312;

/// Start of the second (British colonial) era (ME 1217).
pub(crate) const ERA2_START: i32 = 1217;

/// Start of the Makaranta system 2 era (ME 798).
pub(crate) const ERA_MAKARANTA2_START: i32 = 798;

/// Traditional base offset from Myanmar year to Sasana year (`my2sy`, `k=0`).
pub(crate) const SASANA_OFFSET: i32 = 1182;

/// One row of the full-moon-day exception table: `(myanmar_year, offset)`.
pub(crate) type FullMoonException = (i32, i32);

/// Constants and exception tables for one calendrical era.
pub(crate) struct Era {
    /// Myanmar year at/above which this era's rules apply.
    pub threshold: i32,
    /// Era id: calculation method/constants depend on this value.
    pub ei: f64,
    /// Watat offset to compensate.
    pub wo: f64,
    /// Number of months used to find excess days.
    pub nm: f64,
    /// Full-moon-day offset exceptions, sorted ascending by year.
    pub fme: &'static [FullMoonException],
    /// Watat exception years, sorted ascending.
    pub wte: &'static [i32],
}

const FME_ERA3: [FullMoonException; 1] = [(1377, 1)];
const WTE_ERA3: [i32; 2] = [1344, 1345];

const FME_ERA2: [FullMoonException; 2] = [(1234, 1), (1261, -1)];
const WTE_ERA2: [i32; 2] = [1263, 1264];

const FME_ERA1_3: [FullMoonException; 5] =
    [(1120, 1), (1126, -1), (1150, 1), (1172, -1), (1207, 1)];
const WTE_ERA1_3: [i32; 2] = [1201, 1202];

const FME_ERA1_2: [FullMoonException; 13] = [
    (813, -1),
    (849, -1),
    (851, -1),
    (854, -1),
    (927, -1),
    (933, -1),
    (936, -1),
    (938, -1),
    (949, -1),
    (952, -1),
    (963, -1),
    (968, -1),
    (1039, -1),
];
const WTE_ERA1_2: [i32; 0] = [];

const FME_ERA1_1: [FullMoonException; 10] = [
    (205, 1),
    (246, 1),
    (471, 1),
    (572, -1),
    (651, 1),
    (653, 2),
    (656, 1),
    (672, 1),
    (729, 1),
    (767, -1),
];
const WTE_ERA1_1: [i32; 0] = [];

/// Era table, ordered from the most recent (highest threshold) era first.
/// `era_for(my)` picks the first era whose threshold `my` satisfies.
pub(crate) const ERAS: [Era; 5] = [
    Era {
        threshold: SE3,
        ei: 3.0,
        wo: -0.5,
        nm: 8.0,
        fme: &FME_ERA3,
        wte: &WTE_ERA3,
    },
    Era {
        threshold: ERA2_START,
        ei: 2.0,
        wo: -1.0,
        nm: 4.0,
        fme: &FME_ERA2,
        wte: &WTE_ERA2,
    },
    Era {
        threshold: BGNTG,
        ei: 1.3,
        wo: -0.85,
        nm: -1.0,
        fme: &FME_ERA1_3,
        wte: &WTE_ERA1_3,
    },
    Era {
        threshold: ERA_MAKARANTA2_START,
        ei: 1.2,
        wo: -1.1,
        nm: -1.0,
        fme: &FME_ERA1_2,
        wte: &WTE_ERA1_2,
    },
    // The "else" branch (ME 0 - 797): no finite lower threshold.
    Era {
        threshold: i32::MIN,
        ei: 1.1,
        wo: -1.1,
        nm: -1.0,
        fme: &FME_ERA1_1,
        wte: &WTE_ERA1_1,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eras_are_sorted_descending_by_threshold() {
        for pair in ERAS.windows(2) {
            assert!(pair[0].threshold > pair[1].threshold);
        }
    }

    #[test]
    fn constants_match_upstream_verbatim() {
        assert!((SY - 365.258_756_5).abs() < 1e-7);
        assert!((LM - 29.530_587_95).abs() < 1e-8);
        assert_eq!(MO, 1_954_168.050_623);
        assert_eq!(SG, 2_361_222);
        assert_eq!(BGNTG, 1100);
        assert_eq!(SE3, 1312);
        assert_eq!(SASANA_OFFSET, 1182);
    }
}
