pub use derivation::*;

mod derivation;

/// An Entry inside the synchronized Store
pub struct Entry {
    drv: Derivation,
}
