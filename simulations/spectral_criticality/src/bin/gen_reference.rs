//! Precompute the reference scans shown behind the live points of the spectral criticality page.
//!
//! Usage: `gen_reference <output dir>`. Writes `ising_2d.cbor.gz` and
//! `contact_process_1d.cbor.gz`, each a list of [`ReferenceScan`]s.

use std::path::{Path, PathBuf};
use std::time::Instant;

use artificial_systems::analysis::{
    correlation_matrix, eigenvalues, standardize_rows, ZeroVariance,
};
use artificial_systems::dynamics::Dynamics;
use artificial_systems::ensemble::{Ensemble, Schedule};
use artificial_systems::io::{DataFile, Format};
use artificial_systems::observable::{Density, Magnetization, Observable};
use artificial_systems::site::Binary;
use artificial_systems::state::Prepare;
use spectral_criticality::models::*;
use spectral_criticality::reference::{ReferencePoint, ReferenceScan, SpectrumAccumulator};

/// Matrices per parameter point, as in the thesis.
const N_RUNS: usize = 1000;
const SEED: u64 = 20_260_926;

/// `n` evenly spaced values from `start` to `end`, both included.
fn grid(start: f64, end: f64, n: usize) -> Vec<f64> {
    (0..n)
        .map(|k| start + (end - start) * k as f64 / (n - 1) as f64)
        .collect()
}

fn summarize<Sys, D, P, O>(ensemble: &Ensemble<Sys, D, P, O>, parameter: f64) -> ReferencePoint
where
    Sys: Clone + Send + Sync,
    D: Dynamics<Sys>,
    P: Prepare<Sys>,
    O: Observable<Sys, Output = f64>,
{
    let spectra = ensemble.map_runs(N_RUNS, N_SAMPLES, |_, mut matrix| {
        standardize_rows(&mut matrix, 0, ZeroVariance::Zero).expect("zero variance is zeroed");
        let g = correlation_matrix(matrix.view());
        let eig = eigenvalues(g.view()).expect("eigendecomposition converges");
        (g, eig)
    });
    let mut accumulator = SpectrumAccumulator::default();
    for (g, eig) in &spectra {
        accumulator.push(g.view(), eig);
    }
    accumulator.summarize(parameter)
}

fn ising_scan() -> ReferenceScan {
    let points = grid(0.6, 1.4, 41)
        .into_iter()
        .enumerate()
        .map(|(k, t_over_tc)| {
            let ensemble = Ensemble {
                system: ising_system(),
                dynamics: ising_dynamics(t_over_tc),
                prepare: ising_init(),
                observable: Magnetization,
                schedule: Schedule::new(ISING_N_STEPS),
                seed: SEED + k as u64,
            };
            summarize(&ensemble, t_over_tc)
        })
        .collect();
    ReferenceScan {
        model: "ising_2d".into(),
        parameter: "T/T_c".into(),
        fixed: vec![("L".into(), ISING_LENGTH as f64)],
        n_samples: N_SAMPLES,
        n_steps: ISING_N_STEPS,
        seed: SEED,
        points,
    }
}

fn contact_process_scan(gamma: f64) -> ReferenceScan {
    let points = grid(2.0, 4.0, 41)
        .into_iter()
        .enumerate()
        .map(|(k, alpha)| {
            let ensemble = Ensemble {
                system: contact_process_state(),
                dynamics: contact_process_dynamics(alpha, gamma),
                prepare: contact_process_init(),
                observable: Density(Binary::Active),
                schedule: Schedule::new(CONTACT_PROCESS_N_STEPS),
                seed: SEED + k as u64,
            };
            summarize(&ensemble, alpha)
        })
        .collect();
    ReferenceScan {
        model: "contact_process_1d".into(),
        parameter: "alpha".into(),
        fixed: vec![
            ("L".into(), CONTACT_PROCESS_LENGTH as f64),
            ("gamma".into(), gamma),
        ],
        n_samples: N_SAMPLES,
        n_steps: CONTACT_PROCESS_N_STEPS,
        seed: SEED,
        points,
    }
}

fn write(dir: &Path, stem: &str, scans: Vec<ReferenceScan>) {
    let file = DataFile::new(dir.join(stem), Format::Cbor, true);
    file.write(&scans).expect("writable output");
    println!("Written {}", file.path().display());
}

fn main() {
    let dir: PathBuf = std::env::args()
        .nth(1)
        .expect("usage: gen_reference <output dir>")
        .into();

    let timer = Instant::now();
    write(&dir, "ising_2d", vec![ising_scan()]);
    println!("Ising scan in {:.0} s", timer.elapsed().as_secs_f64());

    let timer = Instant::now();
    let scans = [0.0, 0.5, 1.0].map(contact_process_scan).into();
    write(&dir, "contact_process_1d", scans);
    println!(
        "Contact process scans in {:.0} s",
        timer.elapsed().as_secs_f64()
    );
}
