//! Binary (active/inactive) site-state representation.

/// A site that is either Active or Inactive.
///
/// `#[repr(u8)]` so a `Vec<Binary>` has the exact same memory layout as
/// a `Vec<u8>` of the same values — sound to expose to wasm callers as a
/// raw `*const u8` buffer.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
    Inactive = 0,
    Active = 1,
}

impl Binary {
    pub fn is_active(self) -> bool {
        matches!(self, Binary::Active)
    }

    /// Uniformly random state (~50/50). `uniform` must yield values in
    /// `[0, 1)`.
    pub fn random(uniform: &mut impl FnMut() -> f64) -> Self {
        if uniform() < 0.5 { Binary::Active } else { Binary::Inactive }
    }
}
