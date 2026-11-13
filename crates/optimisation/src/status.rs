//! Common solve outcome statuses (ADR 0003).

/// Common outcome status. Each is mapped separately from the native status,
/// which adapters preserve alongside it. Only [`SolveStatus::Optimal`] and
/// [`SolveStatus::FeasibleTimeLimit`] can carry an incumbent, and even then the
/// incumbent is accepted only after independent physical feasibility checks.
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
    /// Interrupted by cancellation.
    Interrupted,
    /// Stopped without any feasible incumbent.
    NoIncumbent,
    /// The model failed validation before or during load.
    InvalidModel,
    /// The backend failed, crashed or returned invalid output.
    BackendError,
}

impl SolveStatus {
    /// Whether the status can carry a feasible incumbent at all.
    #[must_use]
    pub const fn may_have_incumbent(self) -> bool {
        matches!(self, Self::Optimal | Self::FeasibleTimeLimit)
    }
}

#[cfg(test)]
mod tests {
    use super::SolveStatus;

    #[test]
    fn only_optimal_and_time_limited_feasible_carry_incumbents() {
        let all = [
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
        let with: Vec<_> = all.into_iter().filter(|s| s.may_have_incumbent()).collect();
        assert_eq!(with, [SolveStatus::Optimal, SolveStatus::FeasibleTimeLimit]);
    }
}
