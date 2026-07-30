//! Common traits used across the MoveShield crates.

/// Any type that can report its source location.
pub trait HasLocation {
    /// Gets the location of this item.
    fn location(&self) -> &super::models::Location;
}
