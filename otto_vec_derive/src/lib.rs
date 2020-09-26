#[allow(dead_code)]
#[allow(unused)]
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

mod builder;
mod otto_vec;
mod util;

use crate::builder::impl_derive_macro;
use crate::otto_vec::impl_otto_vec_macro;

#[proc_macro_attribute]
pub fn otto_vec(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ItemFn);
    impl_otto_vec_macro(ast)
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);
    impl_derive_macro(ast)
}
