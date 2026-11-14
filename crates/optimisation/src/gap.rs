//! Canonical MIP gap reporting (ADR 0003, "Solver trait, outcomes and
//! numerical contract").
//!
//! For incumbent `I` and best bound `B` in the same objective units, including
//! the same offset: absolute gap `A = |I - B|` and canonical ratio
//! `R = A / max(U, |I|)`, where `U` is one unit of the objective currency in
//! model units. A missing, non-finite or wrong-side bound gives an unknown gap,
//! never zero.

/// Objective direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveSense {
    /// Minimise the objective.
    Minimise,
    /// Maximise the objective.
    Maximise,
}

/// Why a gap cannot be computed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnknownGap {
    /// No feasible incumbent.
    MissingIncumbent,
    /// No valid best bound.
    MissingBound,
    /// The incumbent, bound, currency unit or tolerance is NaN, infinite or
    /// out of range.
    NonFinite,
    /// The bound is on the wrong side of the incumbent for the objective sense.
    IncompatibleBound,
    /// The inputs are finite but the derived absolute gap or ratio overflowed.
    Overflow,
}

/// A computed or unknown MIP gap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gap {
    /// Both values are known and consistent.
    Known {
        /// `|I - B|` in objective units.
        absolute: f64,
        /// `|I - B| / max(U, |I|)`; never negative.
        ratio: f64,
    },
    /// The gap is unknown; it must not be reported as zero.
    Unknown(UnknownGap),
}

