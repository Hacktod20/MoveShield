//! MoveShield rules crate.
//!
//! Exposes static analysis rule implementations and the core `Rule` trait
//! used to execute analyses on the computed `KnowledgeStore`.

pub mod rules;
pub use rules::*;
