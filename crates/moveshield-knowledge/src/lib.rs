//! MoveShield Engineering Knowledge Store.
//!
//! Exposes the computed static analysis state, call graphs, and metadata metrics
//! required by policy rules to detect security or architectural violations.

pub mod knowledge;
pub use knowledge::*;
