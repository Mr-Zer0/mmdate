# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (Phase 1: Gregorian <-> Myanmar conversion)

- `GregorianDate`: a validated proleptic-Gregorian calendar date
  (`GregorianDate::new`, `year`/`month`/`day`/`weekday` getters,
  `from_myanmar`).
- `MyanmarDate`: a validated Myanmar (Burmese) lunisolar calendar date
  (`MyanmarDate::from_gregorian`, `MyanmarDate::new`, `to_gregorian`, and
  `year`/`month`/`day`/`year_type`/`moon_phase`/`fortnight_day`/
  `month_length`/`sasana_year`/`weekday` getters).
- `Month`: the 15 Myanmar calendar months, preserving mmcal's native
  non-sequential numbering (`FirstWaso = 0`, ..., `LateKason = 14`).
- `YearType`: `Common` / `LittleWatat` / `BigWatat`.
- `MonthPhase`: `Waxing` / `Full` / `Waning` / `New`.
- `Weekday`: `Sat`..`Fri` (mmcal's native ordering), with
  `english_name()`/`burmese_name()` (the latter behind the `burmese`
  feature).
- `Error`: a `no_std`-compatible, `core::error::Error`-implementing error
  type (`OutOfRange`, `InvalidDate`). Out-of-range or invalid input always
  returns `Err`, never a wrong date.
- `chrono` feature (default-on): `From`/`TryFrom` bridges between
  `chrono::NaiveDate` and `GregorianDate`/`MyanmarDate`.
- `burmese` feature: Burmese-script names, purely additive.
- Supported range: Gregorian years 640–9999 CE, validated against the
  upstream reference implementation (`yan9a/mmcal`) across a dense recent
  range, every historical era boundary, and every documented watat/full-moon
  exception year (see `tests/fixtures/golden_vectors.csv` and
  `util/generate_fixtures.js`).
- `#![no_std]` core with zero required dependencies.

### Out of scope for this release

Astrological days (Sabbath, Yatyaza, Pyathada, Thamanyo, Amyeittasote,
Warameittugyi/nge, Yatyotema, Mahayatkyan, Nagapor, Nagahle, Mahabote, Nakhat,
year name) and the Thingyan calculation are planned for a follow-up release.
Holiday labeling, the chronicle of historical events, and time-of-day/
timezone handling are out of scope for this crate entirely.
