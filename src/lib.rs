#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs, unsafe_op_in_unsafe_fn)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms, unreachable_pub)]
//! Conversion between Gregorian and Myanmar (Burmese) lunisolar calendar
//! dates, ported from [yan9a/mmcal](https://github.com/yan9a/mmcal) (MIT,
//! © 2018 Yan Naing Aye).
//!
//! # Scope
//!
//! This crate covers Gregorian <-> Myanmar date conversion and the date value
//! types ([`GregorianDate`], [`MyanmarDate`], [`Month`], [`YearType`],
//! [`MonthPhase`], [`Weekday`]), plus the astrological days and year
//! properties that are a deterministic function of the date: sabbath/
//! sabbath-eve, yatyaza, pyathada, the ten `cal_astro` days ([`AstroDay`]),
//! the naga's direction ([`NagaDirection`]), mahabote ([`Mahabote`]), nakhat
//! ([`Nakhat`]), the traditional year name ([`YearName`]), and the
//! astronomically computed Thingyan transition points ([`Thingyan`]). This
//! crate never labels a day a "holiday" — which days are public, government,
//! or religious holidays is policy, not calendrical fact, and is left to the
//! consumer (see [`MyanmarDate::astro_days`] and [`Thingyan`] for the
//! calendrical facts themselves).
//!
//! # Supported range
//!
//! Gregorian years [`MIN_YEAR`]..=[`MAX_YEAR`] (640 CE to 9999 CE). Every
//! conversion in this range has been checked against the upstream reference
//! implementation (see `tests/golden.rs` and `tests/fixtures/`); dates
//! outside it return [`Error::OutOfRange`] rather than a wrong answer.
//!
//! # Quick start
//!
//! ```
//! use myanmar_date::{GregorianDate, MyanmarDate};
//!
//! let gregorian = GregorianDate::new(2024, 1, 1)?;
//! let myanmar = MyanmarDate::from_gregorian(gregorian)?;
//!
//! assert_eq!(myanmar.year(), 1385);
//! assert_eq!(myanmar.sasana_year(), 2567);
//! assert_eq!(myanmar.to_gregorian(), gregorian);
//! # Ok::<(), myanmar_date::Error>(())
//! ```
//!
//! With the default `chrono` feature, [`chrono::NaiveDate`] converts
//! directly:
//!
//! ```
//! # #[cfg(feature = "chrono")]
//! # fn main() -> Result<(), myanmar_date::Error> {
//! use chrono::NaiveDate;
//! use myanmar_date::MyanmarDate;
//!
//! let today = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
//! let myanmar = MyanmarDate::try_from(today)?;
//! assert_eq!(myanmar.year(), 1385);
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "chrono"))]
//! # fn main() {}
//! ```
//!
//! # Astrological days and Thingyan
//!
//! ```
//! use myanmar_date::{GregorianDate, Mahabote, MyanmarDate, YearName};
//!
//! let g = GregorianDate::new(2024, 1, 1)?;
//! let m = MyanmarDate::from_gregorian(g)?;
//!
//! assert_eq!(m.year_name(), YearName::Jyeshtha);
//! assert_eq!(m.mahabote(), Mahabote::Marana);
//! assert!(m.astro_days().is_empty()); // no cal_astro days on this date
//!
//! let t = m.thingyan()?; // the Thingyan at the end of ME 1385
//! assert_eq!(t.new_year_date(), GregorianDate::new(2023, 4, 17)?);
//! # Ok::<(), myanmar_date::Error>(())
//! ```
//!
//! # Memory and dependencies
//!
//! The core is `#![no_std]` with **zero required dependencies**; every
//! conversion is pure, allocation-free arithmetic over `Copy` value types.
//! `chrono` integration is on by default (and itself used in `no_std` mode);
//! disable default features for a strictly dependency-free build.
//!
//! # Third-party notices
//!
//! The calculation logic is ported from mmcal (MIT, © 2018 Yan Naing Aye);
//! see `THIRD-PARTY-NOTICES.md` in the repository for the full license text
//! and attribution.

// `#![no_std]` disables the implicit std prelude crate-wide, including in
// `#[cfg(test)]` modules (they're compiled as part of this same crate). Our
// unit tests occasionally want `ToString`/string comparisons for convenience;
// this makes `std` available *only* in test builds without affecting the
// shipped no_std guarantee (dev/test builds are never what downstream users
// link against).
#[cfg(test)]
extern crate std;

mod astro;
#[cfg(feature = "burmese")]
mod burmese;
#[cfg(feature = "chrono")]
mod chrono_compat;
mod consts;
mod error;
mod gregorian;
mod julian;
mod math;
mod myanmar;
mod names;
mod thingyan;
mod weekday;

pub use astro::{
    AstroDay, AstroDays, AstroDaysIter, Mahabote, NagaDirection, Nakhat, Pyathada, Sabbath,
    YearName,
};
#[cfg(feature = "burmese")]
pub use burmese::BurmeseDate;
pub use error::Error;
pub use gregorian::{GregorianDate, MAX_YEAR, MIN_YEAR};
pub use myanmar::{Month, MonthPhase, MyanmarDate, YearType};
pub use thingyan::{Thingyan, ThingyanPhase};
pub use weekday::Weekday;
