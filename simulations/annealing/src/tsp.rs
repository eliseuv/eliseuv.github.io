//! Euclidean traveling salesman problem on random two-dimensional
//! domains, solved by simulated annealing over Hamiltonian cycles.
//!
//! The cost of a cycle plays the role of the energy. Both moves only
//! replace two edges of the cycle, so the cost change of a candidate is
//! computed in `O(1)` and the cost itself is tracked incrementally.

use crate::cooling::{metropolis_accept, GeometricSchedule};
use crate::random::standard_normal_pair;

pub type Point = [f64; 2];

/// Distribution of each of the two independent variables `z₁, z₂` that
/// are mixed into correlated coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CorrelatedBase {
    /// `U[−½, ½]`.
    Uniform,
    /// `N(0, 1/12)`: same mean and variance as `Uniform`.
    Normal,
}

/// Random domains on which the cities are placed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Domain {
    /// Uniform on the unit square.
    Uniform,
    /// Coordinates with correlation `rho`, each with the mean and variance
    /// of `base` regardless of `rho`.
    Correlated { rho: f64, base: CorrelatedBase },
    /// Each coordinate independently drawn from the two-tailed power law
    /// `p(x) ∝ |x|^−γ` for `|x| ≥ x₀`. Needs `gamma > 1`; for `gamma ≤ 3`
    /// the variance is infinite.
    PowerLaw { gamma: f64, x0: f64 },
}

impl Domain {
    pub fn sample(&self, n: usize, mut uniform: impl FnMut() -> f64) -> Vec<Point> {
        (0..n).map(|_| self.sample_point(&mut uniform)).collect()
    }

    fn sample_point(&self, mut uniform: impl FnMut() -> f64) -> Point {
        match *self {
            Domain::Uniform => [uniform(), uniform()],
            Domain::Correlated { rho, base } => {
                let (z1, z2) = match base {
                    CorrelatedBase::Uniform => (uniform() - 0.5, uniform() - 0.5),
                    CorrelatedBase::Normal => {
                        let sigma = 12f64.sqrt().recip();
                        let (z1, z2) = standard_normal_pair(&mut uniform);
                        (sigma * z1, sigma * z2)
                    }
                };
                // Unit-norm mixing weights keep the variance of z, and
                // their overlap sin 2φ is the correlation.
                let phi = 0.5 * rho.clamp(-1.0, 1.0).asin();
                let (sin, cos) = phi.sin_cos();
                [z1 * sin + z2 * cos, z1 * cos + z2 * sin]
            }
            Domain::PowerLaw { gamma, x0 } => {
                let mut coordinate = || {
                    let sign = if uniform() < 0.5 { -1.0 } else { 1.0 };
                    // Inverse CDF of one tail; `1 − u` lies in `(0, 1]`.
                    sign * x0 * (1.0 - uniform()).powf((1.0 - gamma).recip())
                };
                [coordinate(), coordinate()]
            }
        }
    }
}

fn distance(a: Point, b: Point) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

/// Uniform index in `0..n`.
fn random_index(n: usize, mut uniform: impl FnMut() -> f64) -> usize {
    ((uniform() * n as f64) as usize).min(n - 1)
}

/// A Hamiltonian cycle visiting the cities in `order`.
pub struct Tour {
    points: Vec<Point>,
    order: Vec<usize>,
    cost: f64,
}

