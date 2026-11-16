//! Backend capability declarations and explicit capability errors.

use std::fmt;

/// A capability a caller may require from a backend (ADR 0003 trait table).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    /// Continuous linear programmes.
    Lp,
    /// Mixed-integer linear programmes.
    Milp,
    /// Duals and reduced costs of a solved continuous model.
    LpDuals,
    /// Incremental sparse row addition to a loaded model.
    IncrementalRows,
    /// LP basis save and restore (warm start).
    BasisSaveRestore,
    /// Cooperative interrupt from a callback.
    Interrupt,
    /// Irreducible infeasible subsystem or conflict diagnosis.
    InfeasibilityDiagnosis,
}

impl Feature {
    /// Every feature, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Lp,
        Self::Milp,
        Self::LpDuals,
        Self::IncrementalRows,
        Self::BasisSaveRestore,
        Self::Interrupt,
        Self::InfeasibilityDiagnosis,
    ];

    /// Stable lower-case name used in logs and manifests.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Lp => "lp",
            Self::Milp => "milp",
            Self::LpDuals => "lp_duals",
            Self::IncrementalRows => "incremental_rows",
            Self::BasisSaveRestore => "basis_save_restore",
            Self::Interrupt => "interrupt",
            Self::InfeasibilityDiagnosis => "infeasibility_diagnosis",
        }
    }
}

/// The set of features a backend implements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Capabilities {
    /// See [`Feature::Lp`].
    pub lp: bool,
    /// See [`Feature::Milp`].
    pub milp: bool,
    /// See [`Feature::LpDuals`].
    pub lp_duals: bool,
    /// See [`Feature::IncrementalRows`].
    pub incremental_rows: bool,
    /// See [`Feature::BasisSaveRestore`].
    pub basis_save_restore: bool,
    /// See [`Feature::Interrupt`].
    pub interrupt: bool,
    /// See [`Feature::InfeasibilityDiagnosis`].
    pub infeasibility_diagnosis: bool,
}

impl Capabilities {
    /// Whether `feature` is declared.
    #[must_use]
    pub const fn supports(&self, feature: Feature) -> bool {
        match feature {
            Feature::Lp => self.lp,
            Feature::Milp => self.milp,
            Feature::LpDuals => self.lp_duals,
            Feature::IncrementalRows => self.incremental_rows,
            Feature::BasisSaveRestore => self.basis_save_restore,
            Feature::Interrupt => self.interrupt,
            Feature::InfeasibilityDiagnosis => self.infeasibility_diagnosis,
        }
    }
}

/// Hardware a backend needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceRequirement {
    /// Runs on the CPU.
    Cpu,
    /// Needs an installed, supported NVIDIA GPU (NFR-7).
    NvidiaGpu,
}

/// Licence a backend needs at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenceRequirement {
    /// No licence needed.
    None,
    /// A commercial licence must be installed; never required by default CI.
    Commercial,
}

/// Whether the backend can be used in this build and on this host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Availability {
    /// Ready to load models.
    Available,
    /// Not usable; the reason is reported to the caller verbatim.
    Unavailable {
        /// Why the backend cannot be used.
        reason: String,
    },
}

/// What a backend declares about itself (FR-18.AC3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendDescription {
    /// Stable backend identifier, for example `highs`.
    pub id: &'static str,
    /// Linked native library version, if any library is linked.
    pub native_version: Option<String>,
    /// Features the adapter implements once available.
    pub capabilities: Capabilities,
    /// Hardware requirement.
    pub device: DeviceRequirement,
    /// Licence requirement.
    pub licence: LicenceRequirement,
    /// Whether the backend can be used now.
    pub availability: Availability,
}

impl BackendDescription {
    /// Checks that the backend is available and supports every requested
    /// feature. The first missing piece is returned as an explicit error.
    pub fn require(&self, features: &[Feature]) -> Result<(), CapabilityError> {
        if let Availability::Unavailable { reason } = &self.availability {
            return Err(CapabilityError::BackendUnavailable {
                backend: self.id,
                reason: reason.clone(),
            });
        }
        match features
            .iter()
            .copied()
            .find(|feature| !self.capabilities.supports(*feature))
        {
            Some(feature) => Err(CapabilityError::Unsupported {
                backend: self.id,
                feature,
            }),
            None => Ok(()),
        }
    }
}

