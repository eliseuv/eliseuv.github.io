use quant::chiarella::{ChiarellaModel, ChiarellaParams};
use wasm_bindgen::prelude::*;

fn uniform() -> f64 {
    js_sys::Math::random()
}

/// wasm-bindgen binding around `quant::chiarella`,
/// wiring its RNG closure to `js_sys::Math::random`.
#[wasm_bindgen]
pub struct ChiarellaSimulation {
    model: ChiarellaModel,
    dt: f64,
}

#[wasm_bindgen]
impl ChiarellaSimulation {
    /// New simulation at `p = V = M = 0`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        kappa: f64,
        beta: f64,
        gamma: f64,
        alpha: f64,
        sigma_noise: f64,
        sigma_fundamental: f64,
        drift: f64,
        dt: f64,
    ) -> ChiarellaSimulation {
        console_error_panic_hook::set_once();

        ChiarellaSimulation {
            model: ChiarellaModel::new(ChiarellaParams {
                kappa,
                beta,
                gamma,
                alpha,
                sigma_noise,
                sigma_fundamental,
                drift,
            }),
            dt,
        }
    }

    pub fn set_kappa(&mut self, kappa: f64) {
        self.model.params.kappa = kappa;
    }

    pub fn set_beta(&mut self, beta: f64) {
        self.model.params.beta = beta;
    }

    pub fn set_gamma(&mut self, gamma: f64) {
        self.model.params.gamma = gamma;
    }

    pub fn set_alpha(&mut self, alpha: f64) {
        self.model.params.alpha = alpha;
    }

    pub fn set_sigma_noise(&mut self, sigma_noise: f64) {
        self.model.params.sigma_noise = sigma_noise;
    }

    pub fn set_sigma_fundamental(&mut self, sigma_fundamental: f64) {
        self.model.params.sigma_fundamental = sigma_fundamental;
    }

    pub fn set_drift(&mut self, drift: f64) {
        self.model.params.drift = drift;
    }

    pub fn set_dt(&mut self, dt: f64) {
        self.dt = dt;
    }

    /// Reset to `p = V = M = 0`, keeping parameters.
    pub fn reset(&mut self) {
        self.model.reset();
    }

    /// Run `n_steps` Euler–Maruyama steps.
    pub fn advance(&mut self, n_steps: usize) {
        for _ in 0..n_steps {
            self.model.step(self.dt, uniform);
        }
    }

    pub fn price(&self) -> f64 {
        self.model.price()
    }

    pub fn fundamental(&self) -> f64 {
        self.model.fundamental()
    }

    pub fn trend(&self) -> f64 {
        self.model.trend()
    }

    pub fn mispricing(&self) -> f64 {
        self.model.mispricing()
    }

    pub fn is_oscillatory(&self) -> bool {
        self.model.is_oscillatory()
    }
}
