//! Lattice models, Markov Chain Monte Carlo sampling and simulated
//! annealing.
//!
//! Deliberately dependency-free (no RNG crate, no wasm-bindgen): callers
//! supply randomness through closures, so the same core runs unchanged
//! behind a wasm binding (using `js_sys::Math::random`) or in a native
//! context (using any `rand`-family generator). A lightweight,
//! wasm-scoped analog of a full research spin-system library, meant to
//! back every lattice simulation on the site.

pub mod annealing;
pub mod binary;
pub mod chiarella;
pub mod contact_process;
pub mod ising;
pub mod lattice;
pub mod mcmc;
pub mod random;
pub mod spin;
pub mod tsp;
