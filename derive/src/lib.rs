#![doc(html_root_url = "https://docs.rs/sval_derive/1.0.0-alpha.5")]
#![recursion_limit = "128"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

mod attr;
mod bound;
mod value;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Value, attributes(sval_generic_api))]
pub fn derive_source(input: TokenStream) -> TokenStream {
    value::derive(parse_macro_input!(input as DeriveInput))
}
