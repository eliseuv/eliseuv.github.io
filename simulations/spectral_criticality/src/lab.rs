//! wasm-bindgen interface of the page: one [`SpectralLab`] per source and parameter point.

use artificial_systems::analysis::{
    correlation_matrix, eigenvalues, standardize_rows, MarchenkoPastur, ZeroVariance,
};
use artificial_systems::ensemble::{Ensemble, Schedule};
use artificial_systems::observable::{Density, Magnetization};
use artificial_systems::rng::entropy_seed;
use artificial_systems::site::Binary;
use ndarray::Array2;
use wasm_bindgen::prelude::*;

use crate::models::*;
use crate::reference::{self, SpectrumAccumulator};
use crate::sources::{Chains, CorrelatedPairs, Source};

/// Series length of the noise and toy sources, as in the thesis toy model.
const TOY_N_STEPS: usize = 300;

/// Time series matrices of one source at one parameter point, generated run after run, with
/// the spectra of their correlation matrices accumulated.
///
/// The time series matrix is the run in progress; the correlation matrix and its eigenvalues
/// are those of the last completed run.
#[wasm_bindgen]
pub struct SpectralLab {
    source: Box<dyn Source>,
    n_steps: usize,
    series: Array2<f64>,
    /// Last filled column of `series`.
    t: usize,
    run: u64,
    correlations: Array2<f64>,
    eigenvalues: Vec<f64>,
    accumulator: SpectrumAccumulator,
}

impl SpectralLab {
    fn new(source: Box<dyn Source>, n_steps: usize) -> SpectralLab {
        console_error_panic_hook::set_once();
        let mut lab = SpectralLab {
            source,
            n_steps,
            series: Array2::zeros((N_SAMPLES, n_steps + 1)),
            t: 0,
            run: 0,
            correlations: Array2::zeros((N_SAMPLES, N_SAMPLES)),
            eigenvalues: Vec::new(),
            accumulator: SpectrumAccumulator::default(),
        };
        lab.start_run();
        lab
    }

    fn start_run(&mut self) {
        self.series.fill(0.0);
        self.source.start_run(self.run, self.series.column_mut(0));
        self.t = 0;
    }

    fn complete_run(&mut self) {
        let mut standardized = self.series.clone();
        standardize_rows(&mut standardized, 0, ZeroVariance::Zero)
            .expect("constant series are zeroed");
        self.correlations = correlation_matrix(standardized.view());
        self.eigenvalues =
            eigenvalues(self.correlations.view()).expect("eigendecomposition converges");
        self.accumulator
            .push(self.correlations.view(), &self.eigenvalues);
        self.run += 1;
    }
}

#[wasm_bindgen]
impl SpectralLab {
    /// Uncorrelated Gaussian series: the Marchenko-Pastur baseline.
    pub fn white_noise() -> SpectralLab {
        SpectralLab::correlated_pairs(0.0)
    }

    /// Gaussian series whose consecutive pairs have correlation `rho`.
    pub fn correlated_pairs(rho: f64) -> SpectralLab {
        let source = CorrelatedPairs::new(rho, N_SAMPLES, TOY_N_STEPS, entropy_seed());
        SpectralLab::new(Box::new(source), TOY_N_STEPS)
    }

    /// Magnetization series of the square lattice Ising model at `t_over_tc` times the Onsager
    /// temperature, from random configurations.
    pub fn ising(t_over_tc: f64) -> SpectralLab {
        let ensemble = Ensemble {
            system: ising_system(),
            dynamics: ising_dynamics(t_over_tc),
            prepare: ising_init(),
            observable: Magnetization,
            schedule: Schedule::new(ISING_N_STEPS),
            seed: entropy_seed(),
        };
        SpectralLab::new(Box::new(Chains::new(ensemble, N_SAMPLES)), ISING_N_STEPS)
    }

    /// Density series of the one-dimensional contact process with infection rate `alpha` and
    /// diffusion probability `gamma`, from fully active chains.
    pub fn contact_process(alpha: f64, gamma: f64) -> SpectralLab {
        let ensemble = Ensemble {
            system: contact_process_state(),
            dynamics: contact_process_dynamics(alpha, gamma),
            prepare: contact_process_init(),
            observable: Density(Binary::Active),
            schedule: Schedule::new(CONTACT_PROCESS_N_STEPS),
            seed: entropy_seed(),
        };
        SpectralLab::new(
            Box::new(Chains::new(ensemble, N_SAMPLES)),
            CONTACT_PROCESS_N_STEPS,
        )
    }

