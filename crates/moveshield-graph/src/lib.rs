//! MoveShield Call Graph construction and traversal library.
//!
//! Exposes the `CallGraph` structure, which builds dependency maps from
//! the `MoveIR` representation, offering cycle and recursion detection.

pub mod graph;
pub use graph::*;
