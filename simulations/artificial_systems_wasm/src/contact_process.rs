//! Contact process on a 2D toroidal lattice — the directed-percolation
//! universality-class model of spreading/healing activity.

use crate::binary::Binary;
use crate::lattice::LatticeState;

/// 2D toroidal lattice of active/inactive sites.
pub struct ContactLattice {
    state: LatticeState<Binary>,
}

impl ContactLattice {
    /// New lattice in a random (~50/50 active/inactive) configuration.
    /// `uniform` must yield values in `[0, 1)`.
    pub fn new_random(nrows: usize, ncols: usize, mut uniform: impl FnMut() -> f64) -> Self {
        Self { state: LatticeState::new(nrows, ncols, || Binary::random(&mut uniform)) }
    }

    pub fn nrows(&self) -> usize {
        self.state.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.state.ncols()
    }

    /// Row-major active-site buffer.
    pub fn active(&self) -> &[Binary] {
        self.state.sites()
    }

    pub fn is_active(&self, row: usize, col: usize) -> bool {
        self.state.get(row, col).is_active()
    }

    pub fn set_active(&mut self, row: usize, col: usize, active: bool) {
        self.state.set(row, col, if active { Binary::Active } else { Binary::Inactive });
    }

    pub fn neighbor_states(&self, row: usize, col: usize) -> [Binary; 4] {
        self.state.neighbor_values(row, col)
    }

    /// Reset to a random (~50/50 active/inactive) configuration.
    pub fn randomize(&mut self, mut uniform: impl FnMut() -> f64) {
        self.state.fill(|| Binary::random(&mut uniform));
    }

    /// Clear to all-inactive, then activate a single site at the center.
    pub fn seed_center(&mut self) {
        self.state.fill(|| Binary::Inactive);
        self.set_active(self.nrows() / 2, self.ncols() / 2, true);
    }

    /// Fraction of sites currently active, in `[0, 1]`.
    pub fn active_fraction(&self) -> f64 {
        let active_count = self.state.sites().iter().filter(|s| s.is_active()).count();
        active_count as f64 / self.state.sites().len() as f64
    }
}

/// Contact-process sampler at a fixed healing probability.
///
/// Each attempt: pick a uniformly random site. If it is Active, heal it
/// to Inactive with probability `p`. If it is Inactive, it adopts the
/// current state of one uniformly-random nearest neighbor — the sole
/// infection mechanism, no separate infection-probability parameter.
pub struct ContactSampler {
    p: f64,
}

impl ContactSampler {
    /// `p` is the per-attempt healing probability for an active site,
    /// in `(0, 1]`.
    pub fn with_healing_probability(p: f64) -> Self {
        Self { p }
    }

    pub fn healing_probability(&self) -> f64 {
        self.p
    }

    pub fn set_healing_probability(&mut self, p: f64) {
        self.p = p;
    }

    /// One sweep: `nrows * ncols` attempts. `site` draws a candidate
    /// `(row, col)`; `neighbor_choice` draws an index in `0..4` (which
    /// neighbor to adopt from, when Inactive); `uniform` draws the
    /// healing threshold from `[0, 1)`.
    pub fn step(
        &self,
        lattice: &mut ContactLattice,
        mut site: impl FnMut(usize, usize) -> (usize, usize),
        mut neighbor_choice: impl FnMut() -> usize,
        mut uniform: impl FnMut() -> f64,
    ) {
        let (nrows, ncols) = (lattice.nrows(), lattice.ncols());
        let n_attempts = nrows * ncols;
        for _ in 0..n_attempts {
            let (row, col) = site(nrows, ncols);

            if lattice.is_active(row, col) {
                if uniform() < self.p {
                    lattice.set_active(row, col, false);
                }
            } else {
                let neighbors = lattice.neighbor_states(row, col);
                let k = neighbor_choice() % 4;
                lattice.set_active(row, col, neighbors[k].is_active());
            }
        }
    }
}
