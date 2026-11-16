//! HiGHS adapter: the default open-source CPU LP/MILP backend (ADR 0003).
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
pub const ID: &str = "highs";

/// The Highs backend.
#[derive(Debug, Default, Clone, Copy)]
pub struct Highs;

impl SolverBackend for Highs {
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
                infeasibility_diagnosis: false,
            },
            device: DeviceRequirement::Cpu,
            licence: LicenceRequirement::None,
            availability: Availability::Unavailable {
                reason: "native HiGHS adapter is not built yet (planned for M2a)".to_owned(),
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
            Highs.require(&[Feature::Lp]),
            Err(CapabilityError::BackendUnavailable { backend: ID, .. })
        ));
    }
}
