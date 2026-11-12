#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs, unsafe_op_in_unsafe_fn)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms, unreachable_pub)]
//! Conversion between Gregorian and Myanmar (Burmese) lunisolar calendar
//! dates, ported from [yan9a/mmcal](https://github.com/yan9a/mmcal) (MIT,
//! © 2018 Yan Naing Aye).
//!
//! # Scope (Phase 1)
//!
//! This release covers Gregorian <-> Myanmar date conversion and the date
//! value types ([`GregorianDate`], [`MyanmarDate`], [`Month`], [`YearType`],
//! [`MonthPhase`], [`Weekday`]). Astrological days (Sabbath, Yatyaza,
//! Mahabote, ...) and the Thingyan calculation are planned for a later
//! release and are not part of this crate yet. This crate also never labels
//! a day a "holiday" — that is policy, not calendrical fact, and is left to
//! the consumer.
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

#[cfg(feature = "chrono")]
mod chrono_compat;
mod consts;
mod error;
mod gregorian;
mod julian;
mod math;
mod myanmar;
mod names;
mod weekday;

pub use error::Error;
pub use gregorian::{GregorianDate, MAX_YEAR, MIN_YEAR};
pub use myanmar::{Month, MonthPhase, MyanmarDate, YearType};
pub use weekday::Weekday;
