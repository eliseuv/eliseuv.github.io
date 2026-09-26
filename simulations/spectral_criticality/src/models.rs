//! Systems whose time series feed the correlation matrices, with the sizes used both live and in
//! the reference scans, so the two are directly comparable.

use std::sync::Arc;

use artificial_systems::automaton::{contact_process, ContactProcess};
use artificial_systems::dynamics::HeatBath;
use artificial_systems::model::Beg;
use artificial_systems::site::{Binary, SpinHalf};
use artificial_systems::state::{Init, LatticeState};
use artificial_systems::system::SpinSystem;
use artificial_systems::topology::{Chain, Square};

pub use artificial_systems::constants::{
    CONTACT_PROCESS_CHAIN_ALPHA_CRITICAL, ISING_SQUARE_T_CRITICAL,
};

/// Time series per correlation matrix.
pub const N_SAMPLES: usize = 100;

/// Side of the square Ising lattice. Small enough to sweep 100 lattices per frame in the
/// browser; the thesis finite size study found the spectral minimum settled from `L = 32` on.
pub const ISING_LENGTH: usize = 32;
/// Monte Carlo steps per Ising series after the initial measurement.
pub const ISING_N_STEPS: usize = 300;

/// Contact process chain length, as in the thesis.
pub const CONTACT_PROCESS_LENGTH: usize = 128;
/// Monte Carlo steps per contact process series after the initial measurement.
pub const CONTACT_PROCESS_N_STEPS: usize = 500;

pub type IsingSystem = SpinSystem<LatticeState<SpinHalf, Square>, Beg>;
pub type ContactProcessState = LatticeState<Binary, Chain>;

/// Ferromagnetic Ising model (`J = 1`, no field) on the periodic square lattice.
pub fn ising_system() -> IsingSystem {
    let topology = Arc::new(Square::periodic([ISING_LENGTH, ISING_LENGTH]));
    SpinSystem::new(
        LatticeState::uniform(topology, SpinHalf::Up),
        Beg::ising(1.0, 0.0),
    )
}

/// Random sequential heat bath dynamics at `t_over_tc` times the Onsager temperature.
pub fn ising_dynamics(t_over_tc: f64) -> HeatBath {
    HeatBath::at_temperature(t_over_tc * ISING_SQUARE_T_CRITICAL)
}

/// Ising chains start from infinite temperature, `m₀ ≈ 0`.
pub fn ising_init() -> Init<SpinHalf> {
    Init::IidUniform
}

/// Periodic chain of the contact process.
pub fn contact_process_state() -> ContactProcessState {
    let topology = Arc::new(Chain::periodic([CONTACT_PROCESS_LENGTH]));
    LatticeState::uniform(topology, Binary::Inactive)
}

/// Contact process with infection rate `alpha` and diffusion probability `gamma`.
pub fn contact_process_dynamics(alpha: f64, gamma: f64) -> ContactProcess {
    contact_process(alpha, gamma)
}

/// Contact process chains start fully active: there is no disordered phase to start from.
pub fn contact_process_init() -> Init<Binary> {
    Init::Uniform(Binary::Active)
}
