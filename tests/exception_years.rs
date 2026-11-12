//! Pins every watat-exception (`wte`) and full-moon-exception (`fme`) year
//! from the upstream era table (`ceMmDateTime.js` `GetMyConst`,
//! lines 692-780).
//!
//! # Why this reads the golden fixture rather than probing a fixed day
//!
//! A first version of this test tried to probe each exception year with a
//! fixed, hand-picked day (`Tagu 1`). That turned out to be unsound: mmcal's
//! `(my, mm)` labeling is keyed to a coarse solar-year estimate from the
//! JDN, not to "the year a human would call this", so immediately after a
//! big watat year, what looks like "next year's Tagu 1" is actually still
//! labeled as the *previous* year's "Late Tagu" — confirmed against the
//! unmodified upstream JS itself, not just this port. Every golden-vector
//! row, by construction, *is* a canonical `(my, mm, md)` triple (it came
//! straight out of upstream `j2m`), so reading the fixture sidesteps the
//! whole problem.

mod common;

use common::{load_golden_vectors, year_type_from_code};
use myanmar_date::YearType;

/// Looks up the year type upstream assigned to `my`, via the first golden
/// row whose Myanmar year is `my`.
///
/// Provenance of the expected values asserted against this: obtained by
/// running `ceMmDateTime.cal_my(my).myt` directly against
/// `/Users/yannaing/experiment/mmcal/javascript/ceMmDateTime.js` in Node.
fn year_type_of(my: i32) -> YearType {
    let rows = load_golden_vectors();
    let row = rows
        .iter()
        .find(|r| r.myanmar_year == my)
        .unwrap_or_else(|| {
            panic!("no golden row for my={my}; widen the generator's window for this year")
        });
    year_type_from_code(row.year_type)
}

#[test]
fn era3_wte_exception_years() {
    assert_eq!(year_type_of(1344), YearType::LittleWatat);
    assert_eq!(year_type_of(1345), YearType::Common);
}

#[test]
fn era2_wte_exception_years() {
    assert_eq!(year_type_of(1263), YearType::BigWatat);
    assert_eq!(year_type_of(1264), YearType::Common);
}

#[test]
fn era1_3_thandeikta_wte_exception_years() {
    assert_eq!(year_type_of(1201), YearType::LittleWatat);
    assert_eq!(year_type_of(1202), YearType::Common);
}

#[test]
fn era3_fme_exception_year() {
    // fme = [[1377, 1]]
    assert_eq!(year_type_of(1377), YearType::BigWatat);
}

#[test]
fn era2_fme_exception_years() {
    // fme = [[1234, 1], [1261, -1]]
    assert_eq!(year_type_of(1234), YearType::BigWatat);
    assert_eq!(year_type_of(1261), YearType::LittleWatat);
}

#[test]
fn era1_3_thandeikta_fme_exception_years() {
    // fme = [[1120,1],[1126,-1],[1150,1],[1172,-1],[1207,1]]
    assert_eq!(year_type_of(1120), YearType::BigWatat);
    assert_eq!(year_type_of(1126), YearType::LittleWatat);
    assert_eq!(year_type_of(1150), YearType::BigWatat);
    assert_eq!(year_type_of(1172), YearType::LittleWatat);
    assert_eq!(year_type_of(1207), YearType::BigWatat);
}

#[test]
fn era1_2_makaranta2_fme_exception_years() {
    // fme = [[813,-1],[849,-1],[851,-1],[854,-1],[927,-1],[933,-1],[936,-1],
    //        [938,-1],[949,-1],[952,-1],[963,-1],[968,-1],[1039,-1]]
    for (my, expected) in [
        (813, YearType::LittleWatat),
        (849, YearType::LittleWatat),
        (851, YearType::LittleWatat),
        (854, YearType::BigWatat),
        (927, YearType::LittleWatat),
        (933, YearType::LittleWatat),
        (936, YearType::BigWatat),
        (938, YearType::LittleWatat),
        (949, YearType::LittleWatat),
        (952, YearType::BigWatat),
        (963, YearType::LittleWatat),
        (968, YearType::LittleWatat),
        (1039, YearType::LittleWatat),
    ] {
        assert_eq!(year_type_of(my), expected, "my={my}");
    }
}

#[test]
fn era1_1_makaranta1_fme_exception_years() {
    // fme = [[205,1],[246,1],[471,1],[572,-1],[651,1],[653,2],[656,1],
    //        [672,1],[729,1],[767,-1]]
    for (my, expected) in [
        (205, YearType::BigWatat),
        (246, YearType::BigWatat),
        (471, YearType::BigWatat),
        (572, YearType::LittleWatat),
        (651, YearType::BigWatat),
        (653, YearType::BigWatat),
        (656, YearType::LittleWatat),
        (672, YearType::BigWatat),
        (729, YearType::BigWatat),
        (767, YearType::LittleWatat),
    ] {
        assert_eq!(year_type_of(my), expected, "my={my}");
    }
}
