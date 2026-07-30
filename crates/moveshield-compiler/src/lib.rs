//! MoveShield compiler adapter and project discovery library.
//!
//! Provides the stable interfaces to integrate Move/Sui compiler compilation stages with MoveShield's Core Data Model (CDM).

pub mod adapter;
pub mod discovery;

pub use adapter::*;
pub use discovery::*;
