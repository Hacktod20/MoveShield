//! MoveShield Procedural Macros placeholder.
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    input
}
