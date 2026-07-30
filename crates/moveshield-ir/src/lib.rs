//! MoveShield Intermediate Representation (MoveIR) crate.
//!
//! Exposes structures and utilities for converting Core Data Model (CDM) constructs
//! into a semantic, normalized representation ready for static analysis and call graph construction.

pub mod ir;
pub use ir::*;
