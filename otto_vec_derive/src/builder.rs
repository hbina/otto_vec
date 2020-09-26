use quote::quote;

use crate::util::get_ident_if_field_is_option;

pub fn impl_derive_macro(ast: syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = ast.ident;
    let builder_name = syn::Ident::new(
        &format!("{}Builder", struct_name),
        proc_macro2::Span::call_site(),
    );

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

    proc_macro::TokenStream::from(expanded)
}