impl Tour {
    /// Cycle visiting `points` in their given order. Needs at least 4
    /// points, the smallest size for which both moves are well defined.
    pub fn new(points: Vec<Point>) -> Self {
        assert!(points.len() >= 4, "a tour needs at least 4 cities");
        let order = (0..points.len()).collect();
        let mut tour = Self {
            points,
            order,
            cost: 0.0,
        };
        tour.recompute_cost();
        tour
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// City indices in visiting order.
    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn cost(&self) -> f64 {
        self.cost
    }

    /// Uniformly random cycle (Fisher–Yates).
    pub fn shuffle(&mut self, mut uniform: impl FnMut() -> f64) {
        for i in (1..self.order.len()).rev() {
            let j = random_index(i + 1, &mut uniform);
            self.order.swap(i, j);
        }
        self.recompute_cost();
    }

    /// Recompute the cost from scratch, discarding accumulated rounding.
    pub fn recompute_cost(&mut self) {
        let n = self.len();
        self.cost = (0..n).map(|i| self.edge_length(i, (i + 1) % n)).sum();
    }

    /// Expected cost of a uniformly random cycle on these cities: `N`
    /// edges times the mean distance between two distinct cities,
    /// `⟨C₀⟩ = 2/(N−1) Σ_{i<j} d(i, j)`. `O(N²)`.
    pub fn expected_random_cost(&self) -> f64 {
        let n = self.len();
        let total: f64 = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .map(|(i, j)| distance(self.points[i], self.points[j]))
            .sum();
        2.0 * total / (n - 1) as f64
    }

    /// Distance between the cities at cycle positions `a` and `b`.
    fn edge_length(&self, a: usize, b: usize) -> f64 {
        distance(self.points[self.order[a]], self.points[self.order[b]])
    }
}

/// Candidate move: reverse the cyclic stretch of positions `first..=last`,
/// changing the cost by `delta`.
#[derive(Clone, Copy, Debug)]
pub struct Proposal {
    pub first: usize,
    pub last: usize,
    pub delta: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Move {
    /// Exchange two cities adjacent in the cycle.
    Swap,
    /// Reverse a stretch of the cycle (2-opt): cut two edges and reconnect
    /// the two paths the other way around.
    TwoOpt,
}

impl Move {
    pub fn propose(&self, tour: &Tour, mut uniform: impl FnMut() -> f64) -> Proposal {
        let n = tour.len();
        let first = random_index(n, &mut uniform);
        // Stretches longer than half the cycle are covered by reversing
        // their complement instead, which gives the same cycle.
        let span = match self {
            Move::Swap => 1,
            Move::TwoOpt => 1 + random_index(n / 2 - 1, &mut uniform),
        };
        let last = (first + span) % n;
        let before = (first + n - 1) % n;
        let after = (last + 1) % n;
        // Swapping adjacent cities is reversing a stretch of two, so both
        // moves exchange edges before–first and last–after for
        // before–last and first–after.
        let delta = tour.edge_length(before, last) + tour.edge_length(first, after)
            - tour.edge_length(before, first)
            - tour.edge_length(last, after);
        Proposal { first, last, delta }
    }

    pub fn apply(&self, tour: &mut Tour, proposal: Proposal) {
        let Proposal { first, last, delta } = proposal;
        let n = tour.len();
        // Reversed in place with wrapped indices rather than reversing the
        // complement when the stretch crosses the end of `order`, so the
        // work stays at most N/4 swaps.
        let length = (last + n - first) % n + 1;
        for offset in 0..length / 2 {
            tour.order
                .swap((first + offset) % n, (first + length - 1 - offset) % n);
        }
        tour.cost += delta;
    }
}

/// Simulated annealing of a tour with Metropolis sampling.
pub struct TspAnnealer {
    tour: Tour,
    pub schedule: GeometricSchedule,
    pub move_kind: Move,
    expected_random_cost: f64,
    best_cost: f64,
    accepted: usize,
    attempted: usize,
}

impl TspAnnealer {
    /// Anneal a random cycle over `points`.
    pub fn new(
        points: Vec<Point>,
        schedule: GeometricSchedule,
        move_kind: Move,
        uniform: impl FnMut() -> f64,
    ) -> Self {
        let tour = Tour::new(points);
        let expected_random_cost = tour.expected_random_cost();
        let mut annealer = Self {
            tour,
            schedule,
            move_kind,
            expected_random_cost,
            best_cost: f64::INFINITY,
            accepted: 0,
            attempted: 0,
        };
        annealer.restart(uniform);
        annealer
    }

    /// New cities, then `restart`.
    pub fn set_points(&mut self, points: Vec<Point>, uniform: impl FnMut() -> f64) {
        self.tour = Tour::new(points);
        self.expected_random_cost = self.tour.expected_random_cost();
        self.restart(uniform);
    }

    /// Start over from a random cycle at `T₀`.
    pub fn restart(&mut self, uniform: impl FnMut() -> f64) {
        self.tour.shuffle(uniform);
        self.schedule.restart();
        self.best_cost = self.tour.cost();
        self.accepted = 0;
        self.attempted = 0;
    }

    pub fn tour(&self) -> &Tour {
        &self.tour
    }

    pub fn expected_random_cost(&self) -> f64 {
        self.expected_random_cost
    }

    pub fn best_cost(&self) -> f64 {
        self.best_cost
    }

