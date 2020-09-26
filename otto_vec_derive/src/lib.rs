extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

mod builder;
mod otto_vec;
mod util;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, Meta};

use crate::builder::impl_derive_macro;
use crate::otto_vec::impl_otto_vec_macro;

#[proc_macro_attribute]
pub fn otto_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("item:{:#?}", item);
    let ast = parse_macro_input!(item as ItemFn);
    impl_otto_vec_macro(ast)
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    impl_derive_macro(ast)
}
