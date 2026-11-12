//! Shared test support: loads the golden-vector fixture generated from the
//! upstream reference implementation (see `util/generate_fixtures.js`) and
//! provides small helpers for mapping its numeric codes to this crate's
//! public enums, which don't expose their underlying codes (keeping the
//! public surface small, per `Claude.md`).

use myanmar_date::{Month, MonthPhase, Weekday, YearType};

/// One row of `tests/fixtures/golden_vectors.csv`.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // not every field is read by every test that loads rows
pub struct GoldenRow {
    pub gregorian_year: i32,
    pub gregorian_month: u8,
    pub gregorian_day: u8,
    pub jdn: i64,
    pub myanmar_year: i32,
    pub myanmar_month: i32,
    pub myanmar_day: u8,
    pub year_type: i32,
    pub moon_phase: i32,
    pub fortnight_day: u8,
    pub weekday: i64,
    pub sasana_year: i32,
}

/// Loads every row of the golden-vector fixture.
///
/// # Panics
///
/// Panics if the fixture is missing or malformed — that's a test-harness
/// setup problem, not something a library caller can hit, so a panic (rather
/// than a `Result`) is appropriate here.
#[allow(dead_code)] // used by some, but not all, test binaries
pub fn load_golden_vectors() -> Vec<GoldenRow> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/golden_vectors.csv"
    );
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!("failed to read {path}: {e}; run `node util/generate_fixtures.js` first")
    });
    content
        .lines()
        .skip(1) // header
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut f = line.split(',');
            let mut next = || f.next().unwrap_or_else(|| panic!("malformed row: {line}"));
            GoldenRow {
                gregorian_year: next().parse().unwrap(),
                gregorian_month: next().parse().unwrap(),
                gregorian_day: next().parse().unwrap(),
                jdn: next().parse().unwrap(),
                myanmar_year: next().parse().unwrap(),
                myanmar_month: next().parse().unwrap(),
                myanmar_day: next().parse().unwrap(),
                year_type: next().parse().unwrap(),
                moon_phase: next().parse().unwrap(),
                fortnight_day: next().parse().unwrap(),
                weekday: next().parse().unwrap(),
                sasana_year: next().parse().unwrap(),
            }
        })
        .collect()
}

/// Maps mmcal's numeric month code (`0..=14`) to this crate's [`Month`].
#[allow(dead_code)]
pub fn month_from_code(code: i32) -> Month {
    match code {
        0 => Month::FirstWaso,
        1 => Month::Tagu,
        2 => Month::Kason,
        3 => Month::Nayon,
        4 => Month::SecondWaso,
        5 => Month::Wagaung,
        6 => Month::Tawthalin,
        7 => Month::Thadingyut,
        8 => Month::Tazaungmon,
        9 => Month::Nadaw,
        10 => Month::Pyatho,
        11 => Month::Tabodwe,
        12 => Month::Tabaung,
        13 => Month::LateTagu,
        14 => Month::LateKason,
        other => panic!("not a valid mmcal month code: {other}"),
    }
}

/// Maps mmcal's numeric year-type code (`0..=2`) to this crate's [`YearType`].
#[allow(dead_code)]
pub fn year_type_from_code(code: i32) -> YearType {
    match code {
        0 => YearType::Common,
        1 => YearType::LittleWatat,
        2 => YearType::BigWatat,
        other => panic!("not a valid mmcal year-type code: {other}"),
    }
}

/// Maps mmcal's numeric moon-phase code (`0..=3`) to this crate's
/// [`MonthPhase`].
#[allow(dead_code)]
pub fn month_phase_from_code(code: i32) -> MonthPhase {
    match code {
        0 => MonthPhase::Waxing,
        1 => MonthPhase::Full,
        2 => MonthPhase::Waning,
        3 => MonthPhase::New,
        other => panic!("not a valid mmcal moon-phase code: {other}"),
    }
}

/// Maps mmcal's numeric weekday code (`0..=6`, `[0=Sat,...,6=Fri]`) to this
/// crate's [`Weekday`].
#[allow(dead_code)]
pub fn weekday_from_code(code: i64) -> Weekday {
    match code {
        0 => Weekday::Sat,
        1 => Weekday::Sun,
        2 => Weekday::Mon,
        3 => Weekday::Tue,
        4 => Weekday::Wed,
        5 => Weekday::Thu,
        6 => Weekday::Fri,
        other => panic!("not a valid mmcal weekday code: {other}"),
    }
}
