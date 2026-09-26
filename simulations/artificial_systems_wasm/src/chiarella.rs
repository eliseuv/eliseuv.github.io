//! Extended Chiarella model (Majewski, Ciliberti & Bouchaud 2020): a
//! log-price driven by fundamentalists, trend followers and noise traders,
//!
//! ```text
//! dV = g dt + σ_V dW₁
//! dp = κ(V − p) dt + β tanh(γM) dt + σ_N dW₂
//! dM = α(dp − M dt)
//! ```
//!
//! where `M` is an exponential moving average of realized price changes.

use crate::random::standard_normal_pair;

/// Model parameters, all freely mutable between steps.
#[derive(Clone, Copy, Debug)]
pub struct ChiarellaParams {
    /// Fundamentalist mean-reversion strength.
    pub kappa: f64,
    /// Trend-follower maximum demand.
    pub beta: f64,
    /// Trend-follower sensitivity to the trend signal.
    pub gamma: f64,
    /// Inverse memory time of the trend signal's moving average.
    pub alpha: f64,
    /// Noise-trader volatility.
    pub sigma_noise: f64,
    /// Fundamental-value volatility.
    pub sigma_fundamental: f64,
    /// Fundamental-value drift.
    pub drift: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChiarellaState {
    pub price: f64,
    pub fundamental: f64,
    pub trend: f64,
}

pub struct ChiarellaModel {
    pub params: ChiarellaParams,
    state: ChiarellaState,
}

impl ChiarellaModel {
    /// New model at `p = V = M = 0`.
    pub fn new(params: ChiarellaParams) -> Self {
        Self {
            params,
            state: ChiarellaState::default(),
        }
    }

    pub fn reset(&mut self) {
        self.state = ChiarellaState::default();
    }

    pub fn state(&self) -> ChiarellaState {
        self.state
    }

    pub fn price(&self) -> f64 {
        self.state.price
    }

    pub fn fundamental(&self) -> f64 {
        self.state.fundamental
    }

    pub fn trend(&self) -> f64 {
        self.state.trend
    }

    /// `δ = p − V`.
    pub fn mispricing(&self) -> f64 {
        self.state.price - self.state.fundamental
    }

    /// Whether the noiseless dynamics around `(δ, M) = (0, 0)` are past
    /// the Hopf bifurcation, i.e. `βγ > 1 + κ/α` (the linearization's trace
    /// turns positive while its determinant `ακ` stays positive).
    pub fn is_oscillatory(&self) -> bool {
        let p = &self.params;
        p.beta * p.gamma > 1.0 + p.kappa / p.alpha
    }

    /// One Euler–Maruyama step of size `dt`. `uniform` must yield values in
    /// `[0, 1)`; two draws per step feed one Box–Muller pair, which covers
    /// both Wiener increments.
    pub fn step(&mut self, dt: f64, uniform: impl FnMut() -> f64) {
        let p = &self.params;
        let s = &mut self.state;
        let (z_fundamental, z_noise) = standard_normal_pair(uniform);
        let sqrt_dt = dt.sqrt();

        let d_price = p.kappa * (s.fundamental - s.price) * dt
            + p.beta * (p.gamma * s.trend).tanh() * dt
            + p.sigma_noise * sqrt_dt * z_noise;
        let d_fundamental = p.drift * dt + p.sigma_fundamental * sqrt_dt * z_fundamental;

        // The trend signal averages the *realized* increment, noise
        // included — that is what trend followers actually observe.
        s.trend += p.alpha * (d_price - s.trend * dt);
        s.price += d_price;
        s.fundamental += d_fundamental;
    }
}
