use std::sync::Arc;

use artificial_systems::automaton::{Asynchronous, ContactRule};
use artificial_systems::dynamics::Dynamics;
use artificial_systems::observable::{Density, Observable};
use artificial_systems::rng::{entropy_seed, stream, DefaultRng};
use artificial_systems::site::Binary;
use artificial_systems::state::{Init, LatticeState, Position, Prepare};
use artificial_systems::topology::Square;
use wasm_bindgen::prelude::*;

/// Upstream parametrises the rule by the infection rate `α`, with healing
/// probability `1/α`; the page exposes the healing probability directly.
fn contact_rule(healing_probability: f64) -> ContactRule {
    ContactRule::new(healing_probability.recip())
}

/// wasm-bindgen binding around `artificial_systems`' contact process
/// (without diffusion) on a periodic square lattice, updated random
/// sequentially.
#[wasm_bindgen]
pub struct ContactProcessModel {
    state: LatticeState<Binary, Square>,
    dynamics: Asynchronous<ContactRule>,
    rng: DefaultRng,
}

#[wasm_bindgen]
impl ContactProcessModel {
    /// New model on an `nrows` x `ncols` lattice at the given healing
    /// probability, starting from a random (~50/50) configuration.
    pub fn new(nrows: usize, ncols: usize, p: f64) -> ContactProcessModel {
        console_error_panic_hook::set_once();

        let topology = Arc::new(Square::periodic([nrows, ncols]));
        let mut model = ContactProcessModel {
            state: LatticeState::uniform(topology, Binary::Inactive),
            dynamics: Asynchronous::new(contact_rule(p)),
            rng: stream(entropy_seed(), &[]),
        };
        model.randomize();
        model
    }

    pub fn nrows(&self) -> usize {
        self.state.topology().lengths()[0]
    }

    pub fn ncols(&self) -> usize {
        self.state.topology().lengths()[1]
    }

    /// Pointer to the active-site buffer in WASM linear memory, row-major,
    /// one `u8` (`1` = active, `0` = inactive) per site.
    pub fn active(&self) -> *const u8 {
        // Sound: `Binary` is `#[repr(u8)]`.
        self.state.sites().as_ptr() as *const u8
    }

    pub fn healing_probability(&self) -> f64 {
        self.dynamics.rule.alpha().recip()
    }

    pub fn set_healing_probability(&mut self, p: f64) {
        self.dynamics.rule = contact_rule(p);
    }

    /// Reset to a random (~50/50 active/inactive) configuration.
    pub fn randomize(&mut self) {
        Init::IidUniform.prepare(&mut self.state, &mut self.rng);
    }

    /// Clear to all-inactive and activate a single site at the center.
    pub fn seed_center(&mut self) {
        Init::Single {
            background: Binary::Inactive,
            value: Binary::Active,
            at: Position::Center,
        }
        .prepare(&mut self.state, &mut self.rng);
    }

    /// Fraction of sites currently active, in `[0, 1]`.
    pub fn active_fraction(&self) -> f64 {
        Density(Binary::Active).measure(&self.state)
    }

    /// One sweep: `nrows * ncols` attempts on randomly chosen sites.
    pub fn step(&mut self) {
        self.dynamics.step(&mut self.state, &mut self.rng);
    }
}
