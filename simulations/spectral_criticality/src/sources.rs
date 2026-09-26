//! Time series sources advanced step by step, so the page can show the time series matrix
//! filling up column by column.

use artificial_systems::analysis::toy::correlated_pairs;
use artificial_systems::dynamics::Dynamics;
use artificial_systems::ensemble::Ensemble;
use artificial_systems::observable::Observable;
use artificial_systems::rng::{stream, DefaultRng};
use artificial_systems::state::{Configuration, Prepare};
use ndarray::{Array2, ArrayViewMut1};

use crate::models::{ContactProcessState, IsingSystem};

/// Source of `n_samples` time series, generated one time step at a time.
pub trait Source {
    /// Restart every series for run `run`, writing the initial measurements into `column`.
    fn start_run(&mut self, run: u64, column: ArrayViewMut1<f64>);

    /// Advance every series to step `t` (`>= 1`), writing its measurements into `column`.
    fn advance(&mut self, t: usize, column: ArrayViewMut1<f64>);

    /// Sites of sample `sample`, one byte each, with the lattice `(rows, cols)`, if the series
    /// come from a lattice.
    fn sites(&self, sample: usize) -> Option<(&[u8], (usize, usize))>;
}

/// Lattice systems whose sites can be shown.
pub trait Lattice {
    fn site_bytes(&self) -> &[u8];
    fn shape(&self) -> (usize, usize);
}

impl Lattice for IsingSystem {
    fn site_bytes(&self) -> &[u8] {
        let sites = self.state().sites();
        // Sound: `SpinHalf` is `#[repr(i8)]`, read back as `i8` by the page.
        unsafe { std::slice::from_raw_parts(sites.as_ptr().cast(), sites.len()) }
    }

    fn shape(&self) -> (usize, usize) {
        let [rows, cols] = self.state().topology().lengths();
        (rows, cols)
    }
}

impl Lattice for ContactProcessState {
    fn site_bytes(&self) -> &[u8] {
        let sites = self.sites();
        // Sound: `Binary` is `#[repr(u8)]`.
        unsafe { std::slice::from_raw_parts(sites.as_ptr().cast(), sites.len()) }
    }

    fn shape(&self) -> (usize, usize) {
        (1, Configuration::len(self))
    }
}

/// Independent Markov chains of an [`Ensemble`], stepped in lockstep.
///
/// Each chain draws from the same random stream as [`Ensemble::time_series`], so run `r` yields
/// exactly the series of `ensemble.matrix(r, n_samples)`.
pub struct Chains<Sys, D, P, O> {
    ensemble: Ensemble<Sys, D, P, O>,
    systems: Vec<Sys>,
    dynamics: Vec<D>,
    rngs: Vec<DefaultRng>,
    frozen: Vec<bool>,
}

impl<Sys, D, P, O> Chains<Sys, D, P, O>
where
    Sys: Clone + Send + Sync,
    D: Dynamics<Sys>,
    P: Prepare<Sys>,
    O: Observable<Sys, Output = f64>,
{
    pub fn new(ensemble: Ensemble<Sys, D, P, O>, n_samples: usize) -> Self {
        assert_eq!(ensemble.schedule.burn_in, 0, "Burn-in is not shown live");
        assert_eq!(ensemble.schedule.stride, 1, "Every step is shown live");
        Self {
            systems: vec![ensemble.system.clone(); n_samples],
            dynamics: vec![ensemble.dynamics.clone(); n_samples],
            rngs: (0..n_samples as u64)
                .map(|sample| stream(ensemble.seed, &[0, sample]))
                .collect(),
            frozen: vec![false; n_samples],
            ensemble,
        }
    }
}

