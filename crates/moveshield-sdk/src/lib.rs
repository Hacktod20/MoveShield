//! MoveShield Software Development Kit (SDK) crate.
//!
//! Exposes public APIs, extension traits, and plugin boundaries for extending MoveShield.

/// Returns the workspace package version.
///
/// # Examples
///
/// ```
/// use moveshield_sdk::version;
/// assert_eq!(version(), env!("CARGO_PKG_VERSION"));
/// ```
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
