//! Lattice spin systems and Markov Chain Monte Carlo sampling.
//!
//! Deliberately dependency-free (no RNG crate, no wasm-bindgen): callers
//! supply randomness through closures, so the same core runs unchanged
//! behind a wasm binding (using `js_sys::Math::random`) or in a native
//! context (using any `rand`-family generator). A lightweight,
//! wasm-scoped analog of a full research spin-system library, meant to
//! back this and future lattice simulations on the site.

/// 2D square lattice of `+1`/`-1` spins with periodic (toroidal) boundary
/// conditions.
pub struct IsingLattice {
    nrows: usize,
    ncols: usize,
    spins: Vec<i8>,
}

impl IsingLattice {
    /// New lattice in a random (infinite-temperature) configuration.
    /// `uniform` must yield values in `[0, 1)`.
    pub fn new_random(nrows: usize, ncols: usize, mut uniform: impl FnMut() -> f64) -> Self {
        let spins = (0..nrows * ncols).map(|_| Self::random_spin(&mut uniform)).collect();
        Self { nrows, ncols, spins }
    }

    fn random_spin(uniform: &mut impl FnMut() -> f64) -> i8 {
        if uniform() < 0.5 { 1 } else { -1 }
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.ncols + col
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    /// Row-major spin buffer, one `i8` (`+1`/`-1`) per site.
    pub fn spins(&self) -> &[i8] {
        &self.spins
    }

    /// Sum of the four nearest-neighbor spins, wrapping toroidally.
    pub fn nn_sum(&self, row: usize, col: usize) -> i32 {
        let up = (row + self.nrows - 1) % self.nrows;
        let down = (row + 1) % self.nrows;
        let left = (col + self.ncols - 1) % self.ncols;
        let right = (col + 1) % self.ncols;

        self.spins[self.idx(up, col)] as i32
            + self.spins[self.idx(down, col)] as i32
            + self.spins[self.idx(row, left)] as i32
            + self.spins[self.idx(row, right)] as i32
    }

    /// Energy change (`J = 1`) a flip of site `(row, col)` would cost.
    pub fn flip_energy(&self, row: usize, col: usize) -> i32 {
        let s = self.spins[self.idx(row, col)] as i32;
        2 * s * self.nn_sum(row, col)
    }

    pub fn flip(&mut self, row: usize, col: usize) {
        let i = self.idx(row, col);
        self.spins[i] = -self.spins[i];
    }

    /// Reset to a random (infinite-temperature) configuration.
    pub fn randomize(&mut self, mut uniform: impl FnMut() -> f64) {
        for spin in self.spins.iter_mut() {
            *spin = Self::random_spin(&mut uniform);
        }
    }

    /// Mean spin per site, in `[-1, 1]`.
    pub fn magnetization(&self) -> f64 {
        self.spins.iter().map(|&s| s as f64).sum::<f64>() / self.spins.len() as f64
    }

    /// Mean bond energy per site (`J = 1`, no external field).
    pub fn energy(&self) -> f64 {
        let mut total = 0f64;
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let s = self.spins[self.idx(row, col)] as f64;
                total -= s * self.nn_sum(row, col) as f64;
            }
        }
        // Each bond was counted twice (once from each endpoint).
        total / (2.0 * self.spins.len() as f64)
    }
}

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
