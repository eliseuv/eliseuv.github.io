use spin_lattice::{IsingLattice, MetropolisSampler};
use wasm_bindgen::prelude::*;

fn uniform() -> f64 {
    js_sys::Math::random()
}

/// wasm-bindgen binding around `spin_lattice`'s Ising lattice and
/// Metropolis sampler, wiring their RNG closures to `js_sys::Math::random`.
#[wasm_bindgen]
pub struct IsingModel {
    lattice: IsingLattice,
    sampler: MetropolisSampler,
}

#[wasm_bindgen]
impl IsingModel {
    /// New model on an `nrows` x `ncols` lattice at the given temperature,
    /// starting from a random (infinite-temperature) configuration.
    pub fn new(nrows: usize, ncols: usize, temperature: f64) -> IsingModel {
        console_error_panic_hook::set_once();

        IsingModel {
            lattice: IsingLattice::new_random(nrows, ncols, uniform),
            sampler: MetropolisSampler::with_temperature(temperature),
        }
    }

    pub fn nrows(&self) -> usize {
        self.lattice.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.lattice.ncols()
    }

    /// Pointer to the spin buffer in WASM linear memory, row-major, one
    /// `i8` (`+1`/`-1`) per site.
    pub fn spins(&self) -> *const i8 {
        self.lattice.spins().as_ptr()
    }

    pub fn temperature(&self) -> f64 {
        self.sampler.temperature()
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.sampler.set_temperature(temperature);
    }

    /// Reset to a random (infinite-temperature) configuration.
    pub fn randomize(&mut self) {
        self.lattice.randomize(uniform);
    }

    /// Mean spin per site, in `[-1, 1]`.
    pub fn magnetization(&self) -> f64 {
        self.lattice.magnetization()
    }

    /// Mean bond energy per site (`J = 1`, no external field).
    pub fn energy(&self) -> f64 {
        self.lattice.energy()
    }

    /// One Metropolis sweep: `nrows * ncols` single-spin-flip attempts on
    /// randomly chosen sites.
    pub fn step(&mut self) {
        self.sampler.step(
            &mut self.lattice,
            |nrows, ncols| {
                let row = (js_sys::Math::random() * nrows as f64) as usize % nrows;
                let col = (js_sys::Math::random() * ncols as f64) as usize % ncols;
                (row, col)
            },
            uniform,
        );
    }
}
