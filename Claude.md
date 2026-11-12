# CLAUDE.md

Project context for Claude Code. This file is **project description and working
guidelines only**. It deliberately contains no implementation detail — no
algorithms, function signatures, module layout, constants, or code mechanics.
Those belong in the code and its tests, not here.

---

## Project description

### What we are building

A small, dependency-light **Rust library** (for publication on crates.io) that
converts between **Gregorian (English) calendar dates** and **Burmese / Myanmar
calendar dates**, and computes the **astrological days** that are a deterministic
function of the date.

**Crate name:** `myanmar-date` — available on crates.io at time of writing;
re-verify immediately before the first publish, and claim it by publishing the
first real version rather than squatting an empty placeholder.

### Scope

**In scope**

- Gregorian → Myanmar conversion (the primary direction).
- Myanmar → Gregorian conversion.
- Astrological days that are deterministic functions of the date: Sabbath and
  Sabbath-eve, Yatyaza, Pyathada, Thamanyo, Amyeittasote, Warameittugyi,
  Warameittunge, Yatyotema, Mahayatkyan, Nagapor, Nagahle (direction of the
  naga), Mahabote, Nakhat, and the year name — plus the astronomically-computed
  Thingyan points.

**Out of scope**

- **Holiday definitions.** Whether a given day is a public, government, or
  religious *holiday* — along with observance rules, substitute days, and
  regional or governmental variation — is policy that changes over time and is
  the **consumer's** responsibility, not the library's. We expose the calendar
  and astrological facts; we never label a day "a holiday." (Note the boundary:
  we still *compute* astronomically-defined events such as Thingyan; we simply do
  not assert that any day is a holiday.)
- Time-of-day / timezone handling and the chronicle of historical events, unless
  explicitly requested later.

### Shape of the library (high-level decisions)

- A **pure, allocation-free core** that is `no_std`-compatible and has **zero
  required dependencies**.
- **chrono integration** is the primary ergonomic API, offered as a default-on
  feature and kept `no_std`-friendly.
- **English names** are always available; **Burmese-script names** are an optional
  feature.

These describe the deliverable. How they are wired up is an implementation matter
for the code, not for this file.

### Reference & licensing

The calculation logic is ported from **mmcal** by Yan Naing Aye:
<https://github.com/yan9a/mmcal> (the readable references are
`javascript/ceMmDateTime.js` and the `cpp/include/` headers). Algorithm write-up:
<http://cool-emerald.blogspot.com/2013/06/algorithm-program-and-calculation-of.html>

Obligations — **non-negotiable**:

- mmcal is **MIT licensed, © 2018 Yan Naing Aye**. Preserve the copyright notice
  and license text.
- Add a `THIRD-PARTY-NOTICES.md` that reproduces mmcal's MIT license and credits
  the underlying "mc1500 / Modern Myanmar Calendrical Calculations" algorithm and
  the cool-emerald write-up; reference it from the README.
- State our crate's own license explicitly in `Cargo.toml` (confirm the choice
  with the maintainer).
- Re-express the algorithm as idiomatic Rust rather than copying source text, and
  carry a short attribution comment on any file that is a direct port.

### Domain background (conceptual)

The Myanmar calendar is **lunisolar**. Conversions pivot on the **Julian Day
Number (JDN)** as a neutral intermediate. The Myanmar year relates to the Sasana
year by a fixed offset. Some years are **watat** (leap years carrying an
intercalary month) and some of those are **big watat** (with one extra day).
Dates are expressed with a month, a **moon phase** (waxing, full moon, waning,
new moon), and a **fortnight day**. The watat rules depend on historical eras and
a set of empirically-corrected exception years — these must be reproduced
faithfully (see guidelines). Astrological days are further deterministic
functions of the date and weekday.

---

## Working guidelines

### Test-first (TDD) — mandatory

Follow **red → green → refactor** and never skip a phase.

1. **RED:** write a failing test that specifies the behavior, and confirm it
   fails for the expected reason before writing any implementation.
2. **GREEN:** write the minimum code to make it pass.
3. **REFACTOR:** clean up with the tests green; re-run tests, clippy, and fmt.

No implementation code exists before a failing test that calls for it. When a
requirement is ambiguous, encode the chosen interpretation as a test and flag the
assumption. Cover unhappy paths deliberately (boundaries, invalid input, error
branches).

### Correctness

- The bar is **agreement with mmcal across its supported range.**
- Validate against **ground-truth vectors generated from the upstream
  implementation**, plus a few **hand-verified anchor dates** and **round-trip
  property tests**.
- **Faithfully reproduce the upstream empirical constants and exception years.**
  Do not re-derive, round, or "simplify" them; pin each exception year with a
  test.
- Out-of-range inputs **return an error, never a wrong date.**

### Memory & resource discipline

- Conversions and astro calculations are pure arithmetic: **no heap allocation**
  on these paths, and value types are `Copy`.
- Prefer borrowing over owning; justify any allocation or clone. Names are
  `&'static str`, not owned strings.

### Error handling

- Use a **library-grade error type**; never `anyhow` in the public API.
- No `.unwrap()` / `.expect()` on caller-influenced input, and no panics for
  conditions a caller can reach. The error type must compile without `std`.

### Public API hygiene (it ships to crates.io)

- Keep the public surface small and hard to misuse; prefer private until a need
  is proven.
- **Semver discipline:** know whether each change is breaking before making it.
- `#![deny(missing_docs)]`, with a compiling example on each public item; standard
  derives (`Debug`/`Clone`/`Copy`/`PartialEq`/`Eq`) on value types.
- **Features must be purely additive** — enabling one never changes or removes
  existing behavior.

### Lints

- `#![forbid(unsafe_code)]` — this crate is pure arithmetic and needs no `unsafe`.
- `#![deny(missing_docs, unsafe_op_in_unsafe_fn)]`,
  `#![warn(clippy::all, clippy::pedantic, rust_2018_idioms, unreachable_pub)]`.

### Commands (run before any work is "done")

```bash
cargo test --all-features
cargo test --no-default-features        # core must pass without chrono/std
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo doc --no-deps --all-features
```

For a release also run `cargo semver-checks`, `cargo deny check`,
`cargo package --list`, and `cargo publish --dry-run`. **Never run
`cargo publish` — leave the final publish to the human.**

### Attribution

Every file that is a direct port of a specific upstream routine carries a short
comment: `// Ported from yan9a/mmcal (MIT, © 2018 Yan Naing Aye).`