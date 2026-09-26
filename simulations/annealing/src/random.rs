//! Sampling helpers built on a caller-supplied uniform `[0, 1)` source.

/// Box–Muller transform: two independent standard normals.
pub fn standard_normal_pair(mut uniform: impl FnMut() -> f64) -> (f64, f64) {
    // `1 − u` maps `[0, 1)` onto `(0, 1]`, keeping `ln` finite.
    let radius = (-2.0 * (1.0 - uniform()).ln()).sqrt();
    let angle = std::f64::consts::TAU * uniform();
    (radius * angle.cos(), radius * angle.sin())
}
