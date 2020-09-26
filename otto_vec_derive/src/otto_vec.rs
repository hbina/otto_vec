use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

pub fn impl_otto_vec_macro(ast: syn::ItemFn) -> TokenStream {
    TokenStream::from(quote! {
        pub fn test() {}
    })
}