    /// Fraction of moves accepted since the previous call (0 if none were
    /// attempted), resetting the count.
    pub fn take_acceptance_rate(&mut self) -> f64 {
        let rate = if self.attempted == 0 {
            0.0
        } else {
            self.accepted as f64 / self.attempted as f64
        };
        self.accepted = 0;
        self.attempted = 0;
        rate
    }

    /// Run `n_moves` Metropolis iterations, cooling as scheduled.
    pub fn sweep(&mut self, n_moves: usize, mut uniform: impl FnMut() -> f64) {
        for _ in 0..n_moves {
            let proposal = self.move_kind.propose(&self.tour, &mut uniform);
            if metropolis_accept(proposal.delta, self.schedule.temperature(), &mut uniform) {
                self.move_kind.apply(&mut self.tour, proposal);
                self.accepted += 1;
                self.best_cost = self.best_cost.min(self.tour.cost());
            }
            self.attempted += 1;
            self.schedule.tick();
        }
        // Incremental updates drift by rounding; resync once per batch.
        self.tour.recompute_cost();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic `[0, 1)` source (64-bit LCG, top 53 bits).
    fn lcg(seed: u64) -> impl FnMut() -> f64 {
        let mut state = seed;
        move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    fn assert_is_permutation(order: &[usize]) {
        let mut sorted = order.to_vec();
        sorted.sort_unstable();
        assert!(sorted.iter().copied().eq(0..order.len()));
    }

    fn check_incremental_cost(move_kind: Move) {
        let mut uniform = lcg(7);
        for n in [4, 5, 8, 33] {
            let mut tour = Tour::new(Domain::Uniform.sample(n, &mut uniform));
            tour.shuffle(&mut uniform);
            for _ in 0..10_000 {
                let proposal = move_kind.propose(&tour, &mut uniform);
                move_kind.apply(&mut tour, proposal);
            }
            let incremental = tour.cost();
            tour.recompute_cost();
            assert!((incremental - tour.cost()).abs() < 1e-9, "n = {n}");
            assert_is_permutation(tour.order());
        }
    }

    #[test]
    fn swap_tracks_cost_incrementally() {
        check_incremental_cost(Move::Swap);
    }

    #[test]
    fn two_opt_tracks_cost_incrementally() {
        check_incremental_cost(Move::TwoOpt);
    }

    #[test]
    fn expected_random_cost_matches_unit_square() {
        let n = 2000;
        let tour = Tour::new(Domain::Uniform.sample(n, lcg(3)));
        let sqrt2 = 2f64.sqrt();
        let exact = n as f64 / 15.0 * (2.0 + sqrt2 + 5.0 * (sqrt2 + 1.0).ln());
        let relative_error = (tour.expected_random_cost() / exact - 1.0).abs();
        assert!(relative_error < 0.02, "relative error {relative_error}");
    }

    #[test]
    fn correlated_domain_has_requested_correlation() {
        for base in [CorrelatedBase::Uniform, CorrelatedBase::Normal] {
            let rho = 0.6;
            let points = Domain::Correlated { rho, base }.sample(100_000, lcg(11));
            let n = points.len() as f64;
            let mean = |k: usize| points.iter().map(|p| p[k]).sum::<f64>() / n;
            let (mx, my) = (mean(0), mean(1));
            let moment = |f: &dyn Fn(&Point) -> f64| points.iter().map(f).sum::<f64>() / n;
            let cov = moment(&|p| (p[0] - mx) * (p[1] - my));
            let var_x = moment(&|p| (p[0] - mx).powi(2));
            let var_y = moment(&|p| (p[1] - my).powi(2));
            let measured = cov / (var_x * var_y).sqrt();
            assert!((measured - rho).abs() < 0.02, "{base:?}: {measured}");
        }
    }

    #[test]
    fn annealing_lowers_cost() {
        let mut uniform = lcg(5);
        let schedule = GeometricSchedule::new(1.0, 1e-4, 0.9, 1000);
        let points = Domain::Uniform.sample(64, &mut uniform);
        let mut annealer = TspAnnealer::new(points, schedule, Move::TwoOpt, &mut uniform);
        while !annealer.schedule.is_frozen() {
            annealer.sweep(1000, &mut uniform);
        }
        let ratio = annealer.tour().cost() / annealer.expected_random_cost();
        assert!(ratio < 0.25, "C/⟨C₀⟩ = {ratio}");
        assert_is_permutation(annealer.tour().order());
    }
}
