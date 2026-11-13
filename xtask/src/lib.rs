//! Repository policy checks and CI gates (D-73, ADR 0001).
//!
//! Run `cargo xtask help`. In CI every check runs from the trusted base
//! revision (`--policy`) against the pull-request checkout (`--repo`).

pub mod attribution;
pub mod docs;
pub mod repository;
pub mod review_gates;
pub mod rust_policy;
pub mod util;
pub mod workflows;
