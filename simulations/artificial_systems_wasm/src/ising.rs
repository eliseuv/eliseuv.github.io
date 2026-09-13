//! Ising model on a 2D toroidal lattice.

use crate::lattice::LatticeState;
use crate::spin::SpinHalf;

/// 2D square lattice of spins with periodic (toroidal) boundary
/// conditions.
pub struct IsingLattice {
    state: LatticeState<SpinHalf>,
}

impl IsingLattice {
    /// New lattice in a random (infinite-temperature) configuration.
    /// `uniform` must yield values in `[0, 1)`.
    pub fn new_random(nrows: usize, ncols: usize, mut uniform: impl FnMut() -> f64) -> Self {
        Self { state: LatticeState::new(nrows, ncols, || SpinHalf::random(&mut uniform)) }
    }

    pub fn nrows(&self) -> usize {
        self.state.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.state.ncols()
    }

    /// Row-major spin buffer.
    pub fn spins(&self) -> &[SpinHalf] {
        self.state.sites()
    }

    /// Sum of the four nearest-neighbor spins, wrapping toroidally.
    pub fn nn_sum(&self, row: usize, col: usize) -> i32 {
        self.state.neighbor_values(row, col).iter().map(|s| s.value()).sum()
    }

    /// Energy change (`J = 1`) a flip of site `(row, col)` would cost.
    pub fn flip_energy(&self, row: usize, col: usize) -> i32 {
        2 * self.state.get(row, col).value() * self.nn_sum(row, col)
    }

    pub fn flip(&mut self, row: usize, col: usize) {
        let flipped = self.state.get(row, col).flipped();
        self.state.set(row, col, flipped);
    }

    /// Reset to a random (infinite-temperature) configuration.
    pub fn randomize(&mut self, mut uniform: impl FnMut() -> f64) {
        self.state.fill(|| SpinHalf::random(&mut uniform));
    }

    /// Mean spin per site, in `[-1, 1]`.
    pub fn magnetization(&self) -> f64 {
        self.state.sites().iter().map(|s| s.value() as f64).sum::<f64>() / self.state.sites().len() as f64
    }

    /// Mean bond energy per site (`J = 1`, no external field).
    pub fn energy(&self) -> f64 {
        let mut total = 0f64;
        for row in 0..self.nrows() {
            for col in 0..self.ncols() {
                let s = self.state.get(row, col).value() as f64;
                total -= s * self.nn_sum(row, col) as f64;
            }
        }
        // Each bond was counted twice (once from each endpoint).
        total / (2.0 * self.state.sites().len() as f64)
    }
}
