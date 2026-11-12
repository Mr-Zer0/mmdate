//! Checks this crate's conversion against ground-truth vectors generated
//! directly from the upstream reference implementation
//! (`util/generate_fixtures.js`, reading
//! `javascript/ceMmDateTime.js` from yan9a/mmcal, MIT © 2018 Yan Naing Aye).
//!
//! Coverage: a dense recent range, every era boundary (around Myanmar years
//! 798, 1100, 1217, 1312), and every `fme`/`wte` exception year from
//! `GetMyConst` — see `util/generate_fixtures.js` for the exact windows.

mod common;

use common::{
    load_golden_vectors, month_from_code, month_phase_from_code, weekday_from_code,
    year_type_from_code,
};
use myanmar_date::{GregorianDate, MyanmarDate};

#[test]
fn golden_vectors_fixture_is_present_and_substantial() {
    let rows = load_golden_vectors();
    assert!(
        rows.len() > 10_000,
        "expected a substantial golden-vector fixture, got {} rows",
        rows.len()
    );
}

#[test]
fn gregorian_to_myanmar_matches_upstream_for_every_golden_row() {
    let rows = load_golden_vectors();
    for row in &rows {
        let gregorian =
            GregorianDate::new(row.gregorian_year, row.gregorian_month, row.gregorian_day)
                .unwrap_or_else(|e| panic!("row {row:?}: GregorianDate::new failed: {e}"));
        let myanmar = MyanmarDate::from_gregorian(gregorian)
            .unwrap_or_else(|e| panic!("row {row:?}: from_gregorian failed: {e}"));

        assert_eq!(myanmar.year(), row.myanmar_year, "row={row:?}");
        assert_eq!(
            myanmar.month(),
            month_from_code(row.myanmar_month),
            "row={row:?}"
        );
        assert_eq!(myanmar.day(), row.myanmar_day, "row={row:?}");
        assert_eq!(
            myanmar.year_type(),
            year_type_from_code(row.year_type),
            "row={row:?}"
        );
        assert_eq!(
            myanmar.moon_phase(),
            month_phase_from_code(row.moon_phase),
            "row={row:?}"
        );
        assert_eq!(myanmar.fortnight_day(), row.fortnight_day, "row={row:?}");
        assert_eq!(
            myanmar.weekday(),
            weekday_from_code(row.weekday),
            "row={row:?}"
        );
        assert_eq!(myanmar.sasana_year(), row.sasana_year, "row={row:?}");
        // Round-trip back through Gregorian, exercising the jdn bridge
        // end-to-end without needing to expose it on the public API.
        assert_eq!(
            GregorianDate::from_myanmar(&myanmar),
            gregorian,
            "row={row:?}"
        );
    }
}

#[test]
fn myanmar_to_gregorian_matches_upstream_for_every_golden_row() {
    let rows = load_golden_vectors();
    for row in &rows {
        let month = month_from_code(row.myanmar_month);
        let myanmar = MyanmarDate::new(row.myanmar_year, month, row.myanmar_day)
            .unwrap_or_else(|e| panic!("row {row:?}: MyanmarDate::new failed: {e}"));
        let gregorian = myanmar.to_gregorian();
        assert_eq!(gregorian.year(), row.gregorian_year, "row={row:?}");
        assert_eq!(gregorian.month(), row.gregorian_month, "row={row:?}");
        assert_eq!(gregorian.day(), row.gregorian_day, "row={row:?}");
    }
}
