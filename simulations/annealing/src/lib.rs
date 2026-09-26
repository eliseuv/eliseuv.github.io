//! Simulated annealing: a Metropolis chain whose temperature is lowered
//! according to a cooling schedule.
//!
//! Dependency-free: callers supply randomness through closures
//! (`js_sys::Math::random` behind the wasm bindings).

pub mod cooling;
pub mod tsp;

mod random;
