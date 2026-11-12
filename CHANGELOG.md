# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (Phase 2: astrological days and Thingyan)

- `MyanmarDate` getters for every astrological day/property that is a
  deterministic function of the date, ported from `ceMmDateTime.js`
  lines 1056-1325: `sabbath()`, `yatyaza()`, `pyathada()`, `thamanyo()`,
  `amyeittasote()`, `warameittugyi()`, `warameittunge()`, `yatpote()`,
  `thamaphyu()`, `nagapor()`, `yatyotema()`, `mahayatkyan()`, `shanyat()`,
  `nagahle()`, `mahabote()`, `nakhat()`, `year_name()`, and `astro_days()`
  (the set of `cal_astro`'s ten astrological days for the date, as an
  allocation-free [`AstroDays`] bitset).
- New enums: `Sabbath`, `Pyathada`, `NagaDirection`, `Mahabote`, `Nakhat`,
  `YearName`, `AstroDay`, `AstroDays`/`AstroDaysIter`. English names always
  available via `english_name()`; `YearName::burmese_name()` under the
  `burmese` feature (purely additive).
- `Thingyan`: the astronomically computed Thingyan (Myanmar New Year)
  transition points (`Thingyan::for_year`, `MyanmarDate::thingyan`), with
  `akya`/`atat`/`new_year` Julian-Day-Number and Gregorian-date accessors and
  a `ThingyanPhase` classifier (`Akyo`/`Akya`/`Akyat`/`Atat`). Faithfully
  reproduces upstream's `cal_holiday` astronomical calculation (including its
  asymmetric use of the raw vs. month-type-adjusted year for the era
  cutover), without adopting any of its holiday *labeling* — this crate never
  asserts that a day is a holiday, per `Claude.md`.
- Golden-vector coverage extended: every row of
  `tests/fixtures/golden_vectors.csv` now also carries the upstream value for
  every astrological day/property above and (where the Myanmar year is at or
  after ME 1100) the Thingyan transition points, checked in
  `tests/golden.rs`.
- Anchor reused from `cpp/test/cedate/cedate.cpp`: 2024-01-01 (ME 1385
  Nadaw 20) has year name Jyeshtha, naga facing South, mahabote Marana, and
  no `cal_astro` days — plus a hand-checked Thingyan pin for ME 1386 (2024's
  Thingyan, April 13-16, New Year's Day April 17).

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

### Out of scope

Holiday labeling (whether a day is a public, government, or religious
holiday), the chronicle of historical events, and time-of-day/timezone
handling are out of scope for this crate entirely — see `Claude.md`.
