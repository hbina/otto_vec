extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

#[proc_macro_attribute]
pub fn otto_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let ast = parse_macro_input!(item as DeriveInput);
    TokenStream::from(quote! { pub fn ttt() {} })
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    impl_derive_macro(ast)
}

fn get_ident_if_field_is_option(field: &syn::Field) -> Option<syn::Type> {
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
                        Some(ty.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn impl_derive_macro(ast: DeriveInput) -> TokenStream {
    let struct_name = ast.ident;
    let builder_name = Ident::new(&format!("{}Builder", struct_name), Span::call_site());

    let struct_fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!("Only normal structs are supported.")
    };

    let optioned_fields = struct_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        get_ident_if_field_is_option(field)
            .map(|wrapped_type| {
                quote! {
                    #field_name : std::option::Option<#wrapped_type>
                }
            })
            .unwrap_or(quote! {
                #field_name : std::option::Option<#field_type>
            })
    });

    let none_fields = struct_fields.iter().map(|x| {
        let field_name = &x.ident;
        quote! {
            #field_name : None
        }
    });

    let methods = struct_fields.iter().map(|field| {
        field.attrs.iter().for_each(|attr|{
           if  let Ok(meta) = attr.parse_meta() {
           }
        });
        let field_name = &field.ident;
        let field_type = &field.ty;
        get_ident_if_field_is_option(field)
            .map(|wrapped_type| {
                quote! {
                    pub fn #field_name(&mut self, #field_name : #wrapped_type) -> &mut #builder_name {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
            })
            .unwrap_or(quote! {
                pub fn #field_name(&mut self, #field_name : #field_type) -> &mut #builder_name {
                    self.#field_name = Some(#field_name);
                    self
                }
            })
    });

    let option_check = struct_fields.iter().map(|field| {
        let field_name = &field.ident;
        get_ident_if_field_is_option(field)
            .map(|_| {
                quote! {
                    #field_name : self.#field_name.clone()
                }
            })
            .unwrap_or(quote! {
                #field_name : self.#field_name.clone().ok_or(concat!(stringify!(#field_name), " is not set."))?
            })
    });

    // Extracts fields
    let expanded = quote! {
        pub struct #builder_name {
            #(#optioned_fields,)*
        }

        impl #builder_name {
            #(#methods)*

            pub fn build(&self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                Ok(#struct_name {
                    #(#option_check,)*
                })
             }
        }

        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#none_fields,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
