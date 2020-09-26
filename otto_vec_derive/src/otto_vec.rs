use syn::spanned::Spanned;

pub fn impl_otto_vec_macro(ast: syn::ItemFn) -> proc_macro::TokenStream {
    let function_name = parse_ident(&ast);
    let function_generics = parse_generics(&ast);
    let (function_arguments_name, function_arguments_name_vec, function_arguments_type) =
        parse_arguments(&ast);
    let function_return_type = parse_return(&ast);
    let function_body = parse_body(&ast);
    proc_macro::TokenStream::from(quote::quote! {
        #ast

        pub fn
        #function_name
        #function_generics
        (#(#function_arguments_name_vec : #function_arguments_type,)*)
        ->
        std::vec::Vec<#function_return_type> {
        #function_body
        }
    })
}

fn parse_ident(ast: &syn::ItemFn) -> syn::Ident {
    syn::Ident::new(&format!("{}_vec", &ast.sig.ident), ast.sig.ident.span())
}

// TODO :: In the future we might want to support generics.
#[allow(dead_code)]
fn parse_generics(ast: &syn::ItemFn) -> syn::ImplGenerics<'_> {
    let (impl_generics, _type_generics, _where_clause) = ast.sig.generics.split_for_impl();
    impl_generics
}

fn parse_arguments(
    ast: &syn::ItemFn,
) -> (
    Vec<&Box<syn::Pat>>,
    Vec<proc_macro2::Ident>,
    Vec<&Box<syn::Type>>,
) {
    (
        ast.sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Vectorization on member functions is not currently supported.")
                }
                syn::FnArg::Typed(pat_type) => &pat_type.pat,
            })
            .collect(),
        ast.sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Vectorization on member functions is not currently supported.")
                }
                syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                    syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
                        eprintln!("ident:{:#?}", ident);
                        syn::Ident::new(&format!("{}_vec", &ident), ident.span())
                    }
                    syn::Pat::Tuple(pat_tuple) => {
                        panic!("Unsupported tuple pattern\n{:#?}", pat_tuple)
                    }
                    v => panic!("unsupported pattern type\n{:#?}", v),
                },
            })
            .collect(),
        ast.sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Vectorization on member functions is not currently supported.")
                }
                syn::FnArg::Typed(pat_type) => &pat_type.ty,
            })
            .collect(),
    )
}

fn parse_return(ast: &syn::ItemFn) -> syn::Type {
    if let syn::ReturnType::Type(_, b) = &ast.sig.output {
        return *b.clone();
    } else {
        panic!("Vectorized function must have a return type.")
    }
}

fn parse_body(ast: &syn::ItemFn) -> proc_macro2::TokenStream {
    let statements = ast.block.stmts.iter().map(|statement| statement);
    quote::quote! {
        #(#statements)*
    }
}
