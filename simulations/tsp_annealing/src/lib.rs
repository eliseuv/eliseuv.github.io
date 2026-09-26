use annealing::cooling::GeometricSchedule;
use annealing::tsp::{CorrelatedBase, Domain, Move, TspAnnealer};
use wasm_bindgen::prelude::*;

fn uniform() -> f64 {
    js_sys::Math::random()
}

/// Domain names as used by the page's `<select>`.
fn parse_domain(kind: &str, rho: f64, gamma: f64, x0: f64) -> Domain {
    match kind {
        "uniform" => Domain::Uniform,
        "correlated-uniform" => Domain::Correlated {
            rho,
            base: CorrelatedBase::Uniform,
        },
        "correlated-normal" => Domain::Correlated {
            rho,
            base: CorrelatedBase::Normal,
        },
        "power-law" => Domain::PowerLaw { gamma, x0 },
        _ => panic!("unknown domain {kind:?}"),
    }
}

fn parse_move(kind: &str) -> Move {
    match kind {
        "swap" => Move::Swap,
        "2-opt" => Move::TwoOpt,
        _ => panic!("unknown move {kind:?}"),
    }
}

/// wasm-bindgen binding around `annealing::tsp`, wiring its
/// RNG closure to `js_sys::Math::random`.
#[wasm_bindgen]
pub struct TspSimulation {
    annealer: TspAnnealer,
}

#[wasm_bindgen]
impl TspSimulation {
    /// `n` cities from `domain` (see `randomize`), annealed from a random
    /// cycle at `initial_temperature`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        n: usize,
        domain: &str,
        rho: f64,
        gamma: f64,
        x0: f64,
        move_kind: &str,
        initial_temperature: f64,
        final_temperature: f64,
        alpha: f64,
        iterations_per_step: usize,
    ) -> TspSimulation {
        console_error_panic_hook::set_once();

        let points = parse_domain(domain, rho, gamma, x0).sample(n, uniform);
        let schedule = GeometricSchedule::new(
            initial_temperature,
            final_temperature,
            alpha,
            iterations_per_step,
        );
        TspSimulation {
            annealer: TspAnnealer::new(points, schedule, parse_move(move_kind), uniform),
        }
    }

    /// New cities, then `anneal`. `domain` is one of `"uniform"`,
    /// `"correlated-uniform"`, `"correlated-normal"` (using `rho`) or
    /// `"power-law"` (using `gamma` and `x0`).
    pub fn randomize(&mut self, n: usize, domain: &str, rho: f64, gamma: f64, x0: f64) {
        let points = parse_domain(domain, rho, gamma, x0).sample(n, uniform);
        self.annealer.set_points(points, uniform);
    }

    /// Restart from a random cycle at `T₀` on the same cities.
    pub fn anneal(&mut self) {
        self.annealer.restart(uniform);
    }

    /// `"swap"` or `"2-opt"`.
    pub fn set_move(&mut self, move_kind: &str) {
        self.annealer.move_kind = parse_move(move_kind);
    }

    pub fn set_initial_temperature(&mut self, temperature: f64) {
        self.annealer.schedule.initial_temperature = temperature;
    }

    pub fn set_final_temperature(&mut self, temperature: f64) {
        self.annealer.schedule.final_temperature = temperature;
    }

    pub fn set_alpha(&mut self, alpha: f64) {
        self.annealer.schedule.alpha = alpha;
    }

    pub fn set_iterations_per_step(&mut self, iterations: usize) {
        self.annealer.schedule.iterations_per_step = iterations;
    }

    /// Manual override of the current temperature.
    pub fn set_temperature(&mut self, temperature: f64) {
        self.annealer.schedule.set_temperature(temperature);
    }

    pub fn set_hold(&mut self, hold: bool) {
        self.annealer.schedule.hold = hold;
    }

    /// Run `n_moves` Metropolis iterations.
    pub fn advance(&mut self, n_moves: usize) {
        self.annealer.sweep(n_moves, uniform);
    }

    pub fn temperature(&self) -> f64 {
        self.annealer.schedule.temperature()
    }

    pub fn annealing_step(&self) -> usize {
        self.annealer.schedule.step()
    }

    pub fn is_frozen(&self) -> bool {
        self.annealer.schedule.is_frozen()
    }

    pub fn cost(&self) -> f64 {
        self.annealer.tour().cost()
    }

    pub fn best_cost(&self) -> f64 {
        self.annealer.best_cost()
    }

    pub fn expected_random_cost(&self) -> f64 {
        self.annealer.expected_random_cost()
    }

    /// Fraction of moves accepted since the previous call.
    pub fn take_acceptance_rate(&mut self) -> f64 {
        self.annealer.take_acceptance_rate()
    }

    pub fn city_count(&self) -> usize {
        self.annealer.tour().len()
    }

    pub fn points_x(&self) -> Vec<f64> {
        self.annealer.tour().points().iter().map(|p| p[0]).collect()
    }

    pub fn points_y(&self) -> Vec<f64> {
        self.annealer.tour().points().iter().map(|p| p[1]).collect()
    }

    /// City indices in visiting order.
    pub fn tour(&self) -> Vec<u32> {
        self.annealer
            .tour()
            .order()
            .iter()
            .map(|&i| i as u32)
            .collect()
    }
}
