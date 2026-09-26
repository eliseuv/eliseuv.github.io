//! Simulated annealing: a Metropolis chain whose temperature is lowered
//! according to a cooling schedule.
//!
//! Dependency-free: callers supply randomness through closures
//! (`js_sys::Math::random` behind the wasm bindings).

pub mod tsp;

mod random;

/// Metropolis acceptance of a move changing the energy by `delta` at
/// `temperature`: always accept downhill moves, accept uphill ones with
/// probability `exp(−delta/T)`. At `T ≤ 0` only downhill moves pass.
/// `uniform` must yield values in `[0, 1)` and is only drawn from for
/// uphill moves.
pub fn metropolis_accept(delta: f64, temperature: f64, mut uniform: impl FnMut() -> f64) -> bool {
    if delta <= 0.0 {
        return true;
    }
    if temperature <= 0.0 {
        return false;
    }
    uniform() < (-delta / temperature).exp()
}

/// Geometric cooling `T_t = αᵗ T₀`, lowering the temperature once every
/// `iterations_per_step` Metropolis iterations until `final_temperature`
/// is reached, where it stays.
///
/// Parameters are freely mutable mid-run. Changing them doesn't rewind
/// the schedule: cooling just continues from the current temperature.
#[derive(Clone, Debug)]
pub struct GeometricSchedule {
    pub initial_temperature: f64,
    pub final_temperature: f64,
    /// Cooling factor, in `(0, 1)`.
    pub alpha: f64,
    pub iterations_per_step: usize,
    /// While set, the temperature is held fixed.
    pub hold: bool,
    temperature: f64,
    step: usize,
    iteration: usize,
}

impl GeometricSchedule {
    pub fn new(
        initial_temperature: f64,
        final_temperature: f64,
        alpha: f64,
        iterations_per_step: usize,
    ) -> Self {
        Self {
            initial_temperature,
            final_temperature,
            alpha,
            iterations_per_step,
            hold: false,
            temperature: initial_temperature,
            step: 0,
            iteration: 0,
        }
    }

    /// Back to `T₀` at step 0.
    pub fn restart(&mut self) {
        self.temperature = self.initial_temperature;
        self.step = 0;
        self.iteration = 0;
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Manual override: jump to `temperature` and cool from there.
    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.iteration = 0;
    }

    /// Number of cooling steps taken since the last restart.
    pub fn step(&self) -> usize {
        self.step
    }

    pub fn is_frozen(&self) -> bool {
        self.temperature <= self.final_temperature
    }

    /// Account for one Metropolis iteration, cooling if a step completed.
    pub fn tick(&mut self) {
        if self.hold || self.is_frozen() {
            return;
        }
        self.iteration += 1;
        if self.iteration >= self.iterations_per_step {
            self.iteration = 0;
            self.step += 1;
            // Clamped so the run ends exactly at `T_f` instead of
            // undershooting by up to a factor α.
            self.temperature = (self.temperature * self.alpha).max(self.final_temperature);
        }
    }
}
