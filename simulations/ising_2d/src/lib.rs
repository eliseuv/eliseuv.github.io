use std::sync::Arc;

use artificial_systems::dynamics::{Dynamics, Metropolis};
use artificial_systems::model::Beg;
use artificial_systems::observable::{EnergyPerSite, Magnetization, Observable};
use artificial_systems::rng::{entropy_seed, stream, DefaultRng};
use artificial_systems::site::SpinHalf;
use artificial_systems::state::{Init, LatticeState, Prepare};
use artificial_systems::system::SpinSystem;
use artificial_systems::topology::Square;
use wasm_bindgen::prelude::*;

type IsingSystem = SpinSystem<LatticeState<SpinHalf, Square>, Beg>;

/// wasm-bindgen binding around `artificial_systems`' ferromagnetic Ising
/// model (`J = 1`, no field) on a periodic square lattice, sampled by
/// random sequential Metropolis dynamics.
#[wasm_bindgen]
pub struct IsingModel {
    system: IsingSystem,
    dynamics: Metropolis,
    rng: DefaultRng,
}

#[wasm_bindgen]
impl IsingModel {
    /// New model on an `nrows` x `ncols` lattice at the given temperature,
    /// starting from a random (infinite-temperature) configuration.
    pub fn new(nrows: usize, ncols: usize, temperature: f64) -> IsingModel {
        console_error_panic_hook::set_once();

        let topology = Arc::new(Square::periodic([nrows, ncols]));
        let state = LatticeState::uniform(topology, SpinHalf::Up);
        let mut model = IsingModel {
            system: SpinSystem::new(state, Beg::ising(1.0, 0.0)),
            dynamics: Metropolis::at_temperature(temperature),
            rng: stream(entropy_seed(), &[]),
        };
        model.randomize();
        model
    }

    pub fn nrows(&self) -> usize {
        self.system.state().topology().lengths()[0]
    }

    pub fn ncols(&self) -> usize {
        self.system.state().topology().lengths()[1]
    }

    /// Pointer to the spin buffer in WASM linear memory, row-major, one
    /// `i8` (`+1`/`-1`) per site.
    pub fn spins(&self) -> *const i8 {
        // Sound: `SpinHalf` is `#[repr(i8)]`.
        self.system.state().sites().as_ptr() as *const i8
    }

    pub fn temperature(&self) -> f64 {
        self.dynamics.beta().recip()
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.dynamics = Metropolis::at_temperature(temperature);
    }

    /// Reset to a random (infinite-temperature) configuration.
    pub fn randomize(&mut self) {
        Init::IidUniform.prepare(&mut self.system, &mut self.rng);
    }

    /// Mean spin per site, in `[-1, 1]`.
    pub fn magnetization(&self) -> f64 {
        Magnetization.measure(&self.system)
    }

    /// Mean bond energy per site (`J = 1`, no external field).
    pub fn energy(&self) -> f64 {
        EnergyPerSite.measure(&self.system)
    }

    /// One Metropolis sweep: `nrows * ncols` single-spin-flip attempts on
    /// randomly chosen sites.
    pub fn step(&mut self) {
        self.dynamics.step(&mut self.system, &mut self.rng);
    }
}
