use ndarray::Array2;
use std::fmt;
use wasm_bindgen::prelude::*;

// Double buffered 2D lattice
#[derive(Debug)]
pub struct Lattice2D<T> {
    pub buffer: Array2<T>,
    buffer_next: Array2<T>,
}

impl<T> Lattice2D<T>
where
    T: Clone,
{
    // Constructor
    pub fn new(nrows: usize, ncols: usize, cell_state: T) -> Lattice2D<T> {
        let buffer = Array2::from_elem((nrows, ncols), cell_state);
        let buffer_next = buffer.clone();

        Lattice2D::<T> {
            buffer,
            buffer_next,
        }
    }

    // Get shape
    pub fn nrows(&self) -> usize {
        self.buffer.nrows()
    }
    pub fn ncols(&self) -> usize {
        self.buffer.ncols()
    }

    // Swap buffers
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.buffer, &mut self.buffer_next)
    }

    // Set lattice to state
    pub fn set_constant(&mut self, state: &T) {
        for site in self.buffer.iter_mut() {
            *site = state.clone();
        }
    }
}

// Single cell state
#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// Cell state methods
impl Cell {
    // Toggle cell dead/alive
    fn toggle(&mut self) {
        *self = match &self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

// Game of life universe
#[wasm_bindgen]
pub struct Universe {
    lattice: Lattice2D<Cell>,
}

// Methods NOT accessible by JS
impl Universe {
    // Calculate the number of live neighbors of a given cell
    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let buffer = &self.lattice.buffer;
        let mut count = 0;
        for delta_row in [self.lattice.nrows() - 1, 0, 1].iter().cloned() {
            for delta_col in [self.lattice.ncols() - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.lattice.nrows();
                let neighbor_col = (col + delta_col) % self.lattice.ncols();
                count += *buffer.get((neighbor_row, neighbor_col)).unwrap() as u8;
            }
        }
        count
    }

    // Set a given set of cell to alive
    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for idx in cells.iter().cloned() {
            *self.lattice.buffer.get_mut(idx).unwrap() = Cell::Alive;
        }
    }
}

// Methods accessible by JS
#[wasm_bindgen]
impl Universe {
    // System size
    pub fn nrows(&self) -> usize {
        self.lattice.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.lattice.ncols()
    }

    // Get pointer to state in WASM linear memory
    pub fn state(&self) -> *const Cell {
        self.lattice.buffer.as_ptr()
    }

    // Clear state
    pub fn clear(&mut self) {
        self.lattice.set_constant(&Cell::Dead)
    }

    // Randomize state
    pub fn randomize(&mut self, p: f64) {
        for cell in self.lattice.buffer.iter_mut() {
            *cell = if js_sys::Math::random() < p {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }
    }

    // Toggle a cell dead/alive
    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        self.lattice.buffer.get_mut((row, col)).unwrap().toggle();
    }

    // Add pattern
    pub fn add_pattern(&mut self, pattern: Pattern, row_center: usize, col_center: usize) {
        // Adding `nrows`/`ncols` before the modulo avoids underflow when
        // `row_center`/`col_center` is smaller than a template offset,
        // since these are unsigned coordinates on a toroidal grid.
        let template: Vec<(usize, usize)> = get_template(pattern)
            .iter()
            .map(|(y, x)| {
                (
                    (row_center + y + self.lattice.nrows()) % self.lattice.nrows(),
                    (col_center + x + self.lattice.ncols()) % self.lattice.ncols(),
                )
            })
            .collect();

        for idx in template {
            *self.lattice.buffer.get_mut(idx).unwrap() = Cell::Alive;
        }
    }

    // Update the whole universe
    pub fn tick(&mut self) {
        // Loop on sites
        for row in 0..self.lattice.nrows() {
            for col in 0..self.lattice.ncols() {
                let idx = (row, col);
                let cell_current = self.lattice.buffer.get(idx).unwrap();
                let live_neighbors = self.live_neighbor_count(row, col);

                // Determine next cell state
                let cell_next = match (cell_current, live_neighbors) {
                    // Starvation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state
                    (&state, _) => state,
                };

                // Store new state in buffer
                *self.lattice.buffer_next.get_mut(idx).unwrap() = cell_next;
            }
        }

        self.lattice.swap_buffers();
    }

    // Constructor set state
    pub fn new(nrows: usize, ncols: usize, cell_state: Option<Cell>) -> Universe {
        console_error_panic_hook::set_once();

        Universe {
            lattice: Lattice2D::<Cell>::new(nrows, ncols, cell_state.unwrap_or(Cell::Dead)),
        }
    }

    // Simple render method
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.lattice.buffer.rows() {
            for &cell in row {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
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