impl<Sys, D, P, O> Source for Chains<Sys, D, P, O>
where
    Sys: Clone + Send + Sync + Lattice,
    D: Dynamics<Sys>,
    P: Prepare<Sys>,
    O: Observable<Sys, Output = f64>,
{
    fn start_run(&mut self, run: u64, mut column: ArrayViewMut1<f64>) {
        for sample in 0..self.systems.len() {
            let mut rng = stream(self.ensemble.seed, &[run, sample as u64]);
            let mut system = self.ensemble.system.clone();
            self.ensemble.prepare.prepare(&mut system, &mut rng);
            column[sample] = self.ensemble.observable.measure(&system);
            self.systems[sample] = system;
            self.dynamics[sample] = self.ensemble.dynamics.clone();
            self.rngs[sample] = rng;
            self.frozen[sample] = false;
        }
    }

    fn advance(&mut self, _t: usize, mut column: ArrayViewMut1<f64>) {
        for sample in 0..self.systems.len() {
            let system = &mut self.systems[sample];
            let dynamics = &mut self.dynamics[sample];
            // Absorbed chains repeat their last measurement without drawing further, as in
            // `Ensemble::time_series`.
            if !self.frozen[sample] && dynamics.is_frozen(system) {
                self.frozen[sample] = true;
            }
            if !self.frozen[sample] {
                dynamics.step(system, &mut self.rngs[sample]);
            }
            column[sample] = self.ensemble.observable.measure(system);
        }
    }

    fn sites(&self, sample: usize) -> Option<(&[u8], (usize, usize))> {
        let system = self.systems.get(sample)?;
        Some((system.site_bytes(), system.shape()))
    }
}

/// Gaussian series whose consecutive pairs of rows have correlation `rho` (white noise for
/// `rho = 0`), drawn a whole matrix at a time and revealed step by step.
pub struct CorrelatedPairs {
    rho: f64,
    n_samples: usize,
    n_steps: usize,
    seed: u64,
    matrix: Array2<f64>,
}

impl CorrelatedPairs {
    /// # Panics
    /// If `n_samples` is odd or `rho` is not in `[-1, 1]`.
    pub fn new(rho: f64, n_samples: usize, n_steps: usize, seed: u64) -> Self {
        assert!(n_samples.is_multiple_of(2), "Samples come in correlated pairs");
        assert!(
            (-1.0..=1.0).contains(&rho),
            "Correlation must be in [-1, 1]"
        );
        Self {
            rho,
            n_samples,
            n_steps,
            seed,
            matrix: Array2::zeros((n_samples, n_steps + 1)),
        }
    }
}

impl Source for CorrelatedPairs {
    fn start_run(&mut self, run: u64, mut column: ArrayViewMut1<f64>) {
        let mut rng = stream(self.seed, &[run]);
        self.matrix = correlated_pairs(self.rho, self.n_steps + 1, self.n_samples / 2, &mut rng);
        column.assign(&self.matrix.column(0));
    }

    fn advance(&mut self, t: usize, mut column: ArrayViewMut1<f64>) {
        column.assign(&self.matrix.column(t));
    }

    fn sites(&self, _sample: usize) -> Option<(&[u8], (usize, usize))> {
        None
    }
}

#[cfg(test)]
mod tests {
    use artificial_systems::ensemble::Schedule;
    use artificial_systems::observable::{Density, Magnetization};
    use artificial_systems::site::Binary;

    use super::*;
    use crate::models::*;

    fn live_matrix<S: Source>(
        source: &mut S,
        run: u64,
        n_samples: usize,
        n_steps: usize,
    ) -> Array2<f64> {
        let mut matrix = Array2::zeros((n_samples, n_steps + 1));
        source.start_run(run, matrix.column_mut(0));
        for t in 1..=n_steps {
            source.advance(t, matrix.column_mut(t));
        }
        matrix
    }

    #[test]
    fn live_ising_chains_reproduce_the_ensemble() {
        let ensemble = Ensemble {
            system: ising_system(),
            dynamics: ising_dynamics(1.0),
            prepare: ising_init(),
            observable: Magnetization,
            schedule: Schedule::new(20),
            seed: 7,
        };
        let expected = ensemble.matrix(3, 4);
        let mut chains = Chains::new(ensemble, 4);
        assert_eq!(live_matrix(&mut chains, 3, 4, 20), expected);
    }

    #[test]
    fn live_contact_process_chains_reproduce_the_ensemble() {
        // Subcritical, so some chains are absorbed within the series.
        let ensemble = Ensemble {
            system: contact_process_state(),
            dynamics: contact_process_dynamics(1.5, 0.5),
            prepare: contact_process_init(),
            observable: Density(Binary::Active),
            schedule: Schedule::new(200),
            seed: 11,
        };
        let expected = ensemble.matrix(1, 6);
        assert!(expected.column(200).iter().any(|&rho| rho == 0.0));
        let mut chains = Chains::new(ensemble, 6);
        assert_eq!(live_matrix(&mut chains, 1, 6, 200), expected);
    }
}
