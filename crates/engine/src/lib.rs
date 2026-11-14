//! Run orchestration, worker lifecycle and rolling, persisted state
//! (ADR 0001, ADR 0005).
//!
//! Skeleton crate (M0). Job dispatch and the rolling pipeline arrive in M2/M3.

#![forbid(unsafe_code)]
#![doc(test(attr(forbid(unsafe_code))))]
