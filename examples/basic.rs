//! Basic usage: convert a Gregorian date to Myanmar and back.
//!
//! Run with: `cargo run --example basic`

use myanmar_date::{GregorianDate, MyanmarDate};

fn main() -> Result<(), myanmar_date::Error> {
    let gregorian = GregorianDate::new(2024, 1, 1)?;
    let myanmar = MyanmarDate::from_gregorian(gregorian)?;

    println!(
        "{gregorian:?} = Myanmar year {}, {:?} {}, {:?}, fortnight day {}, Sasana year {}",
        myanmar.year(),
        myanmar.month(),
        myanmar.day(),
        myanmar.moon_phase(),
        myanmar.fortnight_day(),
        myanmar.sasana_year(),
    );

    assert_eq!(myanmar.year(), 1385);
    assert_eq!(myanmar.sasana_year(), 2567);
    assert_eq!(myanmar.to_gregorian(), gregorian);

    #[cfg(feature = "chrono")]
    {
        let today = chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let myanmar_from_chrono = MyanmarDate::try_from(today)?;
        assert_eq!(myanmar_from_chrono, myanmar);
        println!(
            "chrono round-trip ok: {:?}",
            chrono::NaiveDate::from(myanmar_from_chrono)
        );
    }

    Ok(())
}
