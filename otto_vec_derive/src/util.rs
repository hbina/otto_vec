use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

pub fn get_ident_if_field_is_option(field: &syn::Field) -> Option<syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = field.ty
    {
        if let Some(path_segment) = segments.first() {
            if path_segment.ident == "Option" {
                // <--- HOW TO AVOID THIS HACK?
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = path_segment.arguments
                {
                    if let Some(syn::GenericArgument::Type(ty)) = args.first() {
                        return Some(ty.clone());
                    }
                }
            }
        }
    }
    return None;
}