/// A request the backend cannot honour. Callers must surface it; they must not
/// relax the model or switch backend silently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    /// The backend cannot be used in this build or on this host.
    BackendUnavailable {
        /// Backend identifier.
        backend: &'static str,
        /// Why it is unavailable.
        reason: String,
    },
    /// The backend does not implement a requested feature.
    Unsupported {
        /// Backend identifier.
        backend: &'static str,
        /// The missing feature.
        feature: Feature,
    },
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnavailable { backend, reason } => {
                write!(
                    formatter,
                    "solver backend `{backend}` is unavailable: {reason}"
                )
            }
            Self::Unsupported { backend, feature } => write!(
                formatter,
                "solver backend `{backend}` does not support `{}`",
                feature.name()
            ),
        }
    }
}

impl std::error::Error for CapabilityError {}

/// A solver backend adapter.
///
/// The trait deliberately does not require `Send + Sync`: each worker process
/// exclusively owns its native handle (ADR 0003). Session operations
/// (load, add rows, solve, cancel, sensitivity, diagnostics) are added with
/// the model IR in M2a.
pub trait SolverBackend {
    /// Describes the backend, its requirements and its availability.
    fn describe(&self) -> BackendDescription;

    /// Fails explicitly unless every feature is available.
    fn require(&self, features: &[Feature]) -> Result<(), CapabilityError> {
        self.describe().require(features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lp_only(availability: Availability) -> BackendDescription {
        BackendDescription {
            id: "test-lp",
            native_version: None,
            capabilities: Capabilities {
                lp: true,
                lp_duals: true,
                ..Capabilities::default()
            },
            device: DeviceRequirement::Cpu,
            licence: LicenceRequirement::None,
            availability,
        }
    }

    #[test]
    fn milp_request_on_lp_backend_is_rejected_not_relaxed() {
        let error = lp_only(Availability::Available)
            .require(&[Feature::Lp, Feature::Milp])
            .expect_err("MILP must not be relaxed to LP");
        assert_eq!(
            error,
            CapabilityError::Unsupported {
                backend: "test-lp",
                feature: Feature::Milp
            }
        );
        assert_eq!(
            error.to_string(),
            "solver backend `test-lp` does not support `milp`"
        );
    }

    #[test]
    fn unavailable_backend_rejects_even_supported_features() {
        let description = lp_only(Availability::Unavailable {
            reason: "not linked".into(),
        });
        assert!(matches!(
            description.require(&[Feature::Lp]),
            Err(CapabilityError::BackendUnavailable { .. })
        ));
        assert!(matches!(
            description.require(&[]),
            Err(CapabilityError::BackendUnavailable { .. })
        ));
    }

    #[test]
    fn supported_request_succeeds() {
        assert_eq!(
            lp_only(Availability::Available).require(&[Feature::Lp, Feature::LpDuals]),
            Ok(())
        );
    }

    #[test]
    fn supports_matches_each_field() {
        for feature in Feature::ALL {
            let mut capabilities = Capabilities::default();
            assert!(!capabilities.supports(feature));
            match feature {
                Feature::Lp => capabilities.lp = true,
                Feature::Milp => capabilities.milp = true,
                Feature::LpDuals => capabilities.lp_duals = true,
                Feature::IncrementalRows => capabilities.incremental_rows = true,
                Feature::BasisSaveRestore => capabilities.basis_save_restore = true,
                Feature::Interrupt => capabilities.interrupt = true,
                Feature::InfeasibilityDiagnosis => capabilities.infeasibility_diagnosis = true,
            }
            for other in Feature::ALL {
                assert_eq!(capabilities.supports(other), other == feature, "{other:?}");
            }
        }
    }
}
