//! MoveShield Core domain models, shared errors, and common traits.
//!
//! Serves as the foundational crate establishing the Core Data Model (CDM) and
//! common domain interfaces used across the other modular analysis crates.

pub mod errors;
pub mod models;
pub mod traits;

pub use errors::*;
pub use models::*;
pub use traits::*;
