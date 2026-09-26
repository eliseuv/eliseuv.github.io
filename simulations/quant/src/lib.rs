//! Agent-based and stochastic models of financial markets.
//!
//! Dependency-free, like `annealing`: callers supply randomness through
//! closures (`js_sys::Math::random` behind the wasm bindings).

pub mod chiarella;

mod random;
