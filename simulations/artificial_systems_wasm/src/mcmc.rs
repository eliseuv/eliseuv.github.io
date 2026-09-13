//! Markov Chain Monte Carlo sampling.

use crate::ising::IsingLattice;

/// Single-spin-flip Metropolis sampler at a fixed temperature.
pub struct MetropolisSampler {
    beta: f64,
}

impl MetropolisSampler {
    pub fn with_temperature(temperature: f64) -> Self {
        Self { beta: temperature.recip() }
    }

    pub fn temperature(&self) -> f64 {
        self.beta.recip()
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.beta = temperature.recip();
    }

    /// One sweep: `nrows * ncols` single-spin-flip attempts. `site` draws a
    /// candidate `(row, col)` given the lattice dimensions; `uniform` draws
    /// the accept/reject threshold from `[0, 1)`.
    pub fn step(
        &self,
        lattice: &mut IsingLattice,
        mut site: impl FnMut(usize, usize) -> (usize, usize),
        mut uniform: impl FnMut() -> f64,
    ) {
        let (nrows, ncols) = (lattice.nrows(), lattice.ncols());
        let n_attempts = nrows * ncols;
        for _ in 0..n_attempts {
            let (row, col) = site(nrows, ncols);
            let delta_e = lattice.flip_energy(row, col);

            if delta_e <= 0 || uniform() < (-self.beta * delta_e as f64).exp() {
                lattice.flip(row, col);
            }
        }
    }
}
