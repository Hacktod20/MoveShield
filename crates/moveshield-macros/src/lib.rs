//! MoveShield Procedural Macros library.
//!
//! Provides compile-time code generation and derive helper attributes
//! utilized across the MoveShield core framework.

extern crate proc_macro;
use proc_macro::TokenStream;

/// Custom derive procedural macro placeholder for workspace-wide traits.
#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    input
}
