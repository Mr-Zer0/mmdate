//! Shared test support: loads the golden-vector fixture generated from the
//! upstream reference implementation (see `util/generate_fixtures.js`) and
//! provides small helpers for mapping its numeric codes to this crate's
//! public enums, which don't expose their underlying codes (keeping the
//! public surface small, per `Claude.md`).

use myanmar_date::{
    AstroDay, Mahabote, Month, MonthPhase, NagaDirection, Nakhat, Pyathada, Sabbath, Weekday,
    YearName, YearType,
};

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
    pub year_name: i32,
    pub nagahle: i32,
    pub mahabote: i32,
    pub nakhat: i32,
    pub sabbath: i32,
    pub yatyaza: i32,
    pub pyathada: i32,
    pub thamanyo: i32,
    pub amyeittasote: i32,
    pub warameittugyi: i32,
    pub warameittunge: i32,
    pub yatpote: i32,
    pub thamaphyu: i32,
    pub nagapor: i32,
    pub yatyotema: i32,
    pub mahayatkyan: i32,
    pub shanyat: i32,
    pub thingyan_valid: bool,
    pub akya_jdn: Option<i64>,
    pub atat_jdn: Option<i64>,
    pub new_year_jdn: Option<i64>,
    pub akya_time: Option<f64>,
    pub atat_time: Option<f64>,
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
            // A plain `fn`, not a closure, so it borrows `f` fresh on each
            // call instead of holding a long-lived capture of it (which
            // would conflict with the other `next*` helpers below also
            // needing to borrow `f` mutably within the same struct literal).
            fn next<'a>(f: &mut std::str::Split<'a, char>, line: &str) -> &'a str {
                f.next().unwrap_or_else(|| panic!("malformed row: {line}"))
            }
            fn next_opt_i64(f: &mut std::str::Split<'_, char>, line: &str) -> Option<i64> {
                let s = next(f, line);
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            }
            fn next_opt_f64(f: &mut std::str::Split<'_, char>, line: &str) -> Option<f64> {
                let s = next(f, line);
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            }
            GoldenRow {
                gregorian_year: next(&mut f, line).parse().unwrap(),
                gregorian_month: next(&mut f, line).parse().unwrap(),
                gregorian_day: next(&mut f, line).parse().unwrap(),
                jdn: next(&mut f, line).parse().unwrap(),
                myanmar_year: next(&mut f, line).parse().unwrap(),
                myanmar_month: next(&mut f, line).parse().unwrap(),
                myanmar_day: next(&mut f, line).parse().unwrap(),
                year_type: next(&mut f, line).parse().unwrap(),
                moon_phase: next(&mut f, line).parse().unwrap(),
                fortnight_day: next(&mut f, line).parse().unwrap(),
                weekday: next(&mut f, line).parse().unwrap(),
                sasana_year: next(&mut f, line).parse().unwrap(),
                year_name: next(&mut f, line).parse().unwrap(),
                nagahle: next(&mut f, line).parse().unwrap(),
                mahabote: next(&mut f, line).parse().unwrap(),
                nakhat: next(&mut f, line).parse().unwrap(),
                sabbath: next(&mut f, line).parse().unwrap(),
                yatyaza: next(&mut f, line).parse().unwrap(),
                pyathada: next(&mut f, line).parse().unwrap(),
                thamanyo: next(&mut f, line).parse().unwrap(),
                amyeittasote: next(&mut f, line).parse().unwrap(),
                warameittugyi: next(&mut f, line).parse().unwrap(),
                warameittunge: next(&mut f, line).parse().unwrap(),
                yatpote: next(&mut f, line).parse().unwrap(),
                thamaphyu: next(&mut f, line).parse().unwrap(),
                nagapor: next(&mut f, line).parse().unwrap(),
                yatyotema: next(&mut f, line).parse().unwrap(),
                mahayatkyan: next(&mut f, line).parse().unwrap(),
                shanyat: next(&mut f, line).parse().unwrap(),
                thingyan_valid: next(&mut f, line).parse::<i32>().unwrap() != 0,
                akya_jdn: next_opt_i64(&mut f, line),
                atat_jdn: next_opt_i64(&mut f, line),
                new_year_jdn: next_opt_i64(&mut f, line),
                akya_time: next_opt_f64(&mut f, line),
                atat_time: next_opt_f64(&mut f, line),
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

/// Maps mmcal's numeric year-name index (`my % 12`) to this crate's
/// [`YearName`].
#[allow(dead_code)]
pub fn year_name_from_code(code: i32) -> YearName {
    match code {
        0 => YearName::Hpusha,
        1 => YearName::Magha,
        2 => YearName::Phalguni,
        3 => YearName::Chitra,
        4 => YearName::Visakha,
        5 => YearName::Jyeshtha,
        6 => YearName::Ashadha,
        7 => YearName::Sravana,
        8 => YearName::Bhadrapaha,
        9 => YearName::Asvini,
        10 => YearName::Krittika,
        11 => YearName::Mrigasiras,
        other => panic!("not a valid mmcal year-name code: {other}"),
    }
}

/// Maps mmcal's numeric `cal_nagahle` code (`0..=3`) to this crate's
/// [`NagaDirection`].
#[allow(dead_code)]
pub fn naga_direction_from_code(code: i32) -> NagaDirection {
    match code {
        0 => NagaDirection::West,
        1 => NagaDirection::North,
        2 => NagaDirection::East,
        3 => NagaDirection::South,
        other => panic!("not a valid mmcal nagahle code: {other}"),
    }
}

/// Maps mmcal's numeric `cal_mahabote` code (`0..=6`, already non-negative
/// for every `my` this crate supports) to this crate's [`Mahabote`].
#[allow(dead_code)]
pub fn mahabote_from_code(code: i32) -> Mahabote {
    match code.rem_euclid(7) {
        0 => Mahabote::Binga,
        1 => Mahabote::Atun,
        2 => Mahabote::Yaza,
        3 => Mahabote::Adipati,
        4 => Mahabote::Marana,
        5 => Mahabote::Thike,
        6 => Mahabote::Puti,
        other => panic!("not a valid mmcal mahabote code: {other}"),
    }
}

/// Maps mmcal's numeric `cal_nakhat` code (`0..=2`) to this crate's
/// [`Nakhat`].
#[allow(dead_code)]
pub fn nakhat_from_code(code: i32) -> Nakhat {
    match code.rem_euclid(3) {
        0 => Nakhat::Ogre,
        1 => Nakhat::Elf,
        2 => Nakhat::Human,
        other => panic!("not a valid mmcal nakhat code: {other}"),
    }
}

/// Maps mmcal's numeric `cal_sabbath` code (`0=none, 1=sabbath,
/// 2=sabbath eve`) to this crate's `Option<Sabbath>`.
#[allow(dead_code)]
pub fn sabbath_from_code(code: i32) -> Option<Sabbath> {
    match code {
        0 => None,
        1 => Some(Sabbath::Sabbath),
        2 => Some(Sabbath::SabbathEve),
        other => panic!("not a valid mmcal sabbath code: {other}"),
    }
}

/// Maps mmcal's numeric `cal_pyathada` code (`0=none, 1=pyathada,
/// 2=afternoon`) to this crate's `Option<Pyathada>`.
#[allow(dead_code)]
pub fn pyathada_from_code(code: i32) -> Option<Pyathada> {
    match code {
        0 => None,
        1 => Some(Pyathada::Pyathada),
        2 => Some(Pyathada::Afternoon),
        other => panic!("not a valid mmcal pyathada code: {other}"),
    }
}

/// The ten `cal_astro` flags of a [`GoldenRow`], as `(flag, AstroDay)` pairs,
/// for iterating over all of them generically in golden-vector tests.
#[allow(dead_code)]
pub fn astro_day_flags(row: &GoldenRow) -> [(bool, AstroDay); 10] {
    [
        (row.thamanyo != 0, AstroDay::Thamanyo),
        (row.amyeittasote != 0, AstroDay::Amyeittasote),
        (row.warameittugyi != 0, AstroDay::Warameittugyi),
        (row.warameittunge != 0, AstroDay::Warameittunge),
        (row.yatpote != 0, AstroDay::Yatpote),
        (row.thamaphyu != 0, AstroDay::Thamaphyu),
        (row.nagapor != 0, AstroDay::Nagapor),
        (row.yatyotema != 0, AstroDay::Yatyotema),
        (row.mahayatkyan != 0, AstroDay::Mahayatkyan),
        (row.shanyat != 0, AstroDay::Shanyat),
    ]
}
