//! MoveShield internal reusable utilities.
//!
//! Houses platform-wide helper routines that are free from specific domain or business logic.

/// Returns a greeting string from the utilities crate.
///
/// # Examples
///
/// ```
/// use moveshield_utils::greet_utils;
/// assert_eq!(greet_utils(), "utils");
/// ```
pub fn greet_utils() -> &'static str {
    "utils"
}