/// Computes the canonical gap.
///
/// `currency_unit` is one unit of the objective currency expressed in model
/// objective units (positive and finite). `tolerance` is the numeric tolerance
/// allowed when checking that the bound is on the correct side (non-negative
/// and finite).
#[must_use]
pub fn mip_gap(
    sense: ObjectiveSense,
    incumbent: Option<f64>,
    bound: Option<f64>,
    currency_unit: f64,
    tolerance: f64,
) -> Gap {
    let Some(incumbent) = incumbent else {
        return Gap::Unknown(UnknownGap::MissingIncumbent);
    };
    let Some(bound) = bound else {
        return Gap::Unknown(UnknownGap::MissingBound);
    };
    let valid_unit = currency_unit.is_finite() && currency_unit > 0.0;
    let valid_tolerance = tolerance.is_finite() && tolerance >= 0.0;
    if !(incumbent.is_finite() && bound.is_finite() && valid_unit && valid_tolerance) {
        return Gap::Unknown(UnknownGap::NonFinite);
    }
    let consistent = match sense {
        ObjectiveSense::Minimise => bound <= incumbent + tolerance,
        ObjectiveSense::Maximise => bound >= incumbent - tolerance,
    };
    if !consistent {
        return Gap::Unknown(UnknownGap::IncompatibleBound);
    }
    let absolute = (incumbent - bound).abs();
    let ratio = absolute / currency_unit.max(incumbent.abs());
    // R8: finite inputs can still overflow; a non-finite gap is never Known.
    if !(absolute.is_finite() && ratio.is_finite()) {
        return Gap::Unknown(UnknownGap::Overflow);
    }
    Gap::Known { absolute, ratio }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: ObjectiveSense = ObjectiveSense::Minimise;
    const MAX: ObjectiveSense = ObjectiveSense::Maximise;

    fn known(gap: Gap) -> (f64, f64) {
        match gap {
            Gap::Known { absolute, ratio } => (absolute, ratio),
            Gap::Unknown(reason) => panic!("expected known gap, got {reason:?}"),
        }
    }

    fn close(left: f64, right: f64) -> bool {
        (left - right).abs() <= 1e-12
    }

    #[test]
    fn fifty_pence_against_zero_is_absolute_not_zero_ratio() {
        let (absolute, ratio) = known(mip_gap(MIN, Some(0.5), Some(0.0), 1.0, 1e-9));
        assert!(close(absolute, 0.5));
        assert!(close(ratio, 0.5));
    }

    #[test]
    fn zero_objective_with_matching_bound_is_zero_gap() {
        assert_eq!(
            known(mip_gap(MIN, Some(0.0), Some(0.0), 1.0, 0.0)),
            (0.0, 0.0)
        );
    }

    #[test]
    fn large_objective_uses_incumbent_denominator() {
        let (absolute, ratio) = known(mip_gap(MIN, Some(1_000.0), Some(999.0), 1.0, 0.0));
        assert!(close(absolute, 1.0));
        assert!(close(ratio, 0.001));
    }

    #[test]
    fn negative_and_sign_crossing_objectives_never_give_negative_ratio() {
        let (absolute, ratio) = known(mip_gap(MIN, Some(-100.0), Some(-110.0), 1.0, 0.0));
        assert!(close(absolute, 10.0) && close(ratio, 0.1));
        let (absolute, ratio) = known(mip_gap(MIN, Some(2.0), Some(-2.0), 1.0, 0.0));
        assert!(close(absolute, 4.0) && close(ratio, 2.0));
        let (absolute, ratio) = known(mip_gap(MAX, Some(-2.0), Some(2.0), 1.0, 0.0));
        assert!(close(absolute, 4.0) && close(ratio, 2.0));
    }

    #[test]
    fn maximisation_requires_bound_at_or_above_incumbent() {
        let (absolute, _) = known(mip_gap(MAX, Some(50.0), Some(55.0), 1.0, 0.0));
        assert!(close(absolute, 5.0));
        assert_eq!(
            mip_gap(MAX, Some(50.0), Some(45.0), 1.0, 0.0),
            Gap::Unknown(UnknownGap::IncompatibleBound)
        );
    }

    #[test]
    fn wrong_side_bound_is_unknown_beyond_tolerance_only() {
        assert_eq!(
            mip_gap(MIN, Some(10.0), Some(10.5), 1.0, 0.1),
            Gap::Unknown(UnknownGap::IncompatibleBound)
        );
        assert!(matches!(
            mip_gap(MIN, Some(10.0), Some(10.05), 1.0, 0.1),
            Gap::Known { .. }
        ));
    }

    #[test]
    fn missing_or_non_finite_inputs_are_unknown_not_zero() {
        assert_eq!(
            mip_gap(MIN, None, Some(0.0), 1.0, 0.0),
            Gap::Unknown(UnknownGap::MissingIncumbent)
        );
        assert_eq!(
            mip_gap(MIN, Some(0.0), None, 1.0, 0.0),
            Gap::Unknown(UnknownGap::MissingBound)
        );
        for (incumbent, bound, unit, tolerance) in [
            (f64::NAN, 0.0, 1.0, 0.0),
            (0.0, f64::NEG_INFINITY, 1.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, -1.0, 0.0),
            (0.0, 0.0, 1.0, -1.0),
            (0.0, 0.0, 1.0, f64::NAN),
        ] {
            assert_eq!(
                mip_gap(MIN, Some(incumbent), Some(bound), unit, tolerance),
                Gap::Unknown(UnknownGap::NonFinite)
            );
        }
    }

    // R8: derived values that overflow are Unknown, never Known.
    #[test]
    fn r8_absolute_gap_overflow_is_unknown() {
        assert_eq!(
            mip_gap(MIN, Some(f64::MAX), Some(-f64::MAX), 1.0, 0.0),
            Gap::Unknown(UnknownGap::Overflow)
        );
        assert_eq!(
            mip_gap(MAX, Some(-f64::MAX), Some(f64::MAX), 1.0, 0.0),
            Gap::Unknown(UnknownGap::Overflow)
        );
    }

    #[test]
    fn r8_ratio_overflow_is_unknown() {
        let tiny = f64::MIN_POSITIVE / 8.0;
        assert_eq!(
            mip_gap(MIN, Some(0.0), Some(-1.0), tiny, 0.0),
            Gap::Unknown(UnknownGap::Overflow)
        );
        assert_eq!(
            mip_gap(MAX, Some(0.0), Some(1.0), tiny, 0.0),
            Gap::Unknown(UnknownGap::Overflow)
        );
    }

    #[test]
    fn model_scale_changes_the_currency_unit() {
        // Objective in thousands of pounds: one pound is 0.001 model units.
        let (absolute, ratio) = known(mip_gap(MIN, Some(0.0005), Some(0.0), 0.001, 0.0));
        assert!(close(absolute, 0.0005));
        assert!(close(ratio, 0.5));
    }
}
