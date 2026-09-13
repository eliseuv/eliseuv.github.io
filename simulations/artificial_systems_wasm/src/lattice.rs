//! Toroidal 2D square-lattice geometry and a generic per-site-state
//! container built on it.

/// Dimensions and row-major/toroidal-neighbor indexing for a 2D square
/// lattice with periodic (toroidal) boundary conditions. Holds no
/// per-site data itself — see [`LatticeState`] for that.
#[derive(Debug, Clone, Copy)]
pub struct Grid2D {
    nrows: usize,
    ncols: usize,
}

impl Grid2D {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self { nrows, ncols }
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn len(&self) -> usize {
        self.nrows * self.ncols
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Flat row-major index of `(row, col)`.
    pub fn idx(&self, row: usize, col: usize) -> usize {
        row * self.ncols + col
    }

    /// The four nearest-neighbor coordinates of `(row, col)`, wrapping
    /// toroidally, in a fixed order: up, down, left, right.
    pub fn neighbors(&self, row: usize, col: usize) -> [(usize, usize); 4] {
        let up = (row + self.nrows - 1) % self.nrows;
        let down = (row + 1) % self.nrows;
        let left = (col + self.ncols - 1) % self.ncols;
        let right = (col + 1) % self.ncols;
        [(up, col), (down, col), (row, left), (row, right)]
    }
}

/// A [`Grid2D`] plus one `T` per site — the shared container every
/// lattice model in this crate builds on.
pub struct LatticeState<T> {
    grid: Grid2D,
    sites: Vec<T>,
}

impl<T: Copy> LatticeState<T> {
    pub fn new(nrows: usize, ncols: usize, mut init: impl FnMut() -> T) -> Self {
        let grid = Grid2D::new(nrows, ncols);
        let sites = (0..grid.len()).map(|_| init()).collect();
        Self { grid, sites }
    }

    pub fn nrows(&self) -> usize {
        self.grid.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.grid.ncols()
    }

    pub fn sites(&self) -> &[T] {
        &self.sites
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.sites[self.grid.idx(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let i = self.grid.idx(row, col);
        self.sites[i] = value;
    }

    pub fn neighbor_values(&self, row: usize, col: usize) -> [T; 4] {
        self.grid.neighbors(row, col).map(|(r, c)| self.get(r, c))
    }

    pub fn fill(&mut self, mut init: impl FnMut() -> T) {
        for site in self.sites.iter_mut() {
            *site = init();
        }
    }
}
