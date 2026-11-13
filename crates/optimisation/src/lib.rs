//! Solver-neutral optimisation contract (ADR 0003, FR-18).
//!
//! M0 defines how a backend declares what it can do, how a request for an
//! unsupported capability fails, the common outcome statuses and the canonical
//! MIP gap rule. The sparse model IR and the load/solve/cancel session
//! operations arrive with the first real adapter in M2a.
//!
//! Nothing here relaxes a request: a MILP request to an LP-only or unavailable
//! backend is an error, never a silent LP solve or a fallback to another solver.

mod capability;
mod gap;
mod status;

pub use capability::{
    Availability, BackendDescription, Capabilities, CapabilityError, DeviceRequirement, Feature,
    LicenceRequirement, SolverBackend,
};
pub use gap::{Gap, ObjectiveSense, UnknownGap, mip_gap};
pub use status::SolveStatus;
