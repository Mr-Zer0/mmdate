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
    astro_day_flags, load_golden_vectors, mahabote_from_code, month_from_code,
    month_phase_from_code, naga_direction_from_code, nakhat_from_code, pyathada_from_code,
    sabbath_from_code, weekday_from_code, year_name_from_code, year_type_from_code,
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
fn astro_days_match_upstream_for_every_golden_row() {
    let rows = load_golden_vectors();
    for row in &rows {
        let gregorian =
            GregorianDate::new(row.gregorian_year, row.gregorian_month, row.gregorian_day)
                .unwrap_or_else(|e| panic!("row {row:?}: GregorianDate::new failed: {e}"));
        let myanmar = MyanmarDate::from_gregorian(gregorian)
            .unwrap_or_else(|e| panic!("row {row:?}: from_gregorian failed: {e}"));

        assert_eq!(
            myanmar.year_name(),
            year_name_from_code(row.year_name),
            "row={row:?}"
        );
        assert_eq!(
            myanmar.nagahle(),
            naga_direction_from_code(row.nagahle),
            "row={row:?}"
        );
        assert_eq!(
            myanmar.mahabote(),
            mahabote_from_code(row.mahabote),
            "row={row:?}"
        );
        assert_eq!(
            myanmar.nakhat(),
            nakhat_from_code(row.nakhat),
            "row={row:?}"
        );
        assert_eq!(
            myanmar.sabbath(),
            sabbath_from_code(row.sabbath),
            "row={row:?}"
        );
        assert_eq!(myanmar.yatyaza(), row.yatyaza != 0, "row={row:?}");
        assert_eq!(
            myanmar.pyathada(),
            pyathada_from_code(row.pyathada),
            "row={row:?}"
        );

        let days = myanmar.astro_days();
        for (expected, day) in astro_day_flags(row) {
            assert_eq!(days.contains(day), expected, "row={row:?} day={day:?}");
        }
    }
}

#[test]
fn thingyan_matches_upstream_for_every_golden_row_with_a_valid_thingyan_year() {
    let rows = load_golden_vectors();
    let mut checked = 0usize;
    for row in &rows {
        if !row.thingyan_valid {
            // Confirm our own implementation agrees that this year is out of
            // range too, rather than silently skipping the row.
            let gregorian =
                GregorianDate::new(row.gregorian_year, row.gregorian_month, row.gregorian_day)
                    .unwrap();
            let myanmar = MyanmarDate::from_gregorian(gregorian).unwrap();
            assert!(
                myanmar.thingyan().is_err(),
                "row={row:?}: expected Thingyan::for_year to be out of range"
            );
            continue;
        }
        let gregorian =
            GregorianDate::new(row.gregorian_year, row.gregorian_month, row.gregorian_day).unwrap();
        let myanmar = MyanmarDate::from_gregorian(gregorian).unwrap();
        let t = myanmar
            .thingyan()
            .unwrap_or_else(|e| panic!("row {row:?}: thingyan() failed: {e}"));
        assert_eq!(t.akya_jdn(), row.akya_jdn.unwrap(), "row={row:?}");
        assert_eq!(t.atat_jdn(), row.atat_jdn.unwrap(), "row={row:?}");
        assert_eq!(t.new_year_jdn(), row.new_year_jdn.unwrap(), "row={row:?}");
        if let Some(expected) = row.akya_time {
            assert!(
                (t.akya_time() - expected).abs() < 1e-6,
                "row={row:?}: akya_time {} != expected {expected}",
                t.akya_time()
            );
        }
        if let Some(expected) = row.atat_time {
            assert!(
                (t.atat_time() - expected).abs() < 1e-6,
                "row={row:?}: atat_time {} != expected {expected}",
                t.atat_time()
            );
        }
        checked += 1;
    }
    assert!(
        checked > 1_000,
        "expected a substantial number of rows with a valid Thingyan year, got {checked}"
    );
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
