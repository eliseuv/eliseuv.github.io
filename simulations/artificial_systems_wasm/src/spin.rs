//! Spin-1/2 state representation.

/// Spin-1/2: `+1` (up) or `-1` (down).
///
/// `#[repr(i8)]` so a `Vec<SpinHalf>` has the exact same memory layout as
/// a `Vec<i8>` of the same values — sound to expose to wasm callers as a
/// raw `*const i8` buffer.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinHalf {
    Down = -1,
    Up = 1,
}

impl SpinHalf {
    pub fn value(self) -> i32 {
        self as i32
    }

    pub fn flipped(self) -> Self {
        match self {
            SpinHalf::Up => SpinHalf::Down,
            SpinHalf::Down => SpinHalf::Up,
        }
    }

    /// Uniformly random spin. `uniform` must yield values in `[0, 1)`.
    pub fn random(uniform: &mut impl FnMut() -> f64) -> Self {
        if uniform() < 0.5 { SpinHalf::Up } else { SpinHalf::Down }
    }
}
