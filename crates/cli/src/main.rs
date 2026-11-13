//! `egs` command-line interface (FR-15).
//!
//! M0 provides only `egs solvers`, which lists each solver backend's declared
//! capabilities, requirements and availability (FR-18.AC3).

#![forbid(unsafe_code)]

use std::process::ExitCode;

use egs_optimisation::{Availability, Feature, SolverBackend};

fn backends() -> [&'static dyn SolverBackend; 4] {
    [
        &egs_solver_highs::Highs,
        &egs_solver_gurobi::Gurobi,
        &egs_solver_xpress::Xpress,
        &egs_solver_cuopt::CuOpt,
    ]
}

fn solvers_report() -> String {
    let mut report = String::new();
    for backend in backends() {
        let description = backend.describe();
        let features: Vec<_> = Feature::ALL
            .into_iter()
            .filter(|feature| description.capabilities.supports(*feature))
            .map(Feature::name)
            .collect();
        let availability = match &description.availability {
            Availability::Available => "available".to_owned(),
            Availability::Unavailable { reason } => format!("unavailable: {reason}"),
        };
        report.push_str(&format!(
            "{}\n  features: {}\n  device: {:?}\n  licence: {:?}\n  {availability}\n",
            description.id,
            features.join(", "),
            description.device,
            description.licence,
        ));
    }
    report
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["solvers"] => {
            print!("{}", solvers_report());
            ExitCode::SUCCESS
        }
        ["--version"] => {
            println!("egs {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: egs solvers | egs --version");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_backend_is_listed_and_none_claims_availability_yet() {
        let report = solvers_report();
        for id in ["highs", "gurobi", "xpress", "cuopt"] {
            assert!(report.contains(&format!("{id}\n")), "{id} missing");
        }
        assert!(!report.contains("  available\n"));
    }
}
