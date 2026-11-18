//! Common solve outcome statuses (ADR 0003).
//!
//! The termination cause ([`SolveStatus`]) is kept separate from whether an
//! incumbent exists and whether it has passed independent physical feasibility
//! checks ([`IncumbentState`]). An interrupted or time-limited solve may keep a
//! feasible incumbent; it is never treated as complete, and no incumbent is
//! accepted until it is independently validated.

/// Common outcome status: the termination cause only. Each is mapped
/// separately from the native status, which adapters preserve alongside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolveStatus {
    /// Optimal within the declared tolerances.
    Optimal,
    /// Time limit reached with a feasible incumbent.
    FeasibleTimeLimit,
    /// Proven infeasible.
    Infeasible,
    /// Proven unbounded.
    Unbounded,
    /// Infeasible or unbounded; the backend cannot tell which.
    InfeasibleOrUnbounded,
    /// Interrupted by cancellation, before or after an incumbent was found.
    Interrupted,
    /// Stopped without any feasible incumbent.
    NoIncumbent,
    /// The model failed validation before or during load.
    InvalidModel,
    /// The backend failed, crashed or returned invalid output.
    BackendError,
}

impl SolveStatus {
    /// Whether an outcome with this status may carry an incumbent.
    ///
    /// [`SolveStatus::Interrupted`] may: cancellation can arrive after a
    /// feasible incumbent was found (ADR 0003). The status alone never says
    /// the incumbent is accepted; see [`IncumbentState`].
    #[must_use]
    pub const fn may_have_incumbent(self) -> bool {
        matches!(
            self,
            Self::Optimal | Self::FeasibleTimeLimit | Self::Interrupted
        )
    }

    /// Whether an outcome with this status must carry an incumbent.
    #[must_use]
    pub const fn requires_incumbent(self) -> bool {
        matches!(self, Self::Optimal | Self::FeasibleTimeLimit)
    }

    /// Whether the search finished on its own terms (proof of optimality,
    /// infeasibility or unboundedness). Interrupted and time-limited solves are
    /// never complete, whatever incumbent they carry.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(
            self,
            Self::Optimal | Self::Infeasible | Self::Unbounded | Self::InfeasibleOrUnbounded
        )
    }
}

/// Incumbent presence and validation, independent of the termination cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncumbentState {
    /// No incumbent.
    None,
    /// The backend reported an incumbent that has not yet passed independent
    /// physical feasibility checks; it must not be used.
    Unvalidated,
    /// The incumbent passed independent physical feasibility checks.
    Validated,
    /// The incumbent failed independent physical feasibility checks.
    Rejected,
}

/// An outcome combination that the contract forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeError {
    /// The status cannot carry an incumbent but one was reported.
    UnexpectedIncumbent(SolveStatus),
    /// The status requires an incumbent but none was reported.
    MissingIncumbent(SolveStatus),
}

/// Termination cause plus incumbent state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SolveOutcome {
    status: SolveStatus,
    incumbent: IncumbentState,
}

impl SolveOutcome {
    /// Combines a status and an incumbent state, rejecting contradictions.
    ///
    /// # Errors
    ///
    /// Returns [`OutcomeError`] when the status cannot carry the reported
    /// incumbent, or requires one that is absent.
    pub const fn new(status: SolveStatus, incumbent: IncumbentState) -> Result<Self, OutcomeError> {
        let present = !matches!(incumbent, IncumbentState::None);
        if present && !status.may_have_incumbent() {
            return Err(OutcomeError::UnexpectedIncumbent(status));
        }
        if !present && status.requires_incumbent() {
            return Err(OutcomeError::MissingIncumbent(status));
        }
        Ok(Self { status, incumbent })
    }

    /// The termination cause.
    #[must_use]
    pub const fn status(self) -> SolveStatus {
        self.status
    }

    /// The incumbent state.
    #[must_use]
    pub const fn incumbent(self) -> IncumbentState {
        self.incumbent
    }

    /// Whether the incumbent may be used: only after independent validation,
    /// whatever the termination cause.
    #[must_use]
    pub const fn usable_incumbent(self) -> bool {
        matches!(self.incumbent, IncumbentState::Validated)
    }
}

#[cfg(test)]
mod tests {
    use super::{IncumbentState, OutcomeError, SolveOutcome, SolveStatus};

    const ALL: [SolveStatus; 9] = [
        SolveStatus::Optimal,
        SolveStatus::FeasibleTimeLimit,
        SolveStatus::Infeasible,
        SolveStatus::Unbounded,
        SolveStatus::InfeasibleOrUnbounded,
        SolveStatus::Interrupted,
        SolveStatus::NoIncumbent,
        SolveStatus::InvalidModel,
        SolveStatus::BackendError,
    ];

    #[test]
    fn incumbent_capable_statuses() {
        let with: Vec<_> = ALL.into_iter().filter(|s| s.may_have_incumbent()).collect();
        assert_eq!(
            with,
            [
                SolveStatus::Optimal,
                SolveStatus::FeasibleTimeLimit,
                SolveStatus::Interrupted
            ]
        );
    }

    // R7: an interrupted solve may keep a feasible incumbent.
    #[test]
    fn r7_interrupted_after_incumbent_keeps_it_but_is_not_complete() {
        let outcome = SolveOutcome::new(SolveStatus::Interrupted, IncumbentState::Unvalidated)
            .expect("interrupted outcome may carry an incumbent");
        assert!(!outcome.status().is_complete());
        assert!(
            !outcome.usable_incumbent(),
            "unvalidated incumbent is not usable"
        );
        let checked = SolveOutcome::new(SolveStatus::Interrupted, IncumbentState::Validated)
            .expect("validated incumbent survives interruption");
        assert!(checked.usable_incumbent());
        let rejected = SolveOutcome::new(SolveStatus::Interrupted, IncumbentState::Rejected)
            .expect("rejected incumbent is still reported");
        assert!(!rejected.usable_incumbent());
    }

    #[test]
    fn r7_interrupted_before_incumbent_has_none() {
        let outcome = SolveOutcome::new(SolveStatus::Interrupted, IncumbentState::None)
            .expect("interruption before any incumbent is valid");
        assert!(!outcome.usable_incumbent());
    }

    #[test]
    fn contradictory_outcomes_are_rejected() {
        assert_eq!(
            SolveOutcome::new(SolveStatus::Infeasible, IncumbentState::Validated),
            Err(OutcomeError::UnexpectedIncumbent(SolveStatus::Infeasible))
        );
        assert_eq!(
            SolveOutcome::new(SolveStatus::Optimal, IncumbentState::None),
            Err(OutcomeError::MissingIncumbent(SolveStatus::Optimal))
        );
        for status in ALL {
            let result = SolveOutcome::new(status, IncumbentState::Unvalidated);
            assert_eq!(result.is_ok(), status.may_have_incumbent(), "{status:?}");
        }
    }

    #[test]
    fn optimal_incumbent_still_needs_validation() {
        let outcome = SolveOutcome::new(SolveStatus::Optimal, IncumbentState::Unvalidated)
            .expect("valid combination");
        assert!(outcome.status().is_complete());
        assert!(!outcome.usable_incumbent());
    }
}
