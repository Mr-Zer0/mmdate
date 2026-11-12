//! Hand-verified anchor dates.
//!
//! These assertions are reproduced from
//! `/Users/yannaing/experiment/mmcal/cpp/test/cedate/cedate.cpp` (the
//! upstream C++ regression test). Its astrological-day and holiday
//! assertions are out of scope for Phase 1 and are intentionally not ported
//! here.

use myanmar_date::{GregorianDate, Month, MonthPhase, MyanmarDate, Weekday};

#[test]
fn gregorian_2024_01_01_converts_to_myanmar_1385_nadaw_20() {
    let gregorian = GregorianDate::new(2024, 1, 1).expect("2024-01-01 is in the supported range");
    let myanmar = MyanmarDate::from_gregorian(gregorian).expect("2024-01-01 converts");

    // "western date round-trips to 2024-01-01"
    assert_eq!(GregorianDate::from_myanmar(&myanmar), gregorian);
    assert_eq!(myanmar.to_gregorian(), gregorian);

    // "2024-01-01 is a Monday"
    assert_eq!(gregorian.weekday(), Weekday::Mon);
    assert_eq!(myanmar.weekday(), Weekday::Mon);

    // "Myanmar date is 1385 Nadaw 20"
    assert_eq!(myanmar.year(), 1385);
    assert_eq!(myanmar.month(), Month::Nadaw);
    assert_eq!(myanmar.day(), 20);

    // "moon phase is waning, fortnight day 5"
    assert_eq!(myanmar.moon_phase(), MonthPhase::Waning);
    assert_eq!(myanmar.fortnight_day(), 5);

    // "Sasana year is 2567"
    assert_eq!(myanmar.sasana_year(), 2567);
}

#[test]
fn myanmar_1385_nadaw_20_converts_to_gregorian_2024_01_01() {
    // The reverse direction, constructing the Myanmar date directly.
    let myanmar =
        MyanmarDate::new(1385, Month::Nadaw, 20).expect("1385 Nadaw 20 is a valid Myanmar date");
    let gregorian = myanmar.to_gregorian();
    assert_eq!(gregorian, GregorianDate::new(2024, 1, 1).unwrap());
}
