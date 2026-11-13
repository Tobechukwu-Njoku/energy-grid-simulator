//! Optional Gurobi adapter (ADR 0003).
//!
//! Contract stub: it declares the adapter's planned capabilities and
//! requirements, and reports itself unavailable. It links no native library or
//! SDK, so default builds never need a licence or device (FR-18.AC4, NFR-10.AC3).
//!
//! When the native adapter lands, unsafe FFI may be allowed item by item in
//! this crate only, with a `// SAFETY:` comment per block (ADR 0001).

use egs_optimisation::{
    Availability, BackendDescription, Capabilities, DeviceRequirement, LicenceRequirement,
    SolverBackend,
};

/// Backend identifier.
pub const ID: &str = "gurobi";

/// The Gurobi backend.
#[derive(Debug, Default, Clone, Copy)]
pub struct Gurobi;

impl SolverBackend for Gurobi {
    fn describe(&self) -> BackendDescription {
        BackendDescription {
            id: ID,
            native_version: None,
            capabilities: Capabilities {
                lp: true,
                milp: true,
                lp_duals: true,
                incremental_rows: true,
                basis_save_restore: true,
                interrupt: true,
                infeasibility_diagnosis: true,
            },
            device: DeviceRequirement::Cpu,
            licence: LicenceRequirement::Commercial,
            availability: Availability::Unavailable {
                reason: "optional Gurobi adapter is not implemented; no SDK is linked".to_owned(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egs_optimisation::{CapabilityError, Feature};

    #[test]
    fn stub_fails_explicitly() {
        assert!(matches!(
            Gurobi.require(&[Feature::Lp]),
            Err(CapabilityError::BackendUnavailable { backend: ID, .. })
        ));
    }
}
