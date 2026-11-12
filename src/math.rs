//! Tiny `no_std` floating-point helpers.
//!
//! `core` does not provide `f64::floor`/`f64::round` (they live in `std` via the
//! platform's `libm`), so to keep this crate `no_std` with zero required
//! dependencies we implement the handful of operations the port actually needs
//! using pure integer/float arithmetic. `f64`'s `%` operator *is* available in
//! `core` and already matches C's `fmod`/JS's `%` (truncated, sign-of-dividend),
//! so no helper is needed for that.
//
// Memory note: these are pure, allocation-free scalar functions operating on
// `Copy` f64 values — nothing to free, no ownership questions.

/// `floor` for `f64` without `std`/`libm`.
///
/// Casting `f64 -> i64 -> f64` truncates toward zero; when the input was
/// negative and had a fractional part, truncation rounded *up* (toward zero),
/// so we subtract one to get the true floor. This mirrors JS's `Math.floor`.
///
/// Values must fit in `i64` range, which holds for every quantity this crate
/// computes (Julian day numbers and calendar years are many orders of
/// magnitude smaller than `i64::MAX`).
// The `f64 -> i64 -> f64` round trip is the deliberate mechanism this
// function implements (there is no `f64::floor` to call without `std`), so
// the truncation/precision-loss lints have nothing meaningful to add here.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub(crate) fn floor(x: f64) -> f64 {
    let truncated = x as i64 as f64;
    if x < truncated {
        truncated - 1.0
    } else {
        truncated
    }
}

/// `round` for `f64` without `std`/`libm`, matching JS `Math.round` semantics:
/// halfway values round toward positive infinity (NOT banker's rounding, and
/// NOT round-half-away-from-zero).
pub(crate) fn round(x: f64) -> f64 {
    floor(x + 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_of_positive_integer_is_itself() {
        assert_eq!(floor(3.0), 3.0);
    }

    #[test]
    fn floor_of_positive_fraction_truncates_down() {
        assert_eq!(floor(3.7), 3.0);
    }

    #[test]
    fn floor_of_negative_integer_is_itself() {
        assert_eq!(floor(-3.0), -3.0);
    }

    #[test]
    fn floor_of_negative_fraction_rounds_toward_negative_infinity() {
        // JS: Math.floor(-3.7) === -4
        assert_eq!(floor(-3.7), -4.0);
    }

    #[test]
    fn floor_of_zero_is_zero() {
        assert_eq!(floor(0.0), 0.0);
    }

    #[test]
    fn round_half_rounds_toward_positive_infinity() {
        // JS: Math.round(2.5) === 3, Math.round(-2.5) === -2 (NOT banker's rounding)
        assert_eq!(round(2.5), 3.0);
        assert_eq!(round(-2.5), -2.0);
    }

    #[test]
    fn round_below_half_rounds_down() {
        assert_eq!(round(2.4), 2.0);
        assert_eq!(round(-2.4), -2.0);
    }

    #[test]
    fn round_above_half_rounds_up() {
        assert_eq!(round(2.6), 3.0);
        assert_eq!(round(-2.6), -3.0);
    }

    #[test]
    fn round_of_integer_is_itself() {
        assert_eq!(round(5.0), 5.0);
        assert_eq!(round(-5.0), -5.0);
    }
}
