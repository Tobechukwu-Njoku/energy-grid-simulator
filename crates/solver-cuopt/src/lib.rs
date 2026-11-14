//! Optional NVIDIA cuOpt adapter (ADR 0003): trait and capability stub only in P1.
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
pub const ID: &str = "cuopt";

/// The CuOpt backend.
#[derive(Debug, Default, Clone, Copy)]
pub struct CuOpt;

impl SolverBackend for CuOpt {
    fn describe(&self) -> BackendDescription {
        BackendDescription {
            id: ID,
            native_version: None,
            capabilities: Capabilities {
                // No MIP maturity is assumed for cuOpt (ADR 0003).
                lp: true,
                ..Capabilities::default()
            },
            device: DeviceRequirement::NvidiaGpu,
            licence: LicenceRequirement::None,
            availability: Availability::Unavailable {
                reason: "cuOpt is a capability stub only in P1; no implementation or device is available".to_owned(),
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
            CuOpt.require(&[Feature::Lp]),
            Err(CapabilityError::BackendUnavailable { backend: ID, .. })
        ));
    }
}