    /// Advance by `steps` time steps, completing and analysing runs (and starting new ones) as
    /// needed. Returns the number of runs completed.
    pub fn advance(&mut self, steps: usize) -> usize {
        let mut completed = 0;
        for _ in 0..steps {
            if self.t == self.n_steps {
                self.complete_run();
                completed += 1;
                self.start_run();
                continue;
            }
            self.t += 1;
            self.source.advance(self.t, self.series.column_mut(self.t));
        }
        completed
    }

    pub fn n_samples(&self) -> usize {
        N_SAMPLES
    }

    /// Measurements per series after the initial one.
    pub fn n_steps(&self) -> usize {
        self.n_steps
    }

    /// Last filled time step of the run in progress.
    pub fn t(&self) -> usize {
        self.t
    }

    /// Time series matrix of the run in progress, `n_samples × (n_steps + 1)` row-major `f64`;
    /// columns after `t` are zero.
    pub fn series(&self) -> *const f64 {
        self.series.as_ptr()
    }

    /// Correlation matrix of the last completed run, `n_samples × n_samples` row-major `f64`.
    pub fn correlations(&self) -> *const f64 {
        self.correlations.as_ptr()
    }

    /// Eigenvalues (ascending) of the last completed run; empty before the first one.
    pub fn eigenvalues(&self) -> Vec<f64> {
        self.eigenvalues.clone()
    }

    /// Completed runs, i.e. correlation matrices accumulated.
    pub fn n_matrices(&self) -> u64 {
        self.accumulator.n_matrices()
    }

    /// Eigenvalues of every completed run, pointer to `n_matrices() * n_samples()` `f64`.
    pub fn accumulated_eigenvalues(&self) -> *const f64 {
        self.accumulator.eigenvalues().as_ptr()
    }

    pub fn eigenvalue_variance(&self) -> f64 {
        self.accumulator.eigenvalue_variance()
    }

    pub fn max_eigenvalue_mean(&self) -> f64 {
        self.accumulator.max_eigenvalue().mean()
    }

    pub fn max_eigenvalue_variance(&self) -> f64 {
        self.accumulator.max_eigenvalue().variance(0)
    }

    /// Counts of the off-diagonal correlations of every completed run over
    /// [`reference::N_BINS`] equal bins of `[-1, 1]`.
    pub fn correlation_counts(&self) -> Vec<f64> {
        self.accumulator
            .correlations()
            .map_or_else(Vec::new, |h| h.counts().iter().map(|&c| c as f64).collect())
    }

    /// Sites of sample `sample` (one byte each: `i8` spins, `u8` activity), or null for
    /// sources without a lattice.
    pub fn sites(&self, sample: usize) -> *const u8 {
        self.source
            .sites(sample)
            .map_or(std::ptr::null(), |(sites, _)| sites.as_ptr())
    }

    /// Lattice rows (`0` for sources without a lattice).
    pub fn lattice_rows(&self) -> usize {
        self.source.sites(0).map_or(0, |(_, (rows, _))| rows)
    }

    /// Lattice columns (`0` for sources without a lattice).
    pub fn lattice_cols(&self) -> usize {
        self.source.sites(0).map_or(0, |(_, (_, cols))| cols)
    }
}

/// Marchenko-Pastur density of `n_samples` uncorrelated series of `n_steps + 1` values.
#[wasm_bindgen]
pub fn marchenko_pastur_density(n_steps: usize, lambda: f64) -> f64 {
    MarchenkoPastur::new(N_SAMPLES, n_steps + 1).density(lambda)
}

/// Decode gzipped CBOR reference scans into JSON.
#[wasm_bindgen]
pub fn decode_reference(gzipped_cbor: &[u8]) -> Result<String, JsError> {
    let scans = reference::decode(gzipped_cbor).map_err(|e| JsError::new(&e))?;
    serde_json::to_string(&scans).map_err(|e| JsError::new(&e.to_string()))
}

/// Critical temperature of the infinite square lattice Ising model (Onsager).
#[wasm_bindgen]
pub fn ising_t_critical() -> f64 {
    ISING_SQUARE_T_CRITICAL
}

/// Best estimate of the critical infection rate of the one-dimensional contact process.
#[wasm_bindgen]
pub fn contact_process_alpha_critical() -> f64 {
    CONTACT_PROCESS_CHAIN_ALPHA_CRITICAL
}
