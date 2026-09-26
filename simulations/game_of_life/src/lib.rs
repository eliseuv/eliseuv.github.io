use std::fmt;
use std::sync::Arc;

use artificial_systems::automaton::{LifeLike, Synchronous};
use artificial_systems::dynamics::Dynamics;
use artificial_systems::rng::{entropy_seed, stream, DefaultRng};
use artificial_systems::site::Binary;
use artificial_systems::state::{Init, LatticeState, Prepare};
use artificial_systems::topology::Moore;
use wasm_bindgen::prelude::*;

// Single cell state, laid out like `Binary` so the lattice buffer can be
// handed to JS as-is
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl From<Cell> for Binary {
    fn from(cell: Cell) -> Self {
        Binary::from(cell == Cell::Alive)
    }
}

// Game of life universe: Conway's rule applied synchronously on a periodic
// Moore lattice
#[wasm_bindgen]
pub struct Universe {
    state: LatticeState<Binary, Moore>,
    dynamics: Synchronous<LifeLike, Binary>,
    rng: DefaultRng,
}

// Methods accessible by JS
#[wasm_bindgen]
impl Universe {
    // System size
    pub fn nrows(&self) -> usize {
        self.state.topology().lengths()[0]
    }

    pub fn ncols(&self) -> usize {
        self.state.topology().lengths()[1]
    }

    // Get pointer to state in WASM linear memory
    pub fn state(&self) -> *const Cell {
        // Sound: `Cell` and `Binary` are both `#[repr(u8)]` with 0 = dead/inactive
        self.state.sites().as_ptr() as *const Cell
    }

    // Clear state
    pub fn clear(&mut self) {
        self.state.fill(Binary::Inactive)
    }

    // Randomize state
    pub fn randomize(&mut self, p: f64) {
        Init::bernoulli(p, Binary::Active, Binary::Inactive)
            .prepare(&mut self.state, &mut self.rng);
    }

    // Toggle a cell dead/alive
    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let i = self.state.topology().index([row, col]);
        let toggled = Binary::from(!self.state.get(i).is_active());
        self.state.set(i, toggled);
    }

    // Add pattern
    pub fn add_pattern(&mut self, pattern: Pattern, row_center: usize, col_center: usize) {
        let (nrows, ncols) = (self.nrows(), self.ncols());
        let topology = self.state.topology().clone();
        // Adding `nrows`/`ncols` before the modulo avoids underflow when
        // `row_center`/`col_center` is smaller than a template offset,
        // since these are unsigned coordinates on a toroidal grid.
        for (y, x) in get_template(pattern) {
            let row = (row_center + y + nrows) % nrows;
            let col = (col_center + x + ncols) % ncols;
            self.state.set(topology.index([row, col]), Binary::Active);
        }
    }

    // Update the whole universe
    pub fn tick(&mut self) {
        self.dynamics.step(&mut self.state, &mut self.rng);
    }

    // Constructor set state
    pub fn new(nrows: usize, ncols: usize, cell_state: Option<Cell>) -> Universe {
        console_error_panic_hook::set_once();

        let topology = Arc::new(Moore::periodic([nrows, ncols]));
        let fill = cell_state.unwrap_or(Cell::Dead).into();
        Universe {
            state: LatticeState::uniform(topology, fill),
            dynamics: Synchronous::new(LifeLike::conway()),
            rng: stream(entropy_seed(), &[]),
        }
    }

    // Simple render method
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.state.sites().chunks(self.ncols()) {
            for &site in row {
                let symbol = if site.is_active() { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Common patterns
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Glider,
    Pulsar,
}

fn get_template(pattern: Pattern) -> Vec<(usize, usize)> {
    match pattern {
        Pattern::Glider => vec![(2, 2), (2, 1), (2, 0), (1, 2), (0, 1)],
        Pattern::Pulsar => vec![
            (0, 3),
            (1, 3),
            (2, 3),
            (2, 2),
            (3, 2),
            (3, 1),
            (3, 0),
            (0, 6),
            (1, 6),
            (2, 6),
            (2, 7),
            (3, 7),
            (3, 8),
            (3, 9),
            (6, 0),
            (6, 1),
            (6, 2),
            (7, 2),
            (7, 3),
            (8, 3),
            (9, 3),
            (6, 9),
            (6, 8),
            (6, 7),
            (7, 7),
            (7, 6),
            (8, 6),
            (9, 6),
        ],
    }
}
