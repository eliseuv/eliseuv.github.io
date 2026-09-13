use artificial_systems_wasm::contact_process::{ContactLattice, ContactSampler};
use wasm_bindgen::prelude::*;

fn uniform() -> f64 {
    js_sys::Math::random()
}

/// wasm-bindgen binding around `artificial_systems_wasm::contact_process`,
/// wiring its RNG closures to `js_sys::Math::random`.
#[wasm_bindgen]
pub struct ContactProcessModel {
    lattice: ContactLattice,
    sampler: ContactSampler,
}

#[wasm_bindgen]
impl ContactProcessModel {
    /// New model on an `nrows` x `ncols` lattice at the given healing
    /// probability, starting from a random (~50/50) configuration.
    pub fn new(nrows: usize, ncols: usize, p: f64) -> ContactProcessModel {
        console_error_panic_hook::set_once();

        ContactProcessModel {
            lattice: ContactLattice::new_random(nrows, ncols, uniform),
            sampler: ContactSampler::with_healing_probability(p),
        }
    }

    pub fn nrows(&self) -> usize {
        self.lattice.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.lattice.ncols()
    }

    /// Pointer to the active-site buffer in WASM linear memory, row-major,
    /// one `u8` (`1` = active, `0` = inactive) per site.
    pub fn active(&self) -> *const u8 {
        // Sound: `Binary` is `#[repr(u8)]`.
        self.lattice.active().as_ptr() as *const u8
    }

    pub fn healing_probability(&self) -> f64 {
        self.sampler.healing_probability()
    }

    pub fn set_healing_probability(&mut self, p: f64) {
        self.sampler.set_healing_probability(p);
    }

    /// Reset to a random (~50/50 active/inactive) configuration.
    pub fn randomize(&mut self) {
        self.lattice.randomize(uniform);
    }

    /// Clear to all-inactive and activate a single site at the center.
    pub fn seed_center(&mut self) {
        self.lattice.seed_center();
    }

    /// Fraction of sites currently active, in `[0, 1]`.
    pub fn active_fraction(&self) -> f64 {
        self.lattice.active_fraction()
    }

    /// One sweep: `nrows * ncols` attempts on randomly chosen sites.
    pub fn step(&mut self) {
        self.sampler.step(
            &mut self.lattice,
            |nrows, ncols| {
                let row = (js_sys::Math::random() * nrows as f64) as usize % nrows;
                let col = (js_sys::Math::random() * ncols as f64) as usize % ncols;
                (row, col)
            },
            || (js_sys::Math::random() * 4.0) as usize % 4,
            uniform,
        );
    }
}
